"use client";

import { FormEvent, useEffect, useMemo, useState } from "react";

type HardwareSnapshot = {
  timestamp: string;
  cpu: {
    usagePercent: number;
    temperatureC: number | null;
    cores: number;
    speedGHz: number | null;
    model: string;
  };
  memory: {
    usagePercent: number;
    usedGB: number;
    totalGB: number;
  };
  gpu: {
    usagePercent: number | null;
    temperatureC: number | null;
    model: string;
    vramGB: number | null;
  };
  network: {
    status: "online" | "offline";
    interfaceName: string;
    downloadMBps: number;
    uploadMBps: number;
  };
  storage: Array<{
    name: string;
    mount: string;
    type: string;
    usedGB: number;
    totalGB: number;
    usagePercent: number;
  }>;
};

type FavoriteSite = {
  id: string;
  name: string;
  url: string;
};

type TodoItem = {
  id: string;
  text: string;
  done: boolean;
};

const STORAGE_KEYS = {
  sites: "qwen-dashboard-sites",
  todos: "qwen-dashboard-todos",
};

const DEFAULT_SITES: FavoriteSite[] = [
  { id: "github", name: "GitHub", url: "https://github.com" },
  { id: "youtube", name: "YouTube", url: "https://youtube.com" },
  { id: "reddit", name: "Reddit", url: "https://reddit.com" },
  { id: "stackoverflow", name: "Stack Overflow", url: "https://stackoverflow.com" },
];

function clampPercent(value: number | null | undefined): number {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return 0;
  }
  return Math.min(100, Math.max(0, value));
}

function formatPercent(value: number | null | undefined): string {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return "N/A";
  }
  return `${value.toFixed(1)}%`;
}

function formatTemp(value: number | null): string {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return "N/A";
  }
  return `${value.toFixed(1)}°C`;
}

function formatGb(value: number | null | undefined): string {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return "N/A";
  }
  return `${value.toFixed(1)} GB`;
}

function normalizeUrl(url: string): string {
  return /^https?:\/\//i.test(url) ? url : `https://${url}`;
}

function loadInitialSites(): FavoriteSite[] {
  if (typeof window === "undefined") {
    return DEFAULT_SITES;
  }

  try {
    const storedSites = window.localStorage.getItem(STORAGE_KEYS.sites);
    if (!storedSites) {
      return DEFAULT_SITES;
    }
    const parsed = JSON.parse(storedSites) as FavoriteSite[];
    return Array.isArray(parsed) && parsed.length > 0 ? parsed : DEFAULT_SITES;
  } catch {
    return DEFAULT_SITES;
  }
}

function loadInitialTodos(): TodoItem[] {
  if (typeof window === "undefined") {
    return [];
  }

  try {
    const storedTodos = window.localStorage.getItem(STORAGE_KEYS.todos);
    if (!storedTodos) {
      return [];
    }
    const parsed = JSON.parse(storedTodos) as TodoItem[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

export default function Home() {
  const [now, setNow] = useState(new Date());
  const [hardware, setHardware] = useState<HardwareSnapshot | null>(null);
  const [fetchState, setFetchState] = useState<"loading" | "ok" | "error">(
    "loading",
  );
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  const [sites, setSites] = useState<FavoriteSite[]>(loadInitialSites);
  const [siteName, setSiteName] = useState("");
  const [siteUrl, setSiteUrl] = useState("");

  const [todos, setTodos] = useState<TodoItem[]>(loadInitialTodos);
  const [todoText, setTodoText] = useState("");

  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }
    window.localStorage.setItem(STORAGE_KEYS.sites, JSON.stringify(sites));
  }, [sites]);

  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }
    window.localStorage.setItem(STORAGE_KEYS.todos, JSON.stringify(todos));
  }, [todos]);

  useEffect(() => {
    const clockInterval = window.setInterval(() => {
      setNow(new Date());
    }, 1000);
    return () => window.clearInterval(clockInterval);
  }, []);

  useEffect(() => {
    let isActive = true;

    const fetchHardware = async () => {
      try {
        const response = await fetch("/api/hardware", { cache: "no-store" });
        if (!response.ok) {
          throw new Error(`Hardware API returned ${response.status}`);
        }
        const payload = (await response.json()) as HardwareSnapshot;
        if (!isActive) {
          return;
        }
        setHardware(payload);
        setFetchState("ok");
        setLastUpdated(new Date(payload.timestamp));
      } catch {
        if (!isActive) {
          return;
        }
        setFetchState("error");
      }
    };

    void fetchHardware();
    const interval = window.setInterval(() => {
      void fetchHardware();
    }, 5000);

    return () => {
      isActive = false;
      window.clearInterval(interval);
    };
  }, []);

  const greeting = useMemo(() => {
    const hour = now.getHours();
    if (hour < 12) {
      return "Morning";
    }
    if (hour < 18) {
      return "Afternoon";
    }
    return "Evening";
  }, [now]);

  const remainingTodos = useMemo(
    () => todos.filter((todo) => !todo.done).length,
    [todos],
  );

  const handleAddSite = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedName = siteName.trim();
    const trimmedUrl = siteUrl.trim();
    if (!trimmedName || !trimmedUrl) {
      return;
    }

    setSites((current) => [
      {
        id: `${Date.now()}-${trimmedName.toLowerCase()}`,
        name: trimmedName,
        url: normalizeUrl(trimmedUrl),
      },
      ...current,
    ]);
    setSiteName("");
    setSiteUrl("");
  };

  const handleAddTodo = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmed = todoText.trim();
    if (!trimmed) {
      return;
    }
    setTodos((current) => [
      { id: `${Date.now()}-${trimmed.length}`, text: trimmed, done: false },
      ...current,
    ]);
    setTodoText("");
  };

  const statusColor =
    fetchState === "ok"
      ? "bg-emerald-500/20 text-emerald-300"
      : fetchState === "error"
        ? "bg-red-500/20 text-red-200"
        : "bg-slate-700/60 text-slate-200";

  return (
    <div className="min-h-screen text-slate-100">
      <header className="border-b border-white/10 bg-slate-950/70 backdrop-blur-xl">
        <div className="mx-auto flex max-w-6xl flex-wrap items-end justify-between gap-4 px-4 py-6">
          <div>
            <p className="text-sm uppercase tracking-[0.2em] text-slate-400">
              Good {greeting}
            </p>
            <h1 className="text-4xl font-semibold md:text-5xl">
              {now.toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
                second: "2-digit",
              })}
            </h1>
            <p className="mt-1 text-sm text-slate-300">
              {now.toLocaleDateString([], {
                weekday: "long",
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </p>
          </div>
          <div className="space-y-2 text-right">
            <span className={`inline-flex rounded-full px-3 py-1 text-xs ${statusColor}`}>
              Hardware API:{" "}
              {fetchState === "ok"
                ? "Live"
                : fetchState === "error"
                  ? "Error"
                  : "Loading"}
            </span>
            <p className="text-xs text-slate-400">
              Last update:{" "}
              {lastUpdated
                ? lastUpdated.toLocaleTimeString([], {
                    hour: "2-digit",
                    minute: "2-digit",
                    second: "2-digit",
                  })
                : "--:--:--"}
            </p>
          </div>
        </div>
      </header>

      <main className="mx-auto flex max-w-6xl flex-col gap-6 px-4 py-6">
        <section className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          <article className="rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-sm text-slate-400">CPU</h2>
              <span className="text-xs text-slate-300">
                {formatTemp(hardware?.cpu.temperatureC ?? null)}
              </span>
            </div>
            <p className="text-2xl font-semibold">
              {formatPercent(hardware?.cpu.usagePercent)}
            </p>
            <div className="mt-3 h-2 overflow-hidden rounded-full bg-slate-700">
              <div
                className="h-full bg-gradient-to-r from-blue-500 to-cyan-400"
                style={{ width: `${clampPercent(hardware?.cpu.usagePercent)}%` }}
              />
            </div>
            <p className="mt-2 text-xs text-slate-400">
              {hardware?.cpu.cores ?? "--"} cores •{" "}
              {hardware?.cpu.speedGHz ? `${hardware.cpu.speedGHz.toFixed(2)} GHz` : "N/A"}
            </p>
          </article>

          <article className="rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-sm text-slate-400">Memory</h2>
              <span className="text-xs text-slate-300">
                {formatGb(hardware?.memory.usedGB)} / {formatGb(hardware?.memory.totalGB)}
              </span>
            </div>
            <p className="text-2xl font-semibold">
              {formatPercent(hardware?.memory.usagePercent)}
            </p>
            <div className="mt-3 h-2 overflow-hidden rounded-full bg-slate-700">
              <div
                className="h-full bg-gradient-to-r from-violet-500 to-fuchsia-400"
                style={{ width: `${clampPercent(hardware?.memory.usagePercent)}%` }}
              />
            </div>
          </article>

          <article className="rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-sm text-slate-400">GPU</h2>
              <span className="text-xs text-slate-300">
                {formatTemp(hardware?.gpu.temperatureC ?? null)}
              </span>
            </div>
            <p className="text-2xl font-semibold">
              {formatPercent(hardware?.gpu.usagePercent)}
            </p>
            <div className="mt-3 h-2 overflow-hidden rounded-full bg-slate-700">
              <div
                className="h-full bg-gradient-to-r from-emerald-500 to-teal-400"
                style={{ width: `${clampPercent(hardware?.gpu.usagePercent)}%` }}
              />
            </div>
            <p className="mt-2 truncate text-xs text-slate-400">
              {hardware?.gpu.model ?? "No GPU data"} •{" "}
              {hardware?.gpu.vramGB ? `${hardware.gpu.vramGB.toFixed(1)} GB VRAM` : "VRAM N/A"}
            </p>
          </article>

          <article className="rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-sm text-slate-400">Network</h2>
              <span
                className={`text-xs ${
                  hardware?.network.status === "online"
                    ? "text-emerald-300"
                    : "text-red-200"
                }`}
              >
                {hardware?.network.status ?? "offline"}
              </span>
            </div>
            <p className="text-sm text-slate-300">{hardware?.network.interfaceName ?? "N/A"}</p>
            <div className="mt-3 grid grid-cols-2 gap-2 text-sm">
              <div className="rounded-lg bg-slate-800/80 p-2">
                <p className="text-xs text-slate-400">Download</p>
                <p className="font-semibold text-emerald-300">
                  {hardware ? `${hardware.network.downloadMBps.toFixed(2)} MB/s` : "N/A"}
                </p>
              </div>
              <div className="rounded-lg bg-slate-800/80 p-2">
                <p className="text-xs text-slate-400">Upload</p>
                <p className="font-semibold text-cyan-300">
                  {hardware ? `${hardware.network.uploadMBps.toFixed(2)} MB/s` : "N/A"}
                </p>
              </div>
            </div>
          </article>
        </section>

        <section className="rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-lg font-semibold">Storage</h2>
            <span className="text-xs text-slate-400">
              {hardware?.storage.length ?? 0} drive(s)
            </span>
          </div>
          <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
            {hardware?.storage.map((drive) => (
              <article
                key={`${drive.name}-${drive.mount}`}
                className="rounded-xl border border-white/10 bg-slate-800/80 p-3"
              >
                <div className="mb-2 flex items-center justify-between">
                  <p className="text-sm font-medium">{drive.name}</p>
                  <p className="text-xs text-slate-400">{drive.mount}</p>
                </div>
                <div className="h-2 overflow-hidden rounded-full bg-slate-700">
                  <div
                    className="h-full bg-gradient-to-r from-amber-500 to-orange-400"
                    style={{ width: `${clampPercent(drive.usagePercent)}%` }}
                  />
                </div>
                <p className="mt-2 text-xs text-slate-400">
                  {drive.usedGB.toFixed(1)} / {drive.totalGB.toFixed(1)} GB •{" "}
                  {drive.usagePercent.toFixed(1)}%
                </p>
              </article>
            ))}
            {!hardware?.storage.length && (
              <p className="text-sm text-slate-400">No storage metrics available.</p>
            )}
          </div>
        </section>

        <section className="grid gap-6 lg:grid-cols-5">
          <article className="lg:col-span-3 rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <h2 className="mb-3 text-lg font-semibold">Favorite Websites</h2>
            <form className="mb-4 grid gap-2 sm:grid-cols-3" onSubmit={handleAddSite}>
              <input
                className="rounded-lg border border-white/10 bg-slate-800/90 px-3 py-2 text-sm outline-none transition focus:border-cyan-400"
                placeholder="Site name"
                value={siteName}
                onChange={(event) => setSiteName(event.target.value)}
              />
              <input
                className="rounded-lg border border-white/10 bg-slate-800/90 px-3 py-2 text-sm outline-none transition focus:border-cyan-400"
                placeholder="https://example.com"
                value={siteUrl}
                onChange={(event) => setSiteUrl(event.target.value)}
              />
              <button
                type="submit"
                className="rounded-lg bg-cyan-500 px-3 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-400"
              >
                Add
              </button>
            </form>
            <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
              {sites.map((site) => (
                <div
                  key={site.id}
                  className="flex items-center justify-between rounded-lg border border-white/10 bg-slate-800/70 px-3 py-2"
                >
                  <a
                    href={site.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="truncate text-sm text-cyan-200 hover:text-cyan-100"
                  >
                    {site.name}
                  </a>
                  <button
                    type="button"
                    onClick={() =>
                      setSites((current) =>
                        current.filter((currentSite) => currentSite.id !== site.id),
                      )
                    }
                    className="rounded px-2 py-1 text-xs text-slate-400 transition hover:bg-red-500/20 hover:text-red-200"
                  >
                    Remove
                  </button>
                </div>
              ))}
            </div>
          </article>

          <article className="lg:col-span-2 rounded-2xl border border-white/10 bg-slate-900/70 p-4 backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-lg font-semibold">Todo</h2>
              <span className="text-xs text-slate-400">{remainingTodos} remaining</span>
            </div>
            <form className="mb-3 flex gap-2" onSubmit={handleAddTodo}>
              <input
                className="min-w-0 flex-1 rounded-lg border border-white/10 bg-slate-800/90 px-3 py-2 text-sm outline-none transition focus:border-emerald-400"
                placeholder="Add task..."
                value={todoText}
                onChange={(event) => setTodoText(event.target.value)}
              />
              <button
                type="submit"
                className="rounded-lg bg-emerald-500 px-3 py-2 text-sm font-medium text-slate-950 transition hover:bg-emerald-400"
              >
                Add
              </button>
            </form>
            <div className="space-y-2">
              {todos.map((todo) => (
                <div
                  key={todo.id}
                  className="flex items-center gap-2 rounded-lg border border-white/10 bg-slate-800/70 px-2 py-2"
                >
                  <input
                    type="checkbox"
                    checked={todo.done}
                    onChange={() =>
                      setTodos((current) =>
                        current.map((currentTodo) =>
                          currentTodo.id === todo.id
                            ? { ...currentTodo, done: !currentTodo.done }
                            : currentTodo,
                        ),
                      )
                    }
                  />
                  <span
                    className={`min-w-0 flex-1 truncate text-sm ${
                      todo.done ? "text-slate-500 line-through" : "text-slate-200"
                    }`}
                  >
                    {todo.text}
                  </span>
                  <button
                    type="button"
                    onClick={() =>
                      setTodos((current) =>
                        current.filter((currentTodo) => currentTodo.id !== todo.id),
                      )
                    }
                    className="rounded px-2 py-1 text-xs text-slate-400 transition hover:bg-red-500/20 hover:text-red-200"
                  >
                    Delete
                  </button>
                </div>
              ))}
              {todos.length === 0 && (
                <p className="text-sm text-slate-400">No tasks yet.</p>
              )}
            </div>
            {todos.some((todo) => todo.done) && (
              <button
                type="button"
                onClick={() => setTodos((current) => current.filter((todo) => !todo.done))}
                className="mt-3 text-xs text-slate-400 transition hover:text-red-200"
              >
                Clear completed
              </button>
            )}
          </article>
        </section>
      </main>
    </div>
  );
}
