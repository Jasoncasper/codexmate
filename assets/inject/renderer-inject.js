// CodexMate renderer inject — model injection logic from CodexMate
(function () {
  var helperBase = window.__CODEX_SESSION_DELETE_HELPER__ || "http://127.0.0.1:57321";
  var pluginVersion = window.__CODEXMATE_VERSION__ || "unknown";
  var codexmateBuild = window.__CODEXMATE_BUILD__ || "unknown";
  var settingsKey = "codexmateSettings";
  var styleId = "codexmate-style";

  // ===== Settings =====
  function defaultSettings() {
    return { modelWhitelistUnlock: true };
  }
  function codexmateSettings() {
    try { return Object.assign(defaultSettings(), JSON.parse(localStorage.getItem(settingsKey) || "{}")); } catch (_) { return defaultSettings(); }
  }
  function modelUnlockEnabled() { return !!codexmateSettings().modelWhitelistUnlock; }

  // ===== Model catalog =====
  var modelCatalog = { status: "loading", model: "", default_model: "", model_provider: "", provider_name: "", models: [], sources: [], responses_api: { status: "unknown", message: "" } };
  var modelCatalogLoadedAt = 0;
  var modelCatalogPromise = null;
  var modelListRequestIds = new Set();
  var appServerPatchVer = "1";

  function uniqueValues(values) {
    var result = []; var seen = {};
    for (var i = 0; i < values.length; i++) {
      var v = values[i];
      if (typeof v === "string" && v.trim().length > 0 && !seen[v]) { seen[v] = true; result.push(v); }
    }
    return result;
  }

  function modelNames() {
    return uniqueValues([modelCatalog.default_model, modelCatalog.model].concat(Array.isArray(modelCatalog.models) ? modelCatalog.models : []));
  }

  function loadModelCatalog(force) {
    if (!force && modelCatalogPromise) return modelCatalogPromise;
    if (!force && modelCatalogLoadedAt && Date.now() - modelCatalogLoadedAt < 10000) return Promise.resolve(modelCatalog);
    modelCatalogPromise = postJson("/codex-model-catalog", {}).then(function (result) {
      modelCatalog = result && typeof result === "object" ? result : { status: "failed", model: "", default_model: "", model_provider: "", provider_name: "", models: [], sources: [], responses_api: { status: "unknown", message: "" } };
      modelCatalogLoadedAt = Date.now();
      modelCatalogPromise = null;
      patchWhitelist();
      return modelCatalog;
    }).catch(function () {
      modelCatalog = { status: "failed", model: "", default_model: "", model_provider: "", provider_name: "", models: [], sources: [], responses_api: { status: "unknown", message: "" } };
      modelCatalogLoadedAt = Date.now();
      modelCatalogPromise = null;
      return modelCatalog;
    });
    return modelCatalogPromise;
  }

  function modelDescriptor(name) {
    var shortName = name.indexOf(":") > 0 ? name.split(":")[0] : name;
    return {
      model: name, id: name, slug: name, name: shortName, displayName: shortName,
      description: modelCatalog.provider_name || modelCatalog.model_provider || "Custom model",
      hidden: false,
      isDefault: (modelCatalog.default_model || modelCatalog.model) === name,
      defaultReasoningEffort: "medium",
      supportedReasoningEfforts: ["minimal","low","medium","high","xhigh"].map(function (e) { return { reasoningEffort: e, description: e + " effort" }; })
    };
  }

  // ===== Model patching =====
  function modelArrayLookPatchable(value, allowEmpty) {
    return Array.isArray(value) && (allowEmpty || value.length > 0) && value.every(function (item) { return item && typeof item === "object" && typeof item.model === "string"; });
  }
  function stringArrayLookPatchable(value) {
    return Array.isArray(value) && value.every(function (item) { return typeof item === "string"; });
  }

  function patchModelArray(arr, allowEmpty) {
    if (!modelArrayLookPatchable(arr, allowEmpty)) return false;
    var names = modelNames();
    if (!names.length) return false;
    var changed = false;
    var existing = {};
    for (var i = 0; i < arr.length; i++) { if (arr[i] && arr[i].model) existing[arr[i].model] = true; }
    // 代理模式：隐藏 GPT 模型
    var inProxy = !!modelCatalog.model_provider;
    for (var i = 0; i < arr.length; i++) {
      if (arr[i] && arr[i].model) {
        var isGpt = /^(gpt|o[1-9]|codex-)/.test(arr[i].model);
        if (inProxy && isGpt) { arr[i].hidden = true; changed = true; }
        else if (names.indexOf(arr[i].model) >= 0 && arr[i].hidden !== false) { arr[i].hidden = false; changed = true; }
      }
    }
    for (var i = 0; i < names.length; i++) {
      if (!existing[names[i]]) { arr.push(modelDescriptor(names[i])); changed = true; }
    }
    return changed;
  }

  function patchModelNameArray(arr) {
    if (!stringArrayLookPatchable(arr)) return false;
    var names = modelNames();
    if (!names.length) return false;
    var changed = false;
    for (var i = 0; i < names.length; i++) { if (arr.indexOf(names[i]) < 0) { arr.push(names[i]); changed = true; } }
    return changed;
  }

  function patchModelContainer(value) {
    if (!value || typeof value !== "object") return false;
    var changed = false;
    if (patchModelArray(value.models, "defaultModel" in value || "availableModels" in value)) changed = true;
    if (patchModelNameArray(value.models)) changed = true;
    if (patchModelArray(value.data)) changed = true;
    if (patchModelArray(value.result)) changed = true;
    if (value.pages && value.pages[0] && patchModelArray(value.pages[0].data)) changed = true;
    if (value.result && patchModelArray(value.result.data)) changed = true;
    if (value.result && patchModelArray(value.result.models)) changed = true;
    if (value.message && value.message.result && patchModelArray(value.message.result.data)) changed = true;
    if (value.message && value.message.result && patchModelArray(value.message.result.models)) changed = true;
    var names = modelNames();
    ["availableModels","available_models"].forEach(function (key) {
      if (value[key] instanceof Set) { names.forEach(function (n) { value[key].add(n); }); changed = true; }
      if (Array.isArray(value[key])) { names.forEach(function (n) { if (value[key].indexOf(n) < 0) value[key].push(n); }); changed = true; }
    });
    ["hiddenModels","hidden_models"].forEach(function (key) {
      if (Array.isArray(value[key])) { var before = value[key].length; value[key] = value[key].filter(function (n) { return names.indexOf(n) < 0; }); if (value[key].length !== before) changed = true; }
    });
    if (value.defaultModel == null && names.length > 0) { value.defaultModel = modelDescriptor(names[0]); changed = true; }
    else if (typeof value.defaultModel === "string" && names.indexOf(value.defaultModel) >= 0 && value.model == null) { value.model = value.defaultModel; changed = true; }
    return changed;
  }

  // ===== JSON Response patch =====
  function installJsonPatch() {
    if (window.__codexmateJsonPatchInstalled) return;
    window.__codexmateJsonPatchInstalled = true;
    var originalJson = Response.prototype.json;
    if (typeof originalJson !== "function") return;
    Response.prototype.json = async function patchedJson() {
      var payload = await originalJson.apply(this, arguments);
      if (!modelUnlockEnabled()) return payload;
      if (!modelNames().length) await loadModelCatalog();
      if (!payload || typeof payload !== "object") return payload;
      try { patchModelContainer(payload); patchObjectGraph(payload, new WeakSet(), 0); } catch (_) {}
      return payload;
    };
  }

  // ===== dispatchEvent / message patch =====
  function installMsgPatch() {
    if (window.__codexmateMsgPatchInstalled) return;
    window.__codexmateMsgPatchInstalled = true;
    var originalDispatch = window.dispatchEvent;
    window.dispatchEvent = function patchedDispatch(event) {
      try {
        var detail = event && event.detail;
        var request = detail && detail.request;
        if (event && event.type === "codex-message-from-view" && detail && detail.type === "mcp-request" && request && request.method === "model/list") {
          request.params = Object.assign({}, request.params || {}, { includeHidden: true });
          if (request.id != null) modelListRequestIds.add(String(request.id));
        }
        if (event && event.type === "message") patchMsgData(event.data);
      } catch (_) {}
      return originalDispatch.call(this, event);
    };
    window.addEventListener("message", function (event) { try { patchMsgData(event.data); } catch (_) {} }, true);
  }

  function patchMsgData(data) {
    if (!data || data.type !== "mcp-response") return false;
    var message = data.message || data.response;
    var requestId = message && message.id != null ? String(message.id) : "";
    if (modelListRequestIds.size > 0 && requestId && !modelListRequestIds.has(requestId)) return false;
    modelListRequestIds.delete(requestId);
    return patchModelContainer(data) || patchModelContainer(message) || patchModelContainer(message && message.result) || patchModelContainer(message && message.result && message.result.data);
  }

  
// ===== App server request patch =====
  function appServerModelRequestMethod(input) {
    var url = typeof input === "string" ? input : input && input.url;
    if (typeof url !== "string") return "";
    return url ? "responses" : "";
  }

  function isManagedScopedModel(model) {
    return typeof model === "string" && model.indexOf(":") > 0;
  }

  function findScopedModelPayload(value, depth) {
    if (!value || typeof value !== "object" || depth > 6) return null;
    if (isManagedScopedModel(value.model)) return value;
    if (Array.isArray(value)) {
      for (var i = 0; i < value.length; i++) {
        var arrayMatch = findScopedModelPayload(value[i], depth + 1);
        if (arrayMatch) return arrayMatch;
      }
      return null;
    }
    var keys = Object.keys(value);
    for (var j = 0; j < keys.length; j++) {
      var child = value[keys[j]];
      var objectMatch = findScopedModelPayload(child, depth + 1);
      if (objectMatch) return objectMatch;
    }
    return null;
  }

  async function requestBodyJson(input, init) {
    var body = init && init.body;
    if ((typeof body !== "string" || !body.trim()) && input && typeof input.clone === "function") {
      try { body = await input.clone().text(); } catch (_) {}
    }
    if ((typeof body !== "string" || !body.trim()) && input && typeof input.body === "string") {
      body = input.body;
    }
    if (typeof body !== "string" || !body.trim()) return null;
    try { return JSON.parse(body); } catch (_) { return null; }
  }

  function routedRequestInit(input, init, payload) {
    var nextInit = Object.assign({}, init || {}, { body: JSON.stringify(payload) });
    if (input && typeof input === "object") {
      if (nextInit.method == null && input.method) nextInit.method = input.method;
      if (nextInit.headers == null && input.headers) nextInit.headers = input.headers;
    }
    return nextInit;
  }

  function routedHelperResponsesUrl() {
    return helperBase.replace(/\/$/, "") + "/v1/responses";
  }

  function installAppServerModelRequestPatch() {
    if (window.__codexmateAppServerModelRequestPatched) return;
    if (typeof window.fetch !== "function") return;
    window.__codexmateAppServerModelRequestPatched = appServerPatchVer;
    var originalFetch = window.fetch.bind(window);
    window.fetch = async function patchedFetch(input, init) {
      try {
        if (!modelUnlockEnabled() || appServerModelRequestMethod(input) !== "responses") {
          return originalFetch(input, init);
        }
        if (!modelNames().length) await loadModelCatalog();
        var payload = await requestBodyJson(input, init);
        var routedPayload = findScopedModelPayload(payload, 0);
        if (!routedPayload) {
          return originalFetch(input, init);
        }
        var nextInit = routedRequestInit(input, init, routedPayload);
        return originalFetch(routedHelperResponsesUrl(), nextInit);
      } catch (_) {
        return originalFetch(input, init);
      }
    };
  }

  function installXhrModelRequestPatch() {
    if (window.__codexmateXhrModelRequestPatched) return;
    if (typeof window.XMLHttpRequest !== "function") return;
    window.__codexmateXhrModelRequestPatched = appServerPatchVer;
    var OriginalXhr = window.XMLHttpRequest;
    window.XMLHttpRequest = function PatchedXMLHttpRequest() {
      var xhr = new OriginalXhr();
      var method = "POST";
      var url = "";
      var originalOpen = xhr.open;
      var originalSend = xhr.send;
      xhr.open = function patchedOpen(nextMethod, nextUrl) {
        method = nextMethod || method;
        url = nextUrl || "";
        return originalOpen.apply(xhr, arguments);
      };
      xhr.send = function patchedSend(body) {
        try {
          if (modelUnlockEnabled() && typeof body === "string" && body.trim()) {
            var payload = JSON.parse(body);
            var routedPayload = findScopedModelPayload(payload, 0);
            if (routedPayload) {
              originalOpen.call(xhr, method || "POST", routedHelperResponsesUrl(), true);
              return originalSend.call(xhr, JSON.stringify(routedPayload));
            }
          }
        } catch (_) {}
        return originalSend.apply(xhr, arguments);
      };
      return xhr;
    };
  }


  // ===== Statsig patch =====
  function statsigClients() {
    var root = window.__STATSIG__ || globalThis.__STATSIG__;
    if (!root || typeof root !== "object") return [];
    var clients = [];
    if (root.firstInstance) clients.push(root.firstInstance);
    try { if (typeof root.instance === "function") clients.push(root.instance()); } catch (_) {}
    if (root.instances && typeof root.instances === "object") { Object.keys(root.instances).forEach(function (k) { clients.push(root.instances[k]); }); }
    return clients.filter(function (c, i, a) { return c && typeof c === "object" && a.indexOf(c) === i; });
  }

  function patchStatsig() {
    var names = modelNames();
    if (!names.length) return;
    statsigClients().forEach(function (client) {
      if (typeof client.getDynamicConfig !== "function") return;
      if (!client.__codexmateStatsigPatched) {
        var original = client.getDynamicConfig.bind(client);
        client.getDynamicConfig = function (name, opts) { return patchStatsigConfig(original(name, opts)); };
        client.__codexmateStatsigPatched = true;
      }
      try { patchStatsigConfig(client.getDynamicConfig("107580212", { disableExposureLog: true })); } catch (_) {}
    });
  }

  function patchStatsigConfig(config) {
    var value = config && config.value;
    if (!value || typeof value !== "object") return config;
    var changed = false;
    if (Array.isArray(value.available_models)) {
      modelNames().forEach(function (n) { if (value.available_models.indexOf(n) < 0) { value.available_models.push(n); changed = true; } });
    }
    if (!changed && value.default_model === (modelNames()[0] || value.default_model)) return config;
    var next = Object.assign({}, value, { available_models: value.available_models, default_model: modelNames()[0] || value.default_model });
    try { config.value = next; } catch (_) { return Object.assign({}, config, { value: next }); }
    return config;
  }

  // ===== React fiber walk =====
  function patchReactState() {
    var visited = new WeakSet();
    var nodes = [document.body];
    var sel = document.querySelectorAll("button, [role='menu'], [role='dialog'], [data-radix-popper-content-wrapper]");
    for (var i = 0; i < sel.length; i++) nodes.push(sel[i]);
    var changed = false;
    for (var i = 0; i < nodes.length && i < 220; i++) {
      var node = nodes[i];
      var keys = Object.keys(node).filter(function (k) { return k.indexOf("__reactFiber") === 0 || k.indexOf("__reactInternalInstance") === 0 || k.indexOf("__reactProps") === 0; });
      for (var j = 0; j < keys.length; j++) { var val = node[keys[j]]; if (val && patchObjectGraph(val, visited, 0)) changed = true; }
    }
    return changed;
  }

  function patchObjectGraph(root, visited, depth) {
    if (!root || typeof root !== "object" || visited.has(root) || depth > 5) return false;
    visited.add(root);
    var changed = patchModelContainer(root);
    if (root instanceof Element || root === window || root === document || root === document.body || root === document.documentElement) return changed;
    var keys = Object.keys(root);
    for (var i = 0; i < keys.length; i++) {
      var key = keys[i];
      if (key === "ownerDocument" || key === "parentElement" || key === "parentNode" || key === "children" || key === "childNodes") continue;
      try { var val = root[key]; } catch (_) { continue; }
      if (val && typeof val === "object" && patchObjectGraph(val, visited, depth + 1)) changed = true;
    }
    return changed;
  }

  // ===== postJson =====
  function postJson(path, payload) {
    var bridge = window.__codexSessionDeleteBridge;
    if (typeof bridge !== "function") return Promise.resolve({ status: "failed", message: "Bridge not available" });
    return bridge(path, payload || {}).catch(function (err) { return { status: "failed", message: String(err) }; });
  }

  // ===== Diagnostics =====
  function sendDiag(event, detail) {
    var bridge = window.__codexSessionDeleteBridge;
    if (typeof bridge === "function") { bridge("/diagnostics/log", { event: event, detail: detail || {} }).catch(function () {}); }
  }

  // ===== patchWhitelist (CodexMate flow) =====
  function patchWhitelist() {
    if (!modelUnlockEnabled()) return;
    installJsonPatch();
    installAppServerModelRequestPatch();
    installXhrModelRequestPatch();
    installMsgPatch();
    // 直连模式不注入代理模型到白名单
    if (!modelCatalog.model_provider) return;
    if (!modelNames().length) { loadModelCatalog(); return; }
    patchStatsig();
    patchReactState();
  }

  // ===== Bridge helpers =====
  function checkBackendStatus() { return postJson("/backend/status", {}); }
  function openManager() { return postJson("/manager/open", {}); }

  // ===== CSS =====
  function injectStyles() {
    if (document.getElementById(styleId)) return;
    try {
      var style = document.createElement("style");
      style.id = styleId;
      style.textContent = [
        ".codex-backend-status { position: fixed; bottom: 8px; right: 8px; display: flex; align-items: center; gap: 6px; z-index: 99999; cursor: pointer; padding: 4px 8px; border-radius: 6px; background: rgba(0,0,0,0.3); backdrop-filter: blur(4px); }",
        ".codex-backend-dot { width: 8px; height: 8px; border-radius: 50%; transition: background 0.3s; }",
        ".codex-backend-dot.ok { background: #4ade80; box-shadow: 0 0 6px rgba(74,222,128,0.6); }",
        ".codex-backend-dot.failed { background: #f87171; box-shadow: 0 0 6px rgba(248,113,113,0.6); }",
        ".codex-backend-label { font-size: 10px; color: #888; user-select: none; }"
      ].join("\n");
      if (document.head) document.head.appendChild(style);
    } catch (_) {}
  }

  // ===== Status indicator =====
  function addStatus() {
    if (document.querySelector(".codex-backend-status")) return;
    try {
      if (!document.body) return;
      var el = document.createElement("div");
      el.className = "codex-backend-status";
      el.title = "CodexMate: Connecting...";
      el.addEventListener("click", function () { openManager().catch(function () {}); });
      var dot = document.createElement("span");
      dot.className = "codex-backend-dot";
      var label = document.createElement("span");
      label.className = "codex-backend-label";
      label.textContent = "连接中...";
      el.appendChild(dot);
      el.appendChild(label);
      document.body.appendChild(el);
      function updateModeLabel() {
        loadModelCatalog().then(function () {
          label.textContent = modelCatalog.model_provider ? "代理模式" : "直连模式";
        }).catch(function () {});
      }
      function poll() {
        checkBackendStatus().then(function (result) {
          dot.className = "codex-backend-dot " + (result.status === "ok" ? "ok" : "failed");
          el.title = result.status === "ok" ? "CodexMate: Connected" : "CodexMate: Disconnected";
          updateModeLabel();
        }).catch(function () { dot.className = "codex-backend-dot failed"; el.title = "CodexMate: Error"; });
      }
      poll();
      setInterval(poll, 15000);
    } catch (_) {}
  }

  // ===== Scan system =====
  function scanLightweight() { injectStyles(); addStatus(); }
  function scanDeferred() {
    patchWhitelist();
    setInterval(function () { if (modelNames().length) { patchStatsig(); patchReactState(); } }, 3000);
  }

  function runSafe(fn) { try { fn(); } catch (_) {} }

  function scan() {
    runSafe(scanLightweight);
    requestAnimationFrame(function () { runSafe(scanDeferred); });
  }

  // ===== Init =====
  scan();
  sendDiag("script_loaded", { version: pluginVersion, build: codexmateBuild });
})();
