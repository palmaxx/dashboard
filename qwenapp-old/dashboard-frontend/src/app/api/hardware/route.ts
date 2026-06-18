import os from "node:os";
import { NextResponse } from "next/server";
import si from "systeminformation";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

function round(value: number, decimals = 1): number {
  const factor = 10 ** decimals;
  return Math.round(value * factor) / factor;
}

function clampPercent(value: number): number {
  return Math.min(100, Math.max(0, round(value, 1)));
}

function bytesToGB(bytes: number): number {
  return round(bytes / 1024 / 1024 / 1024, 1);
}

function bytesToMB(bytes: number): number {
  return round(bytes / 1024 / 1024, 2);
}

function getNumeric(value: unknown): number | null {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return null;
  }
  return value;
}

export async function GET() {
  try {
    const [cpuInfo, load, cpuTemp, memory, graphics, filesystems, networkStats] =
      await Promise.all([
        si.cpu(),
        si.currentLoad(),
        si.cpuTemperature(),
        si.mem(),
        si.graphics(),
        si.fsSize(),
        si.networkStats(),
      ]);

    const cpuSpeedRaw = Number.parseFloat(String(cpuInfo.speed));
    const cpuSpeedGHz =
      Number.isFinite(cpuSpeedRaw) && cpuSpeedRaw > 0 ? round(cpuSpeedRaw, 2) : null;

    const cpuTemperatureC =
      typeof cpuTemp.main === "number" && cpuTemp.main > 0
        ? round(cpuTemp.main, 1)
        : null;

    const memoryUsedBytes = memory.active > 0 ? memory.active : memory.used;
    const memoryUsagePercent =
      memory.total > 0 ? clampPercent((memoryUsedBytes / memory.total) * 100) : 0;

    const primaryGpu = graphics.controllers[0];
    const gpuRecord = (primaryGpu ?? {}) as Record<string, unknown>;
    const gpuUsageRaw = getNumeric(gpuRecord.utilizationGpu);
    const gpuTempRaw = getNumeric(gpuRecord.temperatureGpu);
    const gpuVramRaw = getNumeric(gpuRecord.vram);

    const preferredNetwork =
      networkStats.find((entry) => !entry.iface.toLowerCase().includes("loopback")) ??
      networkStats[0];

    const storage = filesystems
      .filter((disk) => disk.size > 0)
      .slice(0, 8)
      .map((disk) => ({
        name: disk.fs || disk.mount || "Disk",
        mount: disk.mount || "N/A",
        type: disk.type || "disk",
        usedGB: bytesToGB(disk.used),
        totalGB: bytesToGB(disk.size),
        usagePercent: clampPercent(
          typeof disk.use === "number" && Number.isFinite(disk.use)
            ? disk.use
            : (disk.used / disk.size) * 100,
        ),
      }));

    return NextResponse.json(
      {
        timestamp: new Date().toISOString(),
        cpu: {
          usagePercent: clampPercent(load.currentLoad),
          temperatureC: cpuTemperatureC,
          cores: os.cpus().length,
          speedGHz: cpuSpeedGHz,
          model: cpuInfo.brand || "Unknown CPU",
        },
        memory: {
          usagePercent: memoryUsagePercent,
          usedGB: bytesToGB(memoryUsedBytes),
          totalGB: bytesToGB(memory.total),
        },
        gpu: {
          usagePercent: gpuUsageRaw !== null ? clampPercent(gpuUsageRaw) : null,
          temperatureC: gpuTempRaw !== null && gpuTempRaw > 0 ? round(gpuTempRaw, 1) : null,
          model: primaryGpu?.model || "Not detected",
          vramGB: gpuVramRaw !== null ? round(gpuVramRaw / 1024, 1) : null,
        },
        network: {
          status: preferredNetwork?.operstate === "up" ? "online" : "offline",
          interfaceName: preferredNetwork?.iface || "N/A",
          downloadMBps: preferredNetwork ? bytesToMB(Math.max(preferredNetwork.rx_sec, 0)) : 0,
          uploadMBps: preferredNetwork ? bytesToMB(Math.max(preferredNetwork.tx_sec, 0)) : 0,
        },
        storage,
      },
      {
        headers: {
          "Cache-Control": "no-store, max-age=0",
        },
      },
    );
  } catch (error) {
    return NextResponse.json(
      {
        error: "Unable to collect hardware stats",
        details: error instanceof Error ? error.message : "Unknown error",
      },
      { status: 500 },
    );
  }
}
