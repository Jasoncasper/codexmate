use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use codexmate_core::install::SILENT_BINARY;
use codexmate_core::settings::SettingsStore;
use codexmate_core::status::{LaunchStatus, StatusStore};
use serde::Serialize;
use serde_json::{Value, json};

use crate::install;

#[derive(Debug, Clone, Serialize)]
pub struct CommandResult<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionPayload {
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PathState {
    pub status: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverviewPayload {
    pub codex_app: PathState,
    pub codex_version: Option<String>,
    pub silent_shortcut: PathState,
    pub management_shortcut: PathState,
    pub latest_launch: Option<LaunchStatus>,
    pub current_version: String,
    pub update_status: String,
    pub settings_path: String,
    pub logs_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayProfileTestPayload {
    pub http_status: u16,
    pub endpoint: String,
    pub response_preview: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRequest {
    #[serde(default)]
    pub app_path: String,
    #[serde(default = "default_debug_port")]
    pub debug_port: u16,
    #[serde(default = "default_helper_port")]
    pub helper_port: u16,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogRequest {
    #[serde(default = "default_log_lines")]
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogsPayload {
    pub path: String,
    pub text: String,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticsPayload {
    pub report: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupPayload {
    pub show_update: bool,
}

#[tauri::command]
pub fn backend_version() -> CommandResult<VersionPayload> {
    ok(
        "后端版本已读取。",
        VersionPayload {
            version: codexmate_core::version::VERSION.to_string(),
        },
    )
}

#[tauri::command]
pub fn startup_options() -> CommandResult<StartupPayload> {
    ok(
        "启动参数已读取。",
        StartupPayload {
            show_update: startup_should_show_update(),
        },
    )
}

pub fn startup_should_show_update() -> bool {
    should_show_update(std::env::args(), std::env::var("CODEXMATE_SHOW_UPDATE").ok().as_deref())
}

fn should_show_update<I, S>(args: I, env_value: Option<&str>) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter().any(|arg| arg.as_ref() == "--show-update") || env_value == Some("1")
}

#[tauri::command]
pub async fn load_overview() -> CommandResult<OverviewPayload> {
    let payload = tauri::async_runtime::spawn_blocking(load_overview_payload).await;
    let Ok((codex_app_path, entrypoints, latest_launch)) = payload else {
        return failed(
            "概览后台任务失败。",
            OverviewPayload {
                codex_app: path_state(None),
                codex_version: None,
                silent_shortcut: path_state(None),
                management_shortcut: path_state(None),
                latest_launch: None,
                current_version: codexmate_core::version::VERSION.to_string(),
                update_status: "not_checked".to_string(),
                settings_path: codexmate_core::paths::default_settings_path()
                    .to_string_lossy()
                    .to_string(),
                logs_path: codexmate_core::paths::default_diagnostic_log_path()
                    .to_string_lossy()
                    .to_string(),
            },
        );
    };
    ok(
        "概览已加载。",
        OverviewPayload {
            codex_version: codex_app_path
                .as_deref()
                .and_then(codexmate_core::app_paths::codex_app_version),
            codex_app: path_state(codex_app_path),
            silent_shortcut: shortcut_state(entrypoints.silent_shortcut),
            management_shortcut: shortcut_state(entrypoints.management_shortcut),
            latest_launch,
            current_version: codexmate_core::version::VERSION.to_string(),
            update_status: "not_checked".to_string(),
            settings_path: codexmate_core::paths::default_settings_path()
                .to_string_lossy()
                .to_string(),
            logs_path: codexmate_core::paths::default_diagnostic_log_path()
                .to_string_lossy()
                .to_string(),
        },
    )
}

#[tauri::command]
pub fn launch_codexmate(request: LaunchRequest) -> CommandResult<Value> {
    spawn_codexmate_launch(request, "已在后台开始，可稍后查看概览状态。")
}

#[tauri::command]
pub fn restart_codexmate(request: LaunchRequest) -> CommandResult<Value> {
    codexmate_core::watcher::stop_launcher_processes();
    codexmate_core::watcher::stop_codex_processes();
    spawn_codexmate_launch(request, "Codex 已请求重启，启动任务正在后台运行。")
}

fn spawn_codexmate_launch(request: LaunchRequest, accepted_message: &str) -> CommandResult<Value> {
    let debug_port = request.debug_port;
    let helper_port = request.helper_port;
    let _ = codexmate_core::diagnostic_log::append_diagnostic_log(
        "manager.launch_requested",
        json!({
            "debug_port": debug_port,
            "helper_port": helper_port,
            "app_path": request.app_path.trim()
        }),
    );
    match spawn_silent_launcher(&request) {
        Ok(()) => CommandResult {
            status: "accepted".to_string(),
            message: accepted_message.to_string(),
            payload: json!({
                "debugPort": debug_port,
                "helperPort": helper_port
            }),
        },
        Err(error) => failed(
            &format!("启动静默入口失败：{error}"),
            json!({
                "debugPort": debug_port,
                "helperPort": helper_port
            }),
        ),
    }
}

fn spawn_silent_launcher(request: &LaunchRequest) -> anyhow::Result<()> {
    let launcher = codexmate_core::install::companion_binary_path(SILENT_BINARY);
    let mut command = std::process::Command::new(&launcher);
    if !request.app_path.trim().is_empty() {
        command.arg("--app-path").arg(request.app_path.trim());
    }
    command
        .arg("--debug-port")
        .arg(request.debug_port.to_string())
        .arg("--helper-port")
        .arg(request.helper_port.to_string());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    command
        .spawn()
        .map(|_| ())
        .map_err(|error| anyhow::anyhow!("无法启动 {}：{error}", launcher.to_string_lossy()))
}

#[tauri::command]
pub fn open_external_url(url: String) -> CommandResult<Value> {
    let trimmed = url.trim();
    if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) {
        return failed("只允许打开 http 或 https 链接。", json!({}));
    }
    match open_url(trimmed) {
        Ok(()) => ok("已在系统浏览器打开链接。", json!({ "url": trimmed })),
        Err(error) => failed(&format!("打开链接失败：{error}"), json!({ "url": trimmed })),
    }
}

#[tauri::command]
pub async fn check_update() -> CommandResult<Value> {
    match codexmate_core::update::check_for_update(codexmate_core::version::VERSION).await {
        Ok(update) => {
            let status = if update.update_available {
                "ok"
            } else {
                "not_checked"
            };
            CommandResult {
                status: status.to_string(),
                message: if update.update_available {
                    "发现可用更新。".to_string()
                } else {
                    "当前已是最新版本。".to_string()
                },
                payload: json!({
                    "currentVersion": update.current_version,
                    "latestVersion": update.latest_version,
                    "releaseSummary": update.release_summary,
                    "assetName": update.asset_name,
                    "assetUrl": update.asset_url,
                    "updateAvailable": update.update_available,
                    "progress": 0
                }),
            }
        }
        Err(error) => failed(
            &format!("检查更新失败：{error}"),
            json!({
                "currentVersion": codexmate_core::version::VERSION,
                "latestVersion": Value::Null,
                "releaseSummary": "",
                "assetName": Value::Null,
                "assetUrl": Value::Null,
                "updateAvailable": false,
                "progress": 0
            }),
        ),
    }
}

#[tauri::command]
pub async fn perform_update(
    release: Option<codexmate_core::update::Release>,
) -> CommandResult<Value> {
    let Some(release) = release else {
        return failed(
            "请先检查更新并选择可下载的 Release asset。",
            json!({
                "currentVersion": codexmate_core::version::VERSION,
                "progress": 0
            }),
        );
    };
    let download_dir = codexmate_core::paths::default_app_state_dir().join("updates");
    match codexmate_core::update::perform_update(&release, &download_dir).await {
        Ok(result) => ok(
            "安装包已下载并启动，请按安装向导完成更新。",
            json!({
                "currentVersion": codexmate_core::version::VERSION,
                "latestVersion": result.release.version,
                "releaseSummary": result.release.body,
                "installedPath": result.installer_path.to_string_lossy(),
                "launched": result.launched,
                "progress": 100
            }),
        ),
        Err(error) => failed(
            &format!("安装更新失败：{error}"),
            json!({
                "currentVersion": codexmate_core::version::VERSION,
                "latestVersion": release.version,
                "releaseSummary": release.body,
                "progress": 0
            }),
        ),
    }
}

#[tauri::command]
pub fn read_latest_logs(request: LogRequest) -> CommandResult<LogsPayload> {
    let path = codexmate_core::paths::default_diagnostic_log_path();
    match read_tail(&path, request.lines) {
        Ok(text) => ok(
            "日志已读取。",
            LogsPayload {
                path: path.to_string_lossy().to_string(),
                text,
                lines: request.lines,
            },
        ),
        Err(error) => failed(
            &format!("读取日志失败：{error}"),
            LogsPayload {
                path: path.to_string_lossy().to_string(),
                text: String::new(),
                lines: request.lines,
            },
        ),
    }
}

#[tauri::command]
pub fn copy_diagnostics() -> CommandResult<DiagnosticsPayload> {
    ok(
        "诊断报告已生成。",
        DiagnosticsPayload {
            report: diagnostics_report(),
        },
    )
}

#[tauri::command]
pub fn write_diagnostic_event(event: String, detail: Value) -> CommandResult<Value> {
    let event = sanitize_manager_event(&event);
    match codexmate_core::diagnostic_log::append_diagnostic_log(&event, detail) {
        Ok(()) => ok("诊断日志已写入。", json!({})),
        Err(error) => failed(&format!("写入诊断日志失败：{error}"), json!({})),
    }
}

fn sync_all_session_providers(home: &Path) -> codexmate_data::ProviderSyncResult {
    codexmate_data::run_provider_sync(Some(home))
}

fn strip_model_provider_from_toml(contents: &str) -> String {
    // 直连模式：只删 model_provider = "custom"，保留 [model_providers.custom] 定义
    // 这样 Codex 可以加载 custom 会话（知道 provider 配置），但默认用原生通道
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.trim() == "model_provider = \"custom\"" {
            continue;
        }
        result.push(line.to_string());
    }
    result.join("\n")
}

fn sanitize_manager_event(event: &str) -> String {
    let suffix = event
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    let suffix = suffix.trim_matches(['.', '_', '-']).trim();
    if suffix.is_empty() {
        "manager.ui.event".to_string()
    } else if suffix.starts_with("manager.") {
        suffix.to_string()
    } else {
        format!("manager.ui.{suffix}")
    }
}

fn open_url(url: &str) -> anyhow::Result<()> {
    #[cfg(windows)]
    {
        codexmate_core::windows_open_url(url)
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map(|_| ())
            .map_err(|error| anyhow::anyhow!("启动系统浏览器失败：{error}"))
    }
}

fn diagnostics_report() -> String {
    let (codex_app_path, entrypoints, latest_launch) = load_overview_payload();
    let overview = ok(
        "概览已加载。",
        OverviewPayload {
            codex_version: codex_app_path
                .as_deref()
                .and_then(codexmate_core::app_paths::codex_app_version),
            codex_app: path_state(codex_app_path),
            silent_shortcut: shortcut_state(entrypoints.silent_shortcut),
            management_shortcut: shortcut_state(entrypoints.management_shortcut),
            latest_launch,
            current_version: codexmate_core::version::VERSION.to_string(),
            update_status: "not_checked".to_string(),
            settings_path: codexmate_core::paths::default_settings_path()
                .to_string_lossy()
                .to_string(),
            logs_path: codexmate_core::paths::default_diagnostic_log_path()
                .to_string_lossy()
                .to_string(),
        },
    );
    let settings = SettingsStore::default().load().unwrap_or_default();
    let generated_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    serde_json::to_string_pretty(&json!({
        "generatedAtMs": generated_at_ms,
        "version": codexmate_core::version::VERSION,
        "overview": overview.payload,
        "settings": settings,
        "logs": {
            "diagnosticLogPath": codexmate_core::paths::default_diagnostic_log_path(),
            "latestStatusPath": codexmate_core::paths::default_latest_status_path()
        },
        "platform": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    }))
    .unwrap_or_else(|error| format!("诊断报告序列化失败：{error}"))
}

fn load_overview_payload() -> (
    Option<PathBuf>,
    install::EntryPointState,
    Option<LaunchStatus>,
) {
    let settings = SettingsStore::default().load().unwrap_or_default();
    (
        codexmate_core::app_paths::resolve_codex_app_dir_with_saved(
            None,
            Some(settings.codex_app_path.as_str()),
        ),
        install::inspect_entrypoints(),
        StatusStore::default().load_latest().unwrap_or(None),
    )
}

fn read_tail(path: &Path, max_lines: usize) -> std::io::Result<String> {
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines().rev().take(max_lines).collect::<Vec<_>>();
    lines.reverse();
    Ok(lines.join("\n"))
}

fn path_state(path: Option<PathBuf>) -> PathState {
    match path {
        Some(path) => PathState {
            status: "found".to_string(),
            path: Some(path.to_string_lossy().to_string()),
        },
        None => PathState {
            status: "missing".to_string(),
            path: None,
        },
    }
}

fn shortcut_state(shortcut: install::ShortcutState) -> PathState {
    PathState {
        status: if shortcut.installed {
            "installed".to_string()
        } else {
            "missing".to_string()
        },
        path: shortcut.path,
    }
}

fn ok<T: Serialize>(message: &str, payload: T) -> CommandResult<T> {
    CommandResult {
        status: "ok".to_string(),
        message: message.to_string(),
        payload,
    }
}

fn failed<T: Serialize>(message: &str, payload: T) -> CommandResult<T> {
    CommandResult {
        status: "failed".to_string(),
        message: message.to_string(),
        payload,
    }
}

fn default_debug_port() -> u16 {
    9229
}

fn default_helper_port() -> u16 {
    57321
}

fn default_log_lines() -> usize {
    200
}

// ==================== 智能路由管理命令 ====================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingConfigPayload {
    pub config: codexmate_core::router::SmartRouterConfig,
    pub config_path: String,
}

fn normalize_routing_config(
    config: codexmate_core::router::SmartRouterConfig,
) -> codexmate_core::router::SmartRouterConfig {
    codexmate_core::router::normalize_router_config(config)
}

#[tauri::command]
pub fn load_routing_config() -> CommandResult<RoutingConfigPayload> {
    let config_path = codexmate_core::paths::default_app_state_dir().join("routing.toml");
    let config = if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|c| toml::from_str(&c).ok())
            .unwrap_or_default()
    } else {
        codexmate_core::router::SmartRouterConfig::default()
    };
    let config = normalize_routing_config(config);
    CommandResult {
        status: "ok".to_string(),
        message: "路由配置加载成功".to_string(),
        payload: RoutingConfigPayload {
            config,
            config_path: config_path.to_string_lossy().to_string(),
        },
    }
}

#[tauri::command]
pub fn save_routing_config(
    mut config: codexmate_core::router::SmartRouterConfig,
) -> CommandResult<RoutingConfigPayload> {
    let config_path = codexmate_core::paths::default_app_state_dir().join("routing.toml");
    // 保持原有 API key（前端可能发回的是脱敏值）
    if let Ok(old_raw) = std::fs::read_to_string(&config_path) {
        if let Ok(old_config) =
            toml::from_str::<codexmate_core::router::SmartRouterConfig>(&old_raw)
        {
            for provider in &mut config.providers {
                if let Some(old) = old_config.providers.iter().find(|p| p.id == provider.id) {
                    if provider.api_key == codexmate_core::router::api_key_masked_str(&old.api_key)
                    {
                        provider.api_key = old.api_key.clone();
                    }
                }
            }
        }
    }
    config = normalize_routing_config(config);
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let toml_content = match toml::to_string_pretty(&config) {
        Ok(c) => c,
        Err(e) => {
            return CommandResult {
                status: "failed".to_string(),
                message: format!("序列化配置失败: {}", e),
                payload: RoutingConfigPayload {
                    config,
                    config_path: config_path.to_string_lossy().to_string(),
                },
            };
        }
    };
    match std::fs::write(&config_path, &toml_content) {
        Ok(_) => CommandResult {
            status: "ok".to_string(),
            message: "路由配置保存成功".to_string(),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
        Err(e) => CommandResult {
            status: "failed".to_string(),
            message: format!("保存配置失败: {}", e),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
    }
}

#[tauri::command]
pub fn upsert_provider(
    mut provider: codexmate_core::router::SmartProvider,
) -> CommandResult<RoutingConfigPayload> {
    let config_path = codexmate_core::paths::default_app_state_dir().join("routing.toml");
    let mut config: codexmate_core::router::SmartRouterConfig = if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|c| toml::from_str(&c).ok())
            .unwrap_or_default()
    } else {
        codexmate_core::router::SmartRouterConfig::default()
    };
    config = normalize_routing_config(config);
    if provider.builtin || provider.id == "openai" {
        return CommandResult {
            status: "failed".to_string(),
            message: "内置 provider 不可编辑".to_string(),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        };
    }
    if let Some(existing) = config.providers.iter_mut().find(|p| p.id == provider.id) {
        // 保持原有 API key（前端可能发回的是脱敏值）
        let incoming_key = std::mem::replace(&mut provider.api_key, String::new());
        if incoming_key == codexmate_core::router::api_key_masked_str(&existing.api_key) {
            provider.api_key = existing.api_key.clone();
        } else {
            provider.api_key = incoming_key;
        }
        *existing = provider;
    } else {
        config.providers.push(provider);
    }
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let toml_content = match toml::to_string_pretty(&config) {
        Ok(c) => c,
        Err(e) => {
            return CommandResult {
                status: "failed".to_string(),
                message: format!("序列化配置失败: {}", e),
                payload: RoutingConfigPayload {
                    config,
                    config_path: config_path.to_string_lossy().to_string(),
                },
            };
        }
    };
    match std::fs::write(&config_path, &toml_content) {
        Ok(_) => CommandResult {
            status: "ok".to_string(),
            message: "模型保存成功".to_string(),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
        Err(e) => CommandResult {
            status: "failed".to_string(),
            message: format!("保存配置失败: {}", e),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProviderRequest {
    provider_id: String,
}

#[tauri::command]
pub fn delete_provider(request: DeleteProviderRequest) -> CommandResult<RoutingConfigPayload> {
    let provider_id = request.provider_id;
    let config_path = codexmate_core::paths::default_app_state_dir().join("routing.toml");
    let mut config: codexmate_core::router::SmartRouterConfig = if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|c| toml::from_str(&c).ok())
            .unwrap_or_default()
    } else {
        codexmate_core::router::SmartRouterConfig::default()
    };
    config = normalize_routing_config(config);
    if provider_id == "openai" {
        return CommandResult {
            status: "failed".to_string(),
            message: "内置 provider 不可删除".to_string(),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        };
    }
    config.providers.retain(|p| p.id != provider_id);
    if config.vision_fallback_model == provider_id {
        config.vision_fallback_model = String::new();
    }
    if config.fallback_provider == provider_id {
        config.fallback_provider = "openai".to_string();
    }
    config = normalize_routing_config(config);
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let toml_content = match toml::to_string_pretty(&config) {
        Ok(c) => c,
        Err(e) => {
            return CommandResult {
                status: "failed".to_string(),
                message: format!("序列化配置失败: {}", e),
                payload: RoutingConfigPayload {
                    config,
                    config_path: config_path.to_string_lossy().to_string(),
                },
            };
        }
    };
    match std::fs::write(&config_path, &toml_content) {
        Ok(_) => CommandResult {
            status: "ok".to_string(),
            message: "模型已删除".to_string(),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
        Err(e) => CommandResult {
            status: "failed".to_string(),
            message: format!("删除配置失败: {}", e),
            payload: RoutingConfigPayload {
                config,
                config_path: config_path.to_string_lossy().to_string(),
            },
        },
    }
}

#[tauri::command]
pub fn get_codex_mode() -> CommandResult<Value> {
    let home = codexmate_core::relay_config::default_codex_home_dir();
    let config_path = home.join("config.toml");
    let mode = match std::fs::read_to_string(&config_path) {
        Ok(contents) if contents.contains("model_provider") => "proxy",
        _ => "direct",
    };
    CommandResult {
        status: "ok".to_string(),
        message: format!("当前模式: {}", if mode == "proxy" { "代理" } else { "直连" }),
        payload: json!({"mode": mode}),
    }
}

#[tauri::command]
pub fn set_codex_mode(mode: String) -> CommandResult<Value> {
    let home = codexmate_core::relay_config::default_codex_home_dir();
    let helper_base_url =
        codexmate_core::protocol_proxy::local_responses_proxy_base_url(default_helper_port());
    let config_path = home.join("config.toml");
    let existing = std::fs::read_to_string(&config_path).unwrap_or_default();
    let mut doc = match codexmate_core::relay_config::parse_toml_document(&existing) {
        Ok(doc) => doc,
        Err(e) => {
            return CommandResult {
                status: "failed".to_string(),
                message: format!("解析 config.toml 失败: {e}"),
                payload: json!({"mode": mode}),
            };
        }
    };

    let changed = if mode == "proxy" {
        codexmate_core::relay_config::sync_scoped_model_provider(&mut doc, &helper_base_url)
            .unwrap_or(false)
    } else {
        codexmate_core::relay_config::clear_scoped_model_provider_if_managed(
            &mut doc,
            &helper_base_url,
        )
    };

    let content = if mode == "direct" {
        codexmate_core::relay_config::ensure_trailing_newline(
            strip_model_provider_from_toml(&existing)
        )
    } else {
        codexmate_core::relay_config::ensure_trailing_newline(doc.to_string())
    };
    if let Err(e) = std::fs::write(&config_path, content.as_bytes()) {
        return CommandResult {
            status: "failed".to_string(),
            message: format!("写入 config.toml 失败: {e}"),
            payload: json!({"mode": mode}),
        };
    }

    // 统一所有会话的 model_provider，确保不分裂
    let sync = sync_all_session_providers(&home);
    if sync.status != codexmate_data::ProviderSyncStatus::Synced {
        return CommandResult {
            status: "failed".to_string(),
            message: format!("模式配置已更新，但存量会话同步失败：{}", sync.message),
            payload: json!({
                "mode": mode,
                "changed": changed,
                "sessionFilesUpdated": sync.changed_session_files,
                "sqliteRowsUpdated": sync.sqlite_rows_updated,
            }),
        };
    }
    let synced = sync.changed_session_files + sync.sqlite_rows_updated;

    CommandResult {
        status: "ok".to_string(),
        message: format!(
            "已切换到{}模式{}",
            if mode == "proxy" { "代理" } else { "直连" },
            if synced > 0 { format!("，已同步 {synced} 个会话") } else { String::new() }
        ),
        payload: json!({
            "mode": mode,
            "changed": changed,
            "sessionFilesUpdated": sync.changed_session_files,
            "sqliteRowsUpdated": sync.sqlite_rows_updated,
        }),
    }
}

#[tauri::command]
pub fn restart_codex(mode: String, request: LaunchRequest) -> CommandResult<Value> {
    codexmate_core::watcher::stop_codex_processes();
    codexmate_core::watcher::stop_launcher_processes();
    std::thread::sleep(std::time::Duration::from_millis(800));

    if mode == "direct" {
        let app_path = if request.app_path.trim().is_empty() {
            "/Applications/Codex.app".to_string()
        } else {
            request.app_path.trim().to_string()
        };
        match std::process::Command::new("open").arg("-a").arg(&app_path).spawn() {
            Ok(_) => CommandResult {
                status: "ok".to_string(),
                message: "Codex 已启动（直连模式）".to_string(),
                payload: json!({"mode": "direct"}),
            },
            Err(e) => CommandResult {
                status: "failed".to_string(),
                message: format!("启动 Codex 失败: {e}"),
                payload: json!({"mode": "direct"}),
            },
        }
    } else {
        spawn_codexmate_launch(request, "Codex 已启动（代理模式·CDP注入）")
    }
}

#[tauri::command]
pub async fn test_smart_provider(
    mut provider: codexmate_core::router::SmartProvider,
) -> CommandResult<RelayProfileTestPayload> {
    let base_url = provider.base_url.trim().to_string();
    // 如果前端传来的是脱敏 key，从磁盘配置恢复真实 key
    let config_path_ref = codexmate_core::paths::default_app_state_dir().join("routing.toml");
    if let Ok(raw) = std::fs::read_to_string(&config_path_ref) {
        if let Ok(stored) = toml::from_str::<codexmate_core::router::SmartRouterConfig>(&raw) {
            if let Some(existing) = stored.providers.iter().find(|p| p.id == provider.id) {
                if provider.api_key == codexmate_core::router::api_key_masked_str(&existing.api_key)
                {
                    provider.api_key = existing.api_key.clone();
                }
            }
        }
    }
    let api_key = provider.api_key.trim().to_string();
    if base_url.is_empty() {
        return CommandResult {
            status: "failed".to_string(),
            message: "Base URL 不能为空".to_string(),
            payload: RelayProfileTestPayload {
                http_status: 0,
                endpoint: String::new(),
                response_preview: String::new(),
            },
        };
    }
    let test_url =
        codexmate_core::protocol_proxy::models_url_with(&base_url, provider.use_full_url);
    let client = reqwest::Client::new();
    match client
        .get(&test_url)
        .bearer_auth(&api_key)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            let preview = if body.len() > 500 {
                format!("{}...", &body[..500])
            } else {
                body
            };
            CommandResult {
                status: if (200..400).contains(&status) {
                    "ok"
                } else {
                    "failed"
                }
                .to_string(),
                message: format!(
                    "HTTP {} - {}",
                    status,
                    if (200..400).contains(&status) {
                        "连接成功"
                    } else {
                        "连接失败"
                    }
                ),
                payload: RelayProfileTestPayload {
                    http_status: status,
                    endpoint: test_url,
                    response_preview: preview,
                },
            }
        }
        Err(e) => CommandResult {
            status: "failed".to_string(),
            message: format!("连接失败: {}", e),
            payload: RelayProfileTestPayload {
                http_status: 0,
                endpoint: test_url,
                response_preview: String::new(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_version_returns_structured_payload() {
        let result = backend_version();

        assert_eq!(result.status, "ok");
        assert!(!result.payload.version.is_empty());
    }

    #[test]
    fn startup_options_returns_structured_payload() {
        let result = startup_options();

        assert_eq!(result.status, "ok");
    }

    #[test]
    fn startup_options_honors_show_update_environment() {
        unsafe {
            std::env::set_var("CODEXMATE_SHOW_UPDATE", "1");
        }

        let result = startup_options();

        unsafe {
            std::env::remove_var("CODEXMATE_SHOW_UPDATE");
        }

        assert_eq!(result.status, "ok");
        assert!(result.payload.show_update);
    }

    #[test]
    fn startup_options_honors_show_update_argument() {
        assert!(should_show_update(
            ["codexmate-manager.exe", "--show-update"],
            None
        ));
    }

    #[test]
    fn overview_contains_expected_operational_fields() {
        let result = tauri::async_runtime::block_on(load_overview());

        assert_eq!(result.status, "ok");
        assert!(!result.payload.current_version.is_empty());
        assert!(
            result.payload.codex_version.is_none()
                || result
                    .payload
                    .codex_version
                    .as_deref()
                    .is_some_and(|version| !version.is_empty())
        );
        assert!(matches!(
            result.payload.codex_app.status.as_str(),
            "found" | "missing"
        ));
        assert!(matches!(
            result.payload.silent_shortcut.status.as_str(),
            "installed" | "missing"
        ));
    }

    #[test]
    fn update_install_requires_release_payload() {
        let result = tauri::async_runtime::block_on(perform_update(None));

        assert_eq!(result.status, "failed");
        assert!(result.message.contains("请先检查更新"));
    }

    #[test]
    fn missing_logs_return_failed_status() {
        let result = read_latest_logs(LogRequest { lines: 25 });

        if result.payload.text.is_empty() {
            assert_eq!(result.status, "failed");
        }
    }

    #[test]
    fn open_external_url_rejects_non_http_urls() {
        let result = open_external_url("file:///C:/Windows/win.ini".to_string());

        assert_eq!(result.status, "failed");
        assert!(result.message.contains("只允许打开 http 或 https 链接"));
    }

    #[test]
    fn sync_all_session_providers_updates_rollouts_at_session_root() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join("config.toml"),
            "model_provider = \"custom\"\n",
        )
        .unwrap();
        let archived = temp.path().join("archived_sessions");
        std::fs::create_dir_all(&archived).unwrap();
        let rollout = archived.join("rollout-direct.jsonl");
        std::fs::write(
            &rollout,
            concat!(
                "{\"type\":\"session_meta\",\"payload\":{\"id\":\"thread-1\",\"model_provider\":\"openai\"}}\n",
                "{\"type\":\"event_msg\",\"payload\":{\"type\":\"user_message\"}}\n"
            ),
        )
        .unwrap();
        let db = rusqlite::Connection::open(temp.path().join("state_5.sqlite")).unwrap();
        db.execute(
            "CREATE TABLE threads (id TEXT PRIMARY KEY, model_provider TEXT, archived INTEGER, has_user_event INTEGER, cwd TEXT)",
            [],
        )
        .unwrap();
        db.execute(
            "INSERT INTO threads VALUES ('thread-1', 'openai', 1, 0, '')",
            [],
        )
        .unwrap();
        drop(db);

        let synced = sync_all_session_providers(temp.path());

        assert_eq!(synced.status, codexmate_data::ProviderSyncStatus::Synced);
        assert_eq!(synced.changed_session_files, 1);
        assert_eq!(synced.sqlite_rows_updated, 1);
        let contents = std::fs::read_to_string(rollout).unwrap();
        let first = contents.lines().next().unwrap();
        let record: Value = serde_json::from_str(first).unwrap();
        assert_eq!(record["payload"]["model_provider"], "custom");
        let db = rusqlite::Connection::open(temp.path().join("state_5.sqlite")).unwrap();
        let provider: String = db
            .query_row(
                "SELECT model_provider FROM threads WHERE id = 'thread-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(provider, "custom");
    }
}
