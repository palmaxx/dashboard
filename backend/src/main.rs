use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::{Components, Disks, Networks, System};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

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
    model: Option<String>,
    #[serde(rename = "vramGB")]
    vram_gb: Option<f32>,
}

#[derive(Serialize, Clone, Debug)]
struct NetworkInfo {
    status: String,
    #[serde(rename = "interfaceName")]
    interface_name: Option<String>,
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
struct HistorySample {
    timestamp: String,
    #[serde(rename = "cpuUsagePercent")]
    cpu_usage_percent: f32,
    #[serde(rename = "memoryUsagePercent")]
    memory_usage_percent: f32,
    #[serde(rename = "gpuUsagePercent")]
    gpu_usage_percent: Option<f32>,
    #[serde(rename = "downloadMBps")]
    download_mbps: f32,
    #[serde(rename = "uploadMBps")]
    upload_mbps: f32,
}

#[derive(Clone, Debug, Default)]
struct GpuProbe {
    usage_percent: Option<f32>,
    temperature_c: Option<f32>,
    model: Option<String>,
    vram_gb: Option<f32>,
}

#[derive(Serialize, Clone, Debug)]
struct HardwareSnapshot {
    timestamp: String,
    cpu: CpuInfo,
    memory: MemoryInfo,
    gpu: GpuInfo,
    network: NetworkInfo,
    storage: Vec<StorageInfo>,
    history: Vec<HistorySample>,
}

#[derive(Serialize, Clone, Debug)]
struct PortsSnapshot {
    timestamp: String,
    listeners: Vec<PortListener>,
}

#[derive(Serialize, Clone, Debug)]
struct PortListener {
    protocol: String,
    #[serde(rename = "localAddress")]
    local_address: String,
    port: u16,
    exposure: String,
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none")]
    process_id: Option<u32>,
    #[serde(rename = "processName", skip_serializing_if = "Option::is_none")]
    process_name: Option<String>,
}

struct RawPortListener {
    protocol: String,
    local_address: String,
    port: u16,
    process_id: Option<u32>,
    process_name: Option<String>,
}

type SharedState = Arc<RwLock<Option<HardwareSnapshot>>>;

fn parse_positive_f32(value: Option<&str>) -> Option<f32> {
    value
        .map(str::trim)
        .and_then(|part| part.parse::<f32>().ok())
        .filter(|number| number.is_finite() && *number >= 0.0)
}

fn log_info(message: impl AsRef<str>) {
    println!(
        "[{}] INFO {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message.as_ref()
    );
}

fn log_warn(message: impl AsRef<str>) {
    eprintln!(
        "[{}] WARN {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message.as_ref()
    );
}

fn log_error(message: impl AsRef<str>) {
    eprintln!(
        "[{}] ERROR {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message.as_ref()
    );
}

fn run_probe_command(command: &mut Command) -> std::io::Result<std::process::Output> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.output()
}

fn command_failure(label: &str, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let message = stderr.trim();
    if message.is_empty() {
        format!("{} exited with status {}", label, output.status)
    } else {
        format!(
            "{} exited with status {}: {}",
            label, output.status, message
        )
    }
}

fn megabytes_per_second(bytes: u64, elapsed_seconds: f32) -> f32 {
    if elapsed_seconds <= 0.0 || !elapsed_seconds.is_finite() {
        return 0.0;
    }
    (bytes as f32 / 1024.0 / 1024.0) / elapsed_seconds
}

fn classify_exposure(address: &str) -> &'static str {
    let clean = address.trim().trim_matches(['[', ']']);
    let lower = clean.to_ascii_lowercase();
    let without_scope = lower.split('%').next().unwrap_or(&lower);

    if without_scope == "*" || without_scope == "localhost" {
        return if without_scope == "*" { "lan" } else { "local" };
    }

    match without_scope.parse::<std::net::IpAddr>() {
        Ok(ip) if ip.is_loopback() => "local",
        Ok(_) => "lan",
        Err(_) if without_scope.starts_with("127.") => "local",
        Err(_) => "unknown",
    }
}

fn normalize_port_listener(raw: RawPortListener) -> PortListener {
    let local_address = raw.local_address.trim().to_string();
    let process_name = raw.process_name.and_then(|name| {
        let trimmed = name.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });

    PortListener {
        protocol: raw.protocol.trim().to_ascii_lowercase(),
        exposure: classify_exposure(&local_address).to_string(),
        local_address,
        port: raw.port,
        process_id: raw.process_id,
        process_name,
    }
}

fn parse_listener_endpoint(endpoint: &str) -> Option<(String, u16)> {
    let endpoint = endpoint
        .trim()
        .split(" (")
        .next()
        .unwrap_or_default()
        .trim();
    if endpoint.is_empty() || endpoint.contains("->") {
        return None;
    }

    let (address, port) = if let Some(rest) = endpoint.strip_prefix('[') {
        let (address, port) = rest.rsplit_once("]:")?;
        (address, port)
    } else {
        endpoint.rsplit_once(':')?
    };

    Some((address.to_string(), port.parse().ok()?))
}

#[cfg(any(windows, test))]
fn parse_tasklist_process_names(stdout: &str) -> HashMap<u32, String> {
    let mut names = HashMap::new();
    for line in stdout.lines() {
        let fields: Vec<&str> = line.trim().trim_matches('"').split("\",\"").collect();
        if fields.len() < 2 {
            continue;
        }
        if let Ok(pid) = fields[1].parse() {
            names.insert(pid, fields[0].to_string());
        }
    }
    names
}

#[cfg(any(windows, test))]
fn parse_netstat_ports(stdout: &str, process_names: &HashMap<u32, String>) -> Vec<PortListener> {
    let mut listeners = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let protocol = parts[0].to_ascii_lowercase();
        let (local_address, port, process_id) = match protocol.as_str() {
            "tcp" if parts.len() >= 5 && parts[3] == "LISTENING" => {
                let Some((local_address, port)) = parse_listener_endpoint(parts[1]) else {
                    continue;
                };
                (local_address, port, parts[4].parse().ok())
            }
            "udp" => {
                let Some((local_address, port)) = parse_listener_endpoint(parts[1]) else {
                    continue;
                };
                (local_address, port, parts.last().and_then(|pid| pid.parse().ok()))
            }
            _ => continue,
        };

        listeners.push(normalize_port_listener(RawPortListener {
            protocol,
            local_address,
            port,
            process_id,
            process_name: process_id.and_then(|pid| process_names.get(&pid).cloned()),
        }));
    }

    listeners
}

#[cfg(any(target_os = "macos", test))]
fn parse_lsof_ports(stdout: &str) -> Vec<PortListener> {
    let mut listeners = Vec::new();
    let mut process_id = None;
    let mut process_name = None;
    let mut protocol = String::new();

    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        let (field, value) = line.split_at(1);
        match field {
            "p" => {
                process_id = value.trim().parse().ok();
                process_name = None;
                protocol.clear();
            }
            "c" => process_name = Some(value.trim().to_string()),
            "P" => protocol = value.trim().to_ascii_lowercase(),
            "n" => {
                let Some((local_address, port)) = parse_listener_endpoint(value) else {
                    continue;
                };
                listeners.push(normalize_port_listener(RawPortListener {
                    protocol: protocol.clone(),
                    local_address,
                    port,
                    process_id,
                    process_name: process_name.clone(),
                }));
            }
            _ => {}
        }
    }

    listeners
}

fn exposure_rank(exposure: &str) -> u8 {
    match exposure {
        "lan" => 0,
        "unknown" => 1,
        _ => 2,
    }
}

fn sort_port_listeners(listeners: &mut [PortListener]) {
    listeners.sort_by(|a, b| {
        exposure_rank(&a.exposure)
            .cmp(&exposure_rank(&b.exposure))
            .then(a.port.cmp(&b.port))
            .then(a.protocol.cmp(&b.protocol))
            .then(a.local_address.cmp(&b.local_address))
    });
}

#[cfg(windows)]
fn probe_ports() -> Result<Vec<PortListener>, String> {
    let mut netstat = Command::new("netstat");
    netstat.args(["-ano"]);

    let output = run_probe_command(&mut netstat)
        .map_err(|error| format!("netstat port probe could not be started: {}", error))?;
    if !output.status.success() {
        return Err(command_failure("netstat port probe", &output));
    }

    let process_names = probe_windows_process_names().unwrap_or_default();
    Ok(parse_netstat_ports(
        &String::from_utf8_lossy(&output.stdout),
        &process_names,
    ))
}

#[cfg(windows)]
fn probe_windows_process_names() -> Result<HashMap<u32, String>, String> {
    let mut tasklist = Command::new("tasklist");
    tasklist.args(["/FO", "CSV", "/NH"]);

    let output = run_probe_command(&mut tasklist)
        .map_err(|error| format!("tasklist process probe could not be started: {}", error))?;
    if !output.status.success() {
        return Err(command_failure("tasklist process probe", &output));
    }

    Ok(parse_tasklist_process_names(&String::from_utf8_lossy(
        &output.stdout,
    )))
}

#[cfg(target_os = "macos")]
fn probe_ports() -> Result<Vec<PortListener>, String> {
    let mut command = Command::new("lsof");
    command.args(["-nP", "-iTCP", "-sTCP:LISTEN", "-iUDP", "-F", "pcPn"]);

    let output = run_probe_command(&mut command)
        .map_err(|error| format!("lsof port probe could not be started: {}", error))?;
    if !output.status.success() {
        return Err(command_failure("lsof port probe", &output));
    }

    Ok(parse_lsof_ports(&String::from_utf8_lossy(&output.stdout)))
}

#[cfg(not(any(windows, target_os = "macos")))]
fn probe_ports() -> Result<Vec<PortListener>, String> {
    Err("Port audit is only implemented for Windows and macOS".to_string())
}

fn probe_nvidia_gpu() -> Result<GpuProbe, String> {
    let mut command = Command::new("nvidia-smi");
    command.args([
        "--query-gpu=name,utilization.gpu,temperature.gpu,memory.total",
        "--format=csv,noheader,nounits",
    ]);

    let output = run_probe_command(&mut command)
        .map_err(|error| format!("nvidia-smi could not be started: {}", error))?;

    if !output.status.success() {
        return Err(command_failure("nvidia-smi", &output));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "nvidia-smi returned no GPU rows".to_string())?;
    let parts: Vec<&str> = first_line.split(',').collect();
    let model = parts
        .first()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let usage_percent = parse_positive_f32(parts.get(1).copied());
    let temperature_c = parse_positive_f32(parts.get(2).copied());
    let vram_gb = parse_positive_f32(parts.get(3).copied()).map(|mib| mib / 1024.0);

    Ok(GpuProbe {
        usage_percent,
        temperature_c,
        model,
        vram_gb,
    })
}

#[cfg(windows)]
fn probe_windows_gpu_metadata() -> Result<GpuProbe, String> {
    let mut command = Command::new("powershell.exe");
    command.args([
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
        "Get-CimInstance Win32_VideoController | Select-Object -First 1 Name,AdapterRAM | ForEach-Object { \"{0}|{1}\" -f $_.Name,$_.AdapterRAM }",
    ]);

    let output = run_probe_command(&mut command).map_err(|error| {
        format!(
            "PowerShell GPU metadata probe could not be started: {}",
            error
        )
    })?;

    if !output.status.success() {
        return Err(command_failure("PowerShell GPU metadata probe", &output));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(first_line) = stdout.lines().find(|line| !line.trim().is_empty()) else {
        return Err("PowerShell GPU metadata probe returned no rows".to_string());
    };

    let mut parts = first_line.split('|');
    let model = parts
        .next()
        .map(str::trim)
        .map(str::to_string)
        .filter(|value| !value.is_empty());
    let vram_gb = parts
        .next()
        .and_then(|value| value.trim().parse::<f32>().ok())
        .filter(|bytes| bytes.is_finite() && *bytes > 0.0)
        .map(|bytes| bytes / 1024.0 / 1024.0 / 1024.0);

    Ok(GpuProbe {
        model,
        vram_gb,
        ..GpuProbe::default()
    })
}

#[cfg(not(windows))]
fn probe_windows_gpu_metadata() -> Result<GpuProbe, String> {
    Ok(GpuProbe::default())
}

#[cfg(windows)]
fn probe_windows_cpu_model() -> Result<String, String> {
    let mut command = Command::new("powershell.exe");
    command.args([
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
        "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name",
    ]);

    let output = run_probe_command(&mut command)
        .map_err(|error| format!("PowerShell CPU metadata probe could not be started: {}", error))?;

    if !output.status.success() {
        return Err(command_failure("PowerShell CPU metadata probe", &output));
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            let model = line.trim();
            (!model.is_empty()).then(|| model.to_string())
        })
        .ok_or_else(|| "PowerShell CPU metadata probe returned no rows".to_string())
}

#[cfg(not(windows))]
fn probe_windows_cpu_model() -> Result<String, String> {
    Ok(String::new())
}

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

        let mut prev_time = Instant::now();
        let mut history: VecDeque<HistorySample> = VecDeque::with_capacity(150);
        let mut first_snapshot_logged = false;
        let mut gpu_source = String::new();
        let mut nvidia_failure_logged = false;
        let cpu_model_fallback = match probe_windows_cpu_model() {
            Ok(model) => {
                if !model.is_empty() {
                    log_info(format!("Windows CPU metadata detected: {}", model));
                }
                model
            }
            Err(error) => {
                log_warn(format!("Windows CPU metadata unavailable: {}", error));
                String::new()
            }
        };
        let gpu_metadata = match probe_windows_gpu_metadata() {
            Ok(metadata) => {
                if let Some(model) = metadata.model.as_deref() {
                    log_info(format!("Windows GPU metadata detected: {}", model));
                }
                metadata
            }
            Err(error) => {
                log_warn(format!("Windows GPU metadata unavailable: {}", error));
                GpuProbe::default()
            }
        };

        // Initial refresh
        sys.refresh_all();
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            // Refresh system info
            sys.refresh_cpu();
            sys.refresh_memory();
            disks.refresh_list();
            networks.refresh_list();
            networks.refresh();
            components.refresh_list();

            let now = Instant::now();
            let elapsed = now.duration_since(prev_time).as_secs_f32();
            prev_time = now;

            // CPU
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let cpu_model = {
                let model = sys.global_cpu_info().brand().trim();
                if model.is_empty() {
                    cpu_model_fallback.clone()
                } else {
                    model.to_string()
                }
            };
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
            let mut active_iface = None;
            let mut max_activity = 0u64;

            for (name, data) in &networks {
                let rx = data.received();
                let tx = data.transmitted();
                total_rx += rx;
                total_tx += tx;

                let activity = rx.saturating_add(tx);
                if active_iface.is_none() || activity > max_activity {
                    max_activity = activity;
                    active_iface = Some(name.clone());
                }
            }

            let download_mbps = megabytes_per_second(total_rx, elapsed);
            let upload_mbps = megabytes_per_second(total_tx, elapsed);
            let network_status = if active_iface.is_some() {
                "online"
            } else {
                "offline"
            }
            .to_string();
            let (gpu_probe, next_gpu_source) = match probe_nvidia_gpu() {
                Ok(probe) => {
                    nvidia_failure_logged = false;
                    (probe, "nvidia-smi")
                }
                Err(error) => {
                    if !nvidia_failure_logged {
                        log_warn(format!(
                            "NVIDIA telemetry unavailable; using Windows GPU metadata where possible: {}",
                            error
                        ));
                        nvidia_failure_logged = true;
                    }
                    (gpu_metadata.clone(), "windows-metadata")
                }
            };

            if gpu_source != next_gpu_source {
                log_info(format!("GPU telemetry source: {}", next_gpu_source));
                gpu_source = next_gpu_source.to_string();
            }

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
                }
                .to_string();

                storage.push(StorageInfo {
                    name: if name.is_empty() { mount.clone() } else { name },
                    mount,
                    disk_type,
                    used_gb,
                    total_gb,
                    usage_percent,
                });
            }

            let timestamp = chrono::Utc::now().to_rfc3339();
            history.push_back(HistorySample {
                timestamp: timestamp.clone(),
                cpu_usage_percent: cpu_usage,
                memory_usage_percent: mem_usage_percent,
                gpu_usage_percent: gpu_probe.usage_percent,
                download_mbps,
                upload_mbps,
            });
            while history.len() > 150 {
                history.pop_front();
            }

            // Create Snapshot
            let snapshot = HardwareSnapshot {
                timestamp,
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
                    usage_percent: gpu_probe.usage_percent,
                    temperature_c: gpu_probe.temperature_c,
                    model: gpu_probe.model,
                    vram_gb: gpu_probe.vram_gb,
                },
                network: NetworkInfo {
                    status: network_status,
                    interface_name: active_iface,
                    download_mbps,
                    upload_mbps,
                },
                storage,
                history: history.iter().cloned().collect(),
            };

            // Write state
            let mut lock = state_clone.write().await;
            *lock = Some(snapshot);
            if !first_snapshot_logged {
                log_info("System metrics are live; rolling history is collecting 150 samples");
                first_snapshot_logged = true;
            }
        }
    });

    // Build the Axum router
    let app = Router::new()
        .route("/api/sysinfo", get(sysinfo_handler))
        .route("/api/ports", get(ports_handler))
        .route("/api/projects", get(projects_handler))
        .route("/api/projects/:project_id/add", post(add_entry_handler))
        .route(
            "/api/projects/:project_id/toggle/:line",
            post(toggle_entry_handler),
        )
        .route(
            "/api/projects/:project_id/update/:line",
            put(update_entry_handler),
        )
        .route(
            "/api/projects/:project_id/delete/:line",
            delete(delete_entry_handler),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "127.0.0.1:9999";
    log_info(format!("Dashboard daemon starting on http://{}", addr));
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(error) => {
            log_error(format!("Could not bind {}: {}", addr, error));
            return;
        }
    };

    if let Err(error) = axum::serve(listener, app).await {
        log_error(format!("Dashboard daemon stopped: {}", error));
    }
}

async fn sysinfo_handler(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let lock = state.read().await;
    match &*lock {
        Some(snapshot) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
            );
            headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
            Ok((headers, Json(snapshot.clone())))
        }
        None => Err((
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "System metrics starting up...".to_string(),
        )),
    }
}

async fn ports_handler() -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut listeners =
        probe_ports().map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;
    sort_port_listeners(&mut listeners);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
    );
    headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));

    Ok((
        headers,
        Json(PortsSnapshot {
            timestamp: chrono::Utc::now().to_rfc3339(),
            listeners,
        }),
    ))
}

// --- REPOTASKS API LOGIC ---

fn repotasks_projects_file() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or("Could not determine config dir")?
        .join("com.repotasks.app");
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
        fs::read_to_string(&note_path)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        format!("# {} — Notes\n\n## Inbox\n\n## Notes\n", project.name)
    };

    let stamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let line = notes::format_entry(text, payload.is_todo, &stamp);
    let updated = notes::append_to_inbox(&content, &line);

    fs::write(&note_path, updated)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
    rewrite_notes(&project_id, |c| {
        notes::update_text_at(c, line, &payload.text)
    })
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

    let content = fs::read_to_string(&note_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let updated = f(&content).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    fs::write(&note_path, updated)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_network_deltas_to_megabytes_per_second() {
        assert!((megabytes_per_second(2 * 1024 * 1024, 2.0) - 1.0).abs() < f32::EPSILON);
        assert_eq!(megabytes_per_second(1024, 0.0), 0.0);
    }

    #[test]
    fn classifies_bind_exposure() {
        assert_eq!(classify_exposure("127.0.0.1"), "local");
        assert_eq!(classify_exposure("::1"), "local");
        assert_eq!(classify_exposure("0.0.0.0"), "lan");
        assert_eq!(classify_exposure("::"), "lan");
        assert_eq!(classify_exposure("192.168.1.20"), "lan");
        assert_eq!(classify_exposure("not-an-address"), "unknown");
    }

    #[test]
    fn parses_windows_netstat_output() {
        let process_names = parse_tasklist_process_names(
            "\"python.exe\",\"1234\",\"Console\",\"1\",\"10,000 K\"\n\"node.exe\",\"5678\",\"Console\",\"1\",\"50,000 K\"\n",
        );
        let ports = parse_netstat_ports(
            "  TCP    0.0.0.0:8188       0.0.0.0:0      LISTENING       1234\n  UDP    127.0.0.1:5353      *:*                           5678\n",
            &process_names,
        );

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].protocol, "tcp");
        assert_eq!(ports[0].exposure, "lan");
        assert_eq!(ports[0].process_name.as_deref(), Some("python.exe"));
        assert_eq!(ports[1].exposure, "local");
        assert_eq!(ports[1].process_name.as_deref(), Some("node.exe"));
    }

    #[test]
    fn parses_lsof_port_output() {
        let ports = parse_lsof_ports(
            "p123\ncpython\nPTCP\nn*:8188\np456\ncbackend\nPTCP\nn127.0.0.1:9999\np789\ncmDNSResponder\nPUDP\nn[::1]:5353\n",
        );

        assert_eq!(ports.len(), 3);
        assert_eq!(ports[0].local_address, "*");
        assert_eq!(ports[0].exposure, "lan");
        assert_eq!(ports[1].process_name.as_deref(), Some("backend"));
        assert_eq!(ports[1].exposure, "local");
        assert_eq!(ports[2].protocol, "udp");
        assert_eq!(ports[2].exposure, "local");
    }
}
