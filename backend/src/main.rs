use std::sync::Arc;
use std::time::{Duration, Instant};
use axum::{
    extract::{State, Path},
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router
};
use axum::http::{HeaderMap, HeaderValue, header, StatusCode};
use serde::{Deserialize, Serialize};
use sysinfo::{
    Components, Disks, Networks, System
};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use std::fs;
use std::path::PathBuf;

mod models;
mod notes;

#[derive(Serialize, Clone, Debug)]
struct CpuInfo {
    #[serde(rename = "usagePercent")]
    usage_percent: f32,
    #[serde(rename = "temperatureC")]
    temperature_c: Option<f32>,
    cores: usize,
    #[serde(rename = "speedGHz")]
    speed_ghz: Option<f32>,
    model: String,
}

#[derive(Serialize, Clone, Debug)]
struct MemoryInfo {
    #[serde(rename = "usagePercent")]
    usage_percent: f32,
    #[serde(rename = "usedGB")]
    used_gb: f32,
    #[serde(rename = "totalGB")]
    total_gb: f32,
}

#[derive(Serialize, Clone, Debug)]
struct GpuInfo {
    #[serde(rename = "usagePercent")]
    usage_percent: Option<f32>,
    #[serde(rename = "temperatureC")]
    temperature_c: Option<f32>,
    model: String,
    #[serde(rename = "vramGB")]
    vram_gb: Option<f32>,
}

#[derive(Serialize, Clone, Debug)]
struct NetworkInfo {
    status: String,
    #[serde(rename = "interfaceName")]
    interface_name: String,
    #[serde(rename = "downloadMBps")]
    download_mbps: f32,
    #[serde(rename = "uploadMBps")]
    upload_mbps: f32,
}

#[derive(Serialize, Clone, Debug)]
struct StorageInfo {
    name: String,
    mount: String,
    #[serde(rename = "type")]
    disk_type: String,
    #[serde(rename = "usedGB")]
    used_gb: f32,
    #[serde(rename = "totalGB")]
    total_gb: f32,
    #[serde(rename = "usagePercent")]
    usage_percent: f32,
}

#[derive(Serialize, Clone, Debug)]
struct HardwareSnapshot {
    timestamp: String,
    cpu: CpuInfo,
    memory: MemoryInfo,
    gpu: GpuInfo,
    network: NetworkInfo,
    storage: Vec<StorageInfo>,
}

type SharedState = Arc<RwLock<Option<HardwareSnapshot>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(RwLock::new(None));

    // Spawn a background task to refresh system info periodically
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut sys = System::new_all();
        let mut disks = Disks::new_with_refreshed_list();
        let mut networks = Networks::new_with_refreshed_list();
        let mut components = Components::new_with_refreshed_list();

        let mut prev_received = 0u64;
        let mut prev_transmitted = 0u64;
        let mut prev_time = Instant::now();

        // Initial refresh
        sys.refresh_all();
        for (_, data) in &networks {
            prev_received += data.received();
            prev_transmitted += data.transmitted();
        }

        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            // Refresh system info
            sys.refresh_cpu();
            sys.refresh_memory();
            disks.refresh_list();
            networks.refresh_list();
            components.refresh_list();

            let now = Instant::now();
            let elapsed = now.duration_since(prev_time).as_secs_f32();
            prev_time = now;

            // CPU
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let cpu_model = sys.global_cpu_info().brand().to_string();
            let cpu_cores = sys.cpus().len();
            let cpu_freq = sys.cpus().first().map(|c| c.frequency() as f32 / 1000.0); // MHz to GHz

            // CPU Temp
            let mut cpu_temp = None;
            for comp in &components {
                let label = comp.label().to_lowercase();
                if label.contains("cpu") || label.contains("core") {
                    cpu_temp = Some(comp.temperature());
                    break;
                }
            }

            // Memory
            let total_mem = sys.total_memory();
            let used_mem = sys.used_memory();
            let mem_usage_percent = if total_mem > 0 {
                (used_mem as f32 / total_mem as f32) * 100.0
            } else {
                0.0
            };
            let total_gb = total_mem as f32 / 1024.0 / 1024.0 / 1024.0;
            let used_gb = used_mem as f32 / 1024.0 / 1024.0 / 1024.0;

            // Network Metrics
            let mut total_rx = 0u64;
            let mut total_tx = 0u64;
            let mut active_iface = "N/A".to_string();
            let mut max_rx = 0u64;

            for (name, data) in &networks {
                let rx = data.received();
                let tx = data.transmitted();
                total_rx += rx;
                total_tx += tx;

                // Pick the interface with the most activity as active
                if rx > max_rx {
                    max_rx = rx;
                    active_iface = name.clone();
                }
            }

            let rx_bytes = total_rx.saturating_sub(prev_received);
            let tx_bytes = total_tx.saturating_sub(prev_transmitted);

            prev_received = total_rx;
            prev_transmitted = total_tx;

            let download_mbps = if elapsed > 0.0 {
                (rx_bytes as f32 / 1024.0 / 1024.0) / elapsed
            } else {
                0.0
            };

            let upload_mbps = if elapsed > 0.0 {
                (tx_bytes as f32 / 1024.0 / 1024.0) / elapsed
            } else {
                0.0
            };

            let network_status = if total_rx > 0 { "online" } else { "offline" }.to_string();

            // Storage
            let mut storage = Vec::new();
            for disk in &disks {
                let name = disk.name().to_string_lossy().to_string();
                let mount = disk.mount_point().to_string_lossy().to_string();
                let total_bytes = disk.total_space();
                let available_bytes = disk.available_space();
                let used_bytes = total_bytes.saturating_sub(available_bytes);

                let total_gb = total_bytes as f32 / 1024.0 / 1024.0 / 1024.0;
                let used_gb = used_bytes as f32 / 1024.0 / 1024.0 / 1024.0;
                let usage_percent = if total_bytes > 0 {
                    (used_bytes as f32 / total_bytes as f32) * 100.0
                } else {
                    0.0
                };

                let disk_type = match disk.kind() {
                    sysinfo::DiskKind::SSD => "SSD",
                    sysinfo::DiskKind::HDD => "HDD",
                    _ => "Disk",
                }.to_string();

                storage.push(StorageInfo {
                    name: if name.is_empty() { mount.clone() } else { name },
                    mount,
                    disk_type,
                    used_gb,
                    total_gb,
                    usage_percent,
                });
            }

            // Create Snapshot
            let snapshot = HardwareSnapshot {
                timestamp: chrono::Utc::now().to_rfc3339(),
                cpu: CpuInfo {
                    usage_percent: cpu_usage,
                    temperature_c: cpu_temp,
                    cores: cpu_cores,
                    speed_ghz: cpu_freq,
                    model: cpu_model,
                },
                memory: MemoryInfo {
                    usage_percent: mem_usage_percent,
                    used_gb,
                    total_gb,
                },
                gpu: GpuInfo {
                    usage_percent: None,
                    temperature_c: None,
                    model: "N/A".to_string(),
                    vram_gb: None,
                },
                network: NetworkInfo {
                    status: network_status,
                    interface_name: active_iface,
                    download_mbps,
                    upload_mbps,
                },
                storage,
            };

            // Write state
            let mut lock = state_clone.write().await;
            *lock = Some(snapshot);
        }
    });

    // Build the Axum router
    let app = Router::new()
        .route("/api/sysinfo", get(sysinfo_handler))
        .route("/api/projects", get(projects_handler))
        .route("/api/projects/:project_id/add", post(add_entry_handler))
        .route("/api/projects/:project_id/toggle/:line", post(toggle_entry_handler))
        .route("/api/projects/:project_id/update/:line", put(update_entry_handler))
        .route("/api/projects/:project_id/delete/:line", delete(delete_entry_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "127.0.0.1:9999";
    println!("Dashboard daemon starting on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sysinfo_handler(
    State(state): State<SharedState>
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let lock = state.read().await;
    match &*lock {
        Some(snapshot) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
            );
            headers.insert(
                header::PRAGMA,
                HeaderValue::from_static("no-cache"),
            );
            Ok((headers, Json(snapshot.clone())))
        }
        None => Err((
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "System metrics starting up...".to_string(),
        )),
    }
}

// --- REPOTASKS API LOGIC ---

fn repotasks_projects_file() -> Result<PathBuf, String> {
    let dir = dirs::config_dir().ok_or("Could not determine config dir")?.join("com.repotasks.app");
    Ok(dir.join("projects.json"))
}

fn load_projects() -> Result<Vec<models::Project>, String> {
    let path = repotasks_projects_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

fn find_project(project_id: &str) -> Result<models::Project, String> {
    load_projects()?
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("No project with id {}", project_id))
}

#[derive(Serialize)]
struct ProjectData {
    project: models::Project,
    entries: Vec<models::Entry>,
}

async fn projects_handler() -> Result<Json<Vec<ProjectData>>, (StatusCode, String)> {
    let projects = load_projects().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let mut result = Vec::new();

    for project in projects {
        let note_path = std::path::Path::new(&project.path).join("NOTES.md");
        let entries = if note_path.exists() {
            let content = fs::read_to_string(&note_path).unwrap_or_default();
            notes::parse_notes(&content)
        } else {
            Vec::new()
        };

        result.push(ProjectData { project, entries });
    }

    Ok(Json(result))
}

#[derive(Deserialize)]
struct AddEntryPayload {
    text: String,
    is_todo: bool,
}

async fn add_entry_handler(
    Path(project_id): Path<String>,
    Json(payload): Json<AddEntryPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let text = payload.text.trim();
    if text.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Empty note".into()));
    }

    let project = find_project(&project_id).map_err(|e| (StatusCode::NOT_FOUND, e))?;
    let note_path = std::path::Path::new(&project.path).join("NOTES.md");
    
    let content = if note_path.exists() {
        fs::read_to_string(&note_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        format!("# {} — Notes\n\n## Inbox\n\n## Notes\n", project.name)
    };

    let stamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let line = notes::format_entry(text, payload.is_todo, &stamp);
    let updated = notes::append_to_inbox(&content, &line);

    fs::write(&note_path, updated).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

async fn toggle_entry_handler(
    Path((project_id, line)): Path<(String, usize)>,
) -> Result<StatusCode, (StatusCode, String)> {
    rewrite_notes(&project_id, |c| notes::toggle_todo_at(c, line))
}

#[derive(Deserialize)]
struct UpdateEntryPayload {
    text: String,
}

async fn update_entry_handler(
    Path((project_id, line)): Path<(String, usize)>,
    Json(payload): Json<UpdateEntryPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    rewrite_notes(&project_id, |c| notes::update_text_at(c, line, &payload.text))
}

async fn delete_entry_handler(
    Path((project_id, line)): Path<(String, usize)>,
) -> Result<StatusCode, (StatusCode, String)> {
    rewrite_notes(&project_id, |c| notes::delete_at(c, line))
}

fn rewrite_notes(
    project_id: &str,
    f: impl FnOnce(&str) -> Result<String, String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let project = find_project(project_id).map_err(|e| (StatusCode::NOT_FOUND, e))?;
    let note_path = std::path::Path::new(&project.path).join("NOTES.md");
    
    if !note_path.exists() {
        return Err((StatusCode::NOT_FOUND, "NOTES.md not found".into()));
    }

    let content = fs::read_to_string(&note_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let updated = f(&content).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    fs::write(&note_path, updated).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

