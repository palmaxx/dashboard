# Dashboard Future Roadmap & Deferred Features

This file tracks features and integrations that are deferred for future iterations to maintain focus on the UI quality, system efficiency, and core Chrome Extension functionality during the initial release.

---

## 1. `repotasks` Deep Integration (Tauri App Clone)
* **Goal:** Create a dashboard widget that mirrors your Tauri `repotasks` app, enabling you to manage repositories and developer tasks directly from your Chrome New Tab page.
* **Backend Tasks (Rust Daemon):**
  * Locate and read the `repotasks` data directory (e.g., SQLite DB or JSON state in `C:\Users\<user>\AppData\Roaming\repotasks`).
  * Expose REST or WebSocket endpoints (e.g., `/api/repotasks/tasks`, `/api/repotasks/repos`) to query active tasks and registered git repositories.
  * Provide action routes (e.g., `/api/repotasks/start-task`, `/api/repotasks/complete-task`) to update the database.
* **Frontend Tasks (Svelte):**
  * Create a detailed `Repotasks.svelte` widget.
  * Display a list of repository cards with their active task counts, current branch names, and uncommitted changes.
  * Render an interactive list of tasks grouped by status (Todo, In Progress, Done) with quick-toggle buttons.

---

## 2. Real `yt-dlp` Downloader Integration
* **Goal:** Make the video/audio downloader widget fully functional using your local system's `yt-dlp` installation.
* **Backend Tasks (Rust Daemon):**
  * Add a `/api/download` endpoint that receives video URLs, format specifications, and target directories.
  * Spawn `yt-dlp` as a child process.
  * Establish a WebSocket connection to stream download progress stdout (e.g., speed, ETA, percentage) back to the Chrome Extension UI in real-time.
* **Frontend Tasks (Svelte):**
  * Connect the existing downloader inputs to the WebSocket progress bar.
  * Manage a download history queue locally.

---

## 3. Advanced Hardware & Process Diagnostics
* **Goal:** Provide detailed system analytics beyond aggregate percentages.
* **Backend Tasks (Rust Daemon):**
  * Read the top 5 CPU and Memory consuming processes dynamically from the OS.
  * Retrieve network adapter properties (IP addresses, adapter models).
* **Frontend Tasks (Svelte):**
  * Add a hover-card or "view details" modal to the CPU and RAM widgets showing the list of active heavy processes.

---

## 4. Custom Local Wallpapers
* **Goal:** Set custom background images directly from a local folder on your computer.
* **Backend Tasks (Rust Daemon):**
  * Serve images from a specific directory (e.g. `C:\Users\<user>\Pictures\Dashboard`) through a static file router in Axum.
* **Frontend Tasks (Svelte):**
  * Retrieve list of local wallpapers from the backend and populate them in the wallpaper selection grid.
