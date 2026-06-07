use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use codexmate_core::launcher::{
    CodexLaunch, LaunchHooks, LaunchOptions, ProcessWaitStrategy, launch_and_inject_with_hooks,
};
use codexmate_core::session::{DeleteResult, DeleteStatus, ExportResult, ExportStatus, SessionRef};
use codexmate_core::routes::{
    BridgeContext, BridgeDataService, BridgeRuntimeService, BridgeSettingsService,
    CoreRuntimeService, handle_bridge_request,
};
use codexmate_core::settings::BackendSettings;
use codexmate_core::status::StatusStore;
use serde_json::{Value, json};

#[tokio::test]
async fn bridge_routes_cover_all_current_paths() {
    let ctx = test_context();

    let cases = [
        ("/settings/get", json!({})),
        ("/settings/set", json!({"enhancementsEnabled": true})),
        ("/devtools/open", json!({})),
        ("/manager/open", json!({})),
        ("/backend/status", json!({})),
        ("/backend/repair", json!({})),
        ("/codex-model-catalog", json!({})),
        ("/codex-config-model", json!({})),
        ("/zed-remote/status", json!({})),
        (
            "/zed-remote/resolve-host",
            json!({"hostId": "remote-ssh-codex-managed:remote"}),
        ),
        (
            "/zed-remote/fallback-request",
            json!({"hostId": "remote-ssh-codex-managed:remote"}),
        ),
        (
            "/zed-remote/open",
            json!({"ssh": {"host": "example.com"}, "path": "/home/app.py"}),
        ),
        ("/upstream-worktree/status", json!({})),
        ("/upstream-worktree/defaults", json!({"repoPath": "/repo"})),
        (
            "/upstream-worktree/prepare",
            json!({"repoPath": "/repo", "remote": "upstream", "baseBranch": "main"}),
        ),
        (
            "/upstream-worktree/create",
            json!({"repoPath": "/repo", "branchName": "feature/demo"}),
        ),
        ("/delete", json!({"session_id": "s1", "title": "First"})),
        ("/undo", json!({"undo_token": "undo-1"})),
        (
            "/export-markdown",
            json!({"session_id": "s1", "title": "First"}),
        ),
        ("/archived-thread", json!({"title": "Archived"})),
        (
            "/move-thread-workspace",
            json!({"session_id": "s1", "title": "First", "target_cwd": "/new"}),
        ),
        (
            "/thread-sort-key",
            json!({"session_id": "s1", "title": "First"}),
        ),
        (
            "/thread-sort-keys",
            json!({"sessions": [{"session_id": "s1", "title": "First"}]}),
        ),
    ];

    for (path, payload) in cases {
        let result = handle_bridge_request(ctx.clone(), path, payload).await;
        assert_ne!(
            result["message"], "Unknown bridge path",
            "{path} should be routed"
        );
    }
}

#[tokio::test]
async fn upstream_worktree_routes_are_dispatched_to_runtime() {
    let ctx = test_context();

    assert_eq!(
        handle_bridge_request(ctx.clone(), "/upstream-worktree/status", json!({})).await,
        json!({"status": "ok", "feature": "upstream-worktree"})
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/upstream-worktree/defaults",
            json!({"repoPath": "/repo"}),
        )
        .await,
        json!({
            "status": "ok",
            "repoRoot": "/repo",
            "defaultRemote": "upstream",
            "defaultBaseBranch": "main",
        })
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/upstream-worktree/create",
            json!({"repoPath": "/repo", "branchName": "feature/demo"}),
        )
        .await,
        json!({
            "status": "ok",
            "repoRoot": "/repo",
            "branchName": "feature/demo",
            "worktreePath": "/repo-feature-demo",
        })
    );
    assert_eq!(
        handle_bridge_request(
            ctx,
            "/upstream-worktree/prepare",
            json!({"repoPath": "/repo", "remote": "upstream", "baseBranch": "main"}),
        )
        .await,
        json!({
            "status": "ok",
            "repoRoot": "/repo",
            "sourceRef": "upstream/main",
            "qualifiedSourceRef": "refs/remotes/upstream/main",
        })
    );
}

#[tokio::test]
async fn unknown_bridge_path_preserves_empty_session_id_shape() {
    let result = handle_bridge_request(
        test_context(),
        "/missing",
        json!({"session_id": "should-not-leak"}),
    )
    .await;

    assert_eq!(
        result,
        json!({
            "status": "failed",
            "session_id": "",
            "message": "Unknown bridge path"
        })
    );
}

#[tokio::test]
async fn settings_routes_use_settings_service() {
    let ctx = test_context();

    let updated = handle_bridge_request(
        ctx.clone(),
        "/settings/set",
        json!({"enhancementsEnabled": true, "codexGoalsEnabled": false}),
    )
    .await;
    let loaded = handle_bridge_request(ctx, "/settings/get", json!({})).await;

    assert_eq!(updated["enhancementsEnabled"], true);
    assert_eq!(updated["codexGoalsEnabled"], false);
    assert_eq!(loaded, updated);
}

#[tokio::test]
async fn runtime_status_devtools_repair_and_zed_routes_are_dispatched() {
    let ctx = test_context();

    assert_eq!(
        handle_bridge_request(ctx.clone(), "/devtools/open", json!({})).await,
        json!({"status": "ok", "opened": true})
    );
    assert_eq!(
        handle_bridge_request(ctx.clone(), "/manager/open", json!({})).await,
        json!({"status": "ok", "opened": "manager"})
    );
    assert_eq!(
        handle_bridge_request(ctx.clone(), "/backend/status", json!({})).await,
        json!({"status": "ok", "message": "后端已连接", "version": codexmate_core::version::VERSION})
    );
    assert_eq!(
        handle_bridge_request(ctx.clone(), "/backend/repair", json!({})).await,
        json!({"status": "ok", "message": "后端已修复", "version": codexmate_core::version::VERSION})
    );
    assert_eq!(
        handle_bridge_request(ctx.clone(), "/zed-remote/status", json!({})).await,
        json!({"status": "ok", "platformSupported": true, "zedAppFound": true, "zedCliFound": false})
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/zed-remote/resolve-host",
            json!({"hostId": "remote-ssh-codex-managed:remote"}),
        )
        .await,
        json!({"status": "ok", "ssh": {"user": "longnv", "host": "192.168.100.31", "port": null}})
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/zed-remote/fallback-request",
            json!({"hostId": "remote-ssh-codex-managed:remote"}),
        )
        .await,
        json!({
            "status": "ok",
            "request": {
                "hostId": "remote-ssh-codex-managed:remote",
                "ssh": {"user": "longnv", "host": "192.168.100.31", "port": null},
                "path": "/Users/longnv/bin/repo/sealos-skills",
            }
        })
    );
    assert_eq!(
        handle_bridge_request(
            ctx,
            "/zed-remote/open",
            json!({"ssh": {"host": "example.com"}, "path": "/home/app.py"}),
        )
        .await,
        json!({"status": "ok", "url": "ssh://example.com/home/app.py"})
    );
}

#[tokio::test]
async fn data_routes_forward_payloads_to_data_service() {
    let ctx = test_context();

    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/delete",
            json!({"session_id": "s1", "title": "First"}),
        )
        .await["undo_token"],
        "undo-s1"
    );
    assert_eq!(
        handle_bridge_request(ctx.clone(), "/undo", json!({"undo_token": "undo-s1"})).await,
        json!({
            "status": "undone",
            "session_id": "s1",
            "message": "undone",
            "undo_token": "undo-s1",
            "backup_path": null
        })
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/export-markdown",
            json!({"session_id": "s1", "title": "First"}),
        )
        .await["filename"],
        "First.md"
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/archived-thread",
            json!({"title": "Archived"})
        )
        .await,
        json!({"session_id": "archived-1", "title": "Archived"})
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/move-thread-workspace",
            json!({"session_id": "s1", "title": "First", "target_cwd": "/new"}),
        )
        .await,
        json!({"status": "moved", "session_id": "s1", "target_cwd": "/new"})
    );
    assert_eq!(
        handle_bridge_request(
            ctx.clone(),
            "/thread-sort-key",
            json!({"session_id": "s1", "title": "First"}),
        )
        .await,
        json!({"status": "ok", "session_id": "s1", "updated_at": 123})
    );
    assert_eq!(
        handle_bridge_request(
            ctx,
            "/thread-sort-keys",
            json!({"sessions": [{"session_id": "s1", "title": "First"}, null, {"session_id": "s2"}]}),
        )
        .await,
        json!({"status": "ok", "sort_keys": [{"session_id": "s1"}, {"session_id": "s2"}]})
    );
}

#[tokio::test]
async fn bridge_context_core_with_data_uses_injected_data_service() {
    let ctx = BridgeContext::core_with_data(
        Arc::new(CoreRuntimeService::new(9229, StatusStore::default())),
        Arc::new(FakeData),
    );

    let result = handle_bridge_request(
        ctx,
        "/delete",
        json!({"session_id": "s1", "title": "First"}),
    )
    .await;

    assert_eq!(result["status"], "local_deleted");
    assert_eq!(result["undo_token"], "undo-s1");
    assert_ne!(
        result["message"],
        "Delete service is not wired in core launcher hooks"
    );
}

#[tokio::test]
async fn core_runtime_open_devtools_uses_inspector_url_opener() {
    let opened = Arc::new(Mutex::new(Vec::<String>::new()));
    let runtime = CoreRuntimeService::new(9229, StatusStore::default())
        .with_devtools_opener({
            let opened = opened.clone();
            Arc::new(move |url| {
                opened.lock().unwrap().push(url.to_string());
                Ok(())
            })
        })
        .with_devtools_target_id("page-1");
    let ctx = BridgeContext::core_with_data(Arc::new(runtime), Arc::new(FakeData));

    let result = handle_bridge_request(ctx, "/devtools/open", json!({})).await;

    assert_eq!(result["status"], "ok");
    assert_eq!(result["target_id"], "page-1");
    assert_eq!(
        opened.lock().unwrap().as_slice(),
        ["http://127.0.0.1:9229/devtools/inspector.html?ws=127.0.0.1:9229/devtools/page/page-1"]
    );
}

#[tokio::test]
async fn core_runtime_manager_route_attempts_to_open_manager_binary() {
    let ctx = BridgeContext::core(Arc::new(CoreRuntimeService::new(
        9229,
        StatusStore::default(),
    )));

    let result = handle_bridge_request(ctx, "/manager/open", json!({})).await;

    assert_ne!(result["message"], "管理工具启动未接入当前运行时");
}

#[tokio::test]
async fn bridge_backend_status_writes_diagnostic_log() {
    let temp = tempfile::tempdir().unwrap();
    let log_path = temp.path().join("codexmate.log");
    codexmate_core::diagnostic_log::set_diagnostic_log_path_for_tests(Some(log_path.clone()));
    let ctx = BridgeContext::core(Arc::new(CoreRuntimeService::new(
        9229,
        StatusStore::default(),
    )));

    let result = handle_bridge_request(ctx, "/backend/status", json!({})).await;

    assert_eq!(result["status"], "ok");
    let contents = std::fs::read_to_string(&log_path).unwrap();
    assert!(contents.contains("bridge.request"));
    assert!(contents.contains("bridge.backend_status_ok"));
    assert!(contents.contains("/backend/status"));
    codexmate_core::diagnostic_log::set_diagnostic_log_path_for_tests(None);
}

#[tokio::test]
async fn launch_lifecycle_uses_hook_supplied_bridge_context_for_injection() {
    let temp = tempfile::tempdir().unwrap();
    let app_dir = temp.path().join("Codex.app");
    std::fs::create_dir_all(&app_dir).unwrap();
    let events = Arc::new(Mutex::new(Vec::<String>::new()));
    let hooks = ContextHooks {
        events: events.clone(),
    };

    launch_and_inject_with_hooks(
        LaunchOptions {
            app_dir: Some(app_dir),
            debug_port: 9229,
            helper_port: 57321,
            ..LaunchOptions::default()
        },
        &hooks,
    )
    .await
    .unwrap();

    assert_eq!(
        *events.lock().unwrap(),
        vec![
            "bridge-context:9229",
            "inject-bridge:9229:57321",
            "watchdog:9229:57321",
            "status:running",
        ]
    );
}

fn test_context() -> BridgeContext {
    BridgeContext::new(
        Arc::new(FakeSettings::default()),
        Arc::new(FakeRuntime),
        Arc::new(FakeData),
    )
}

#[derive(Default)]
struct FakeSettings {
    settings: Mutex<BackendSettings>,
}

#[async_trait]
impl BridgeSettingsService for FakeSettings {
    async fn get_settings(&self) -> anyhow::Result<BackendSettings> {
        Ok(self.settings.lock().unwrap().clone())
    }

    async fn set_settings(&self, payload: Value) -> anyhow::Result<BackendSettings> {
        let current = self.settings.lock().unwrap().clone();
        let mut raw = serde_json::to_value(current).unwrap();
        let raw = raw.as_object_mut().unwrap();
        if let Some(value) = payload.get("enhancementsEnabled").and_then(Value::as_bool) {
            raw.insert("enhancementsEnabled".to_string(), json!(value));
        }
        if let Some(value) = payload.get("codexGoalsEnabled").and_then(Value::as_bool) {
            raw.insert("codexGoalsEnabled".to_string(), json!(value));
        }
        let updated: BackendSettings = serde_json::from_value(Value::Object(raw.clone())).unwrap();
        *self.settings.lock().unwrap() = updated.clone();
        Ok(updated)
    }
}

struct FakeRuntime;

#[async_trait]
impl BridgeRuntimeService for FakeRuntime {
    async fn open_devtools(&self) -> anyhow::Result<Value> {
        Ok(json!({"status": "ok", "opened": true}))
    }

    async fn open_manager(&self) -> anyhow::Result<Value> {
        Ok(json!({"status": "ok", "opened": "manager"}))
    }

    async fn backend_status(&self) -> anyhow::Result<Value> {
        Ok(
            json!({"status": "ok", "message": "后端已连接", "version": codexmate_core::version::VERSION}),
        )
    }

    async fn repair_backend(&self) -> anyhow::Result<Value> {
        Ok(
            json!({"status": "ok", "message": "后端已修复", "version": codexmate_core::version::VERSION}),
        )
    }

    async fn codex_model_catalog(&self) -> anyhow::Result<Value> {
        Ok(json!({
            "status": "ok",
            "model": "qwen3-coder",
            "default_model": "qwen3-coder",
            "model_provider": "relay",
            "provider_name": "Relay",
            "models": ["qwen3-coder"],
            "sources": []
        }))
    }

    async fn zed_remote_status(&self) -> anyhow::Result<Value> {
        Ok(json!({
            "status": "ok",
            "platformSupported": true,
            "zedAppFound": true,
            "zedCliFound": false
        }))
    }

    async fn resolve_zed_remote_host(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["hostId"], json!("remote-ssh-codex-managed:remote"));
        Ok(json!({
            "status": "ok",
            "ssh": {"user": "longnv", "host": "192.168.100.31", "port": null}
        }))
    }

    async fn fallback_zed_remote_request(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["hostId"], json!("remote-ssh-codex-managed:remote"));
        Ok(json!({
            "status": "ok",
            "request": {
                "hostId": "remote-ssh-codex-managed:remote",
                "ssh": {"user": "longnv", "host": "192.168.100.31", "port": null},
                "path": "/Users/longnv/bin/repo/sealos-skills",
            }
        }))
    }

    async fn open_zed_remote(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["path"], json!("/home/app.py"));
        Ok(json!({"status": "ok", "url": "ssh://example.com/home/app.py"}))
    }

    async fn upstream_worktree_status(&self) -> anyhow::Result<Value> {
        Ok(json!({"status": "ok", "feature": "upstream-worktree"}))
    }

    async fn upstream_worktree_defaults(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["repoPath"], json!("/repo"));
        Ok(json!({
            "status": "ok",
            "repoRoot": "/repo",
            "defaultRemote": "upstream",
            "defaultBaseBranch": "main",
        }))
    }

    async fn upstream_worktree_prepare(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["repoPath"], json!("/repo"));
        assert_eq!(payload["remote"], json!("upstream"));
        assert_eq!(payload["baseBranch"], json!("main"));
        Ok(json!({
            "status": "ok",
            "repoRoot": "/repo",
            "sourceRef": "upstream/main",
            "qualifiedSourceRef": "refs/remotes/upstream/main",
        }))
    }

    async fn upstream_worktree_create(&self, payload: Value) -> anyhow::Result<Value> {
        assert_eq!(payload["repoPath"], json!("/repo"));
        assert_eq!(payload["branchName"], json!("feature/demo"));
        Ok(json!({
            "status": "ok",
            "repoRoot": "/repo",
            "branchName": "feature/demo",
            "worktreePath": "/repo-feature-demo",
        }))
    }
}

struct FakeData;

#[async_trait]
impl BridgeDataService for FakeData {
    async fn delete(&self, session: SessionRef) -> anyhow::Result<DeleteResult> {
        Ok(DeleteResult {
            status: DeleteStatus::LocalDeleted,
            session_id: session.session_id.clone(),
            message: format!("deleted {}", session.title),
            undo_token: Some(format!("undo-{}", session.session_id)),
            backup_path: None,
        })
    }

    async fn undo(&self, undo_token: String) -> anyhow::Result<DeleteResult> {
        Ok(DeleteResult {
            status: DeleteStatus::Undone,
            session_id: "s1".to_string(),
            message: "undone".to_string(),
            undo_token: Some(undo_token),
            backup_path: None,
        })
    }

    async fn export_markdown(&self, session: SessionRef) -> anyhow::Result<ExportResult> {
        Ok(ExportResult {
            status: ExportStatus::Exported,
            session_id: session.session_id,
            message: "exported".to_string(),
            filename: Some("First.md".to_string()),
            markdown: Some("# First\n".to_string()),
        })
    }

    async fn find_archived_thread_by_title(
        &self,
        title: String,
    ) -> anyhow::Result<Option<SessionRef>> {
        Ok(Some(SessionRef {
            session_id: "archived-1".to_string(),
            title,
        }))
    }

    async fn move_thread_workspace(
        &self,
        session: SessionRef,
        target_cwd: String,
    ) -> anyhow::Result<Value> {
        Ok(json!({"status": "moved", "session_id": session.session_id, "target_cwd": target_cwd}))
    }

    async fn thread_sort_key(&self, session: SessionRef) -> anyhow::Result<Value> {
        Ok(json!({"status": "ok", "session_id": session.session_id, "updated_at": 123}))
    }

    async fn thread_sort_keys(&self, sessions: Vec<SessionRef>) -> anyhow::Result<Value> {
        Ok(json!({
            "status": "ok",
            "sort_keys": sessions
                .into_iter()
                .map(|session| json!({"session_id": session.session_id}))
                .collect::<Vec<_>>()
        }))
    }
}

#[derive(Clone)]
struct ContextHooks {
    events: Arc<Mutex<Vec<String>>>,
}

impl ContextHooks {
    fn event(&self, event: impl Into<String>) {
        self.events.lock().unwrap().push(event.into());
    }
}

#[async_trait(?Send)]
impl LaunchHooks for ContextHooks {
    fn resolve_app_dir(
        &self,
        app_dir: Option<&std::path::Path>,
        _settings: &BackendSettings,
    ) -> anyhow::Result<std::path::PathBuf> {
        app_dir
            .map(std::path::Path::to_path_buf)
            .ok_or_else(|| anyhow::anyhow!("missing app dir"))
    }

    fn select_debug_port(&self, requested: u16) -> u16 {
        requested
    }

    fn select_helper_port(&self, requested: u16) -> u16 {
        requested
    }

    async fn load_settings(&self) -> anyhow::Result<BackendSettings> {
        Ok(BackendSettings::default())
    }

    async fn run_provider_sync(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn start_helper(&self, _helper_port: u16) -> anyhow::Result<()> {
        Ok(())
    }

    async fn launch_codex(
        &self,
        _app_dir: &std::path::Path,
        _debug_port: u16,
        _extra_args: &[String],
    ) -> anyhow::Result<CodexLaunch> {
        Ok(CodexLaunch::Process {
            command: vec!["codex".to_string()],
            wait_strategy: ProcessWaitStrategy::TrackedChild,
            macos_cleanup_policy: None,
        })
    }

    async fn bridge_context(&self, debug_port: u16) -> anyhow::Result<Option<BridgeContext>> {
        self.event(format!("bridge-context:{debug_port}"));
        Ok(Some(test_context()))
    }

    async fn inject(&self, _debug_port: u16, _helper_port: u16) -> anyhow::Result<()> {
        anyhow::bail!("legacy inject should not run when bridge context is supplied")
    }

    async fn inject_bridge(
        &self,
        debug_port: u16,
        helper_port: u16,
        _ctx: BridgeContext,
    ) -> anyhow::Result<()> {
        self.event(format!("inject-bridge:{debug_port}:{helper_port}"));
        Ok(())
    }

    async fn start_bridge_watchdog(&self, debug_port: u16, helper_port: u16) -> anyhow::Result<()> {
        self.event(format!("watchdog:{debug_port}:{helper_port}"));
        Ok(())
    }

    async fn write_status(&self, status: &str) {
        self.event(format!("status:{status}"));
    }

    async fn wait_for_codex_exit(&self, _launch: &CodexLaunch) -> anyhow::Result<()> {
        Ok(())
    }

    async fn shutdown_helper(&self, _helper_port: u16) {}

    async fn terminate_codex(&self, _launch: &CodexLaunch) {}
}
