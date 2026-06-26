# Sleek Dashboard

A lightweight, responsive, and powerful web dashboard designed to override your default **Chrome New Tab page** on both macOS and Windows.

This project uses a decoupled hybrid architecture:
1. **Frontend (Chrome Extension):** Built with **Svelte + Vite + Tailwind CSS v4**. It loads instantly from your local drive and interacts with native browser APIs (like `chrome.bookmarks`) in Chrome on macOS and Windows.
2. **Backend (Daemon):** A lightweight background service written in **Rust (Axum + sysinfo)**. It queries system stats (CPU, RAM, Disks, Network) and serves them locally with a memory footprint of **under 15MB RAM**.

---

## Project Structure

```text
dashboard/
├── backend/            # Rust Axum daemon (system metrics api)
│   ├── src/main.rs
│   └── Cargo.toml
├── frontend/           # Svelte + Vite Chrome Extension (the dashboard UI)
│   ├── src/            # App code & components (Clock, Stats, Bookmarks, Checklist)
│   ├── public/         # Static assets & manifest.json
│   └── package.json
├── todos.md            # Feature roadmap & deferred items (e.g. repotasks integration)
└── README.md           # This file
```

---

## Getting Started

### Prerequisites
* [Node.js](https://nodejs.org/) (v18+ recommended)
* [Rust & Cargo](https://www.rust-lang.org/tools/install) (to compile the daemon)
* Google Chrome (or any Chromium browser) on macOS or Windows

---

## Build Everything

From the repository root, run:

```bash
npm run build
```

That will install the frontend dependencies if needed, build the Svelte app, and compile the Rust backend in release mode.

---

## Step 1: Run the Rust Backend Daemon

The backend daemon must run locally on your system to fetch hardware stats. It listens on `http://127.0.0.1:9999`.

1. Open a terminal and navigate to the backend folder:
   ```bash
   cd backend
   ```
2. Build and run the daemon in release mode (for maximum performance and minimum size):
   ```bash
   cargo run --release
   ```
3. You will see: `Dashboard daemon starting on http://127.0.0.1:9999`. You can minimize this terminal and let it run in the background.

---

## Step 2: Build the Svelte Frontend

To load the dashboard into Chrome, the Svelte code must be compiled into static web files.

1. Open a new terminal in the repository root.
2. Build the project with `npm run build` if you have not already done so.
3. This generates a `frontend/dist/` directory containing the compiled HTML, CSS, JavaScript, and `manifest.json`.

---

## Step 3: Load the Extension in Google Chrome

1. Open Google Chrome.
2. In the URL bar, go to:
   ```text
   chrome://extensions
   ```
3. In the top-right corner, toggle **Developer mode** to **ON**.
4. In the top-left corner, click **Load unpacked**.
5. Select the `frontend/dist` folder.
6. Open a new tab in Chrome (`Ctrl + T`). 
7. Chrome will show a popup saying *"Is this the new tab page you expected?"*. Click **Keep it** to set the dashboard as your default home page.

---

## Development Preview

If you want to edit Svelte components and see changes update in real-time, you can run the Vite dev server:

```bash
cd frontend
npm run dev
```

> [!NOTE]
> When running the dev server (`http://localhost:5173`), the `chrome.bookmarks` API is not available because it is running as a standard web page. The dashboard will automatically fall back to **mock bookmarks** for preview. To test real bookmarks, run `npm run build` and refresh the extension in Chrome.

---

## Troubleshooting

### Q: Why do my system stats say "Offline"?
**A:** Make sure you ran `cargo run --release` in the `backend/` folder and the console is still open. If the backend is closed, the dashboard continues to work (bookmarks, clock, checklist, search) but stats widgets will show a clean "Local Daemon: Offline" indicator.

### Q: How do I update the extension after modifying code?
**A:** 
1. Modify your code in `frontend/src/`.
2. Run `npm run build` from the repository root or `cd frontend && npm run build`.
3. Open `chrome://extensions` in Chrome and click the **Refresh (circular arrow)** icon on the **Sleek Dashboard** card.

---

## Future Roadmap
To see planned features, including cloning the Tauri-based `repotasks` UI directly onto the dashboard, checkout the [todos.md](todos.md) roadmap file in the root folder.
