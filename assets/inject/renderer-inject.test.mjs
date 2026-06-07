import assert from "node:assert/strict";
import fs from "node:fs";
import vm from "node:vm";

const script = fs.readFileSync(new URL("./renderer-inject.js", import.meta.url), "utf8");

function createElementStub() {
  return {
    appendChild() {},
    addEventListener() {},
    className: "",
    style: {},
    textContent: "",
    title: "",
  };
}

async function createHarness(catalog) {
  const fetchCalls = [];
  const xhrCalls = [];
  const bridgeCalls = [];
  const originalFetch = (input, init) => {
    fetchCalls.push({ input, init });
    return Promise.resolve({ ok: true });
  };
  function Response() {}
  Response.prototype.json = async function json() {
    return {};
  };
  class Request {
    constructor(url, init = {}) {
      this.url = url;
      this.method = init.method || "GET";
      this.headers = init.headers;
      this.body = init.body;
    }
    clone() {
      return new Request(this.url, {
        method: this.method,
        headers: this.headers,
        body: this.body,
      });
    }
    async text() {
      return typeof this.body === "string" ? this.body : "";
    }
  }
  class Element {}
  class XMLHttpRequest {
    open(method, url) {
      this.method = method;
      this.url = url;
      xhrCalls.push({ type: "open", method, url });
    }
    send(body) {
      xhrCalls.push({ type: "send", method: this.method, url: this.url, body });
    }
  }
  const sandbox = {
    Element,
    Request,
    Response,
    XMLHttpRequest,
    URL,
    fetch: originalFetch,
    globalThis: {},
    localStorage: { getItem: () => "{}" },
    setInterval() {},
    requestAnimationFrame: (fn) => fn(),
    document: {
      body: createElementStub(),
      documentElement: createElementStub(),
      head: createElementStub(),
      createElement: () => createElementStub(),
      getElementById: () => null,
      querySelector: () => null,
      querySelectorAll: () => [],
    },
    window: null,
  };
  sandbox.window = sandbox;
  sandbox.__codexSessionDeleteBridge = (path, payload) => {
    bridgeCalls.push({ path, payload });
    if (path === "/codex-model-catalog") return Promise.resolve(catalog);
    return Promise.resolve({ status: "ok" });
  };
  vm.runInNewContext(script, sandbox, { filename: "renderer-inject.js" });
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
  return { sandbox, fetchCalls, xhrCalls, bridgeCalls };
}

const official = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: ["deepseek-v4-pro:gpt-5.4-mini"],
  sources: [],
});
await official.sandbox.fetch("https://chatgpt.com/backend-api/responses", {
  body: JSON.stringify({ model: "gpt-5.4-mini", input: "hello" }),
});
assert.equal(
  official.fetchCalls.at(-1).input,
  "https://chatgpt.com/backend-api/responses",
  "official models should not be routed to the local helper",
);

const managed = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: ["deepseek-v4-pro:gpt-5.4-mini"],
  sources: [],
});
await managed.sandbox.fetch("https://chatgpt.com/backend-api/responses", {
  body: JSON.stringify({ model: "deepseek-v4-pro:gpt-5.4-mini", input: "hello" }),
});
assert.equal(
  managed.fetchCalls.at(-1).input,
  "http://127.0.0.1:57321/v1/responses",
  "managed third-party models should be routed to the local helper",
);

const managedRequest = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: ["deepseek-v4-flash:deepseek-v4-flash"],
  sources: [],
});
await managedRequest.sandbox.fetch(
  new managedRequest.sandbox.Request("https://chatgpt.com/backend-api/responses", {
    method: "POST",
    body: JSON.stringify({ model: "deepseek-v4-flash:deepseek-v4-flash", input: "hello" }),
  }),
);
assert.equal(
  managedRequest.fetchCalls.at(-1).input,
  "http://127.0.0.1:57321/v1/responses",
  "managed third-party Request objects should be routed to the local helper",
);

const managedBackendApi = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: ["deepseek-v4-flash:deepseek-v4-flash"],
  sources: [],
});
await managedBackendApi.sandbox.fetch("https://chatgpt.com/backend-api/codex/responses", {
  body: JSON.stringify({ model: "deepseek-v4-flash:deepseek-v4-flash", input: "hello" }),
});
assert.equal(
  managedBackendApi.fetchCalls.at(-1).input,
  "http://127.0.0.1:57321/v1/responses",
  "managed third-party backend API requests should be routed to the local helper",
);

const managedNestedPayload = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: [],
  sources: [],
});
await managedNestedPayload.sandbox.fetch("https://chatgpt.com/backend-api/codex/turn/start", {
  body: JSON.stringify({
    request: {
      model: "deepseek-v4-flash:deepseek-v4-flash",
      input: "hello",
      stream: true,
    },
  }),
});
assert.equal(
  managedNestedPayload.fetchCalls.at(-1).input,
  "http://127.0.0.1:57321/v1/responses",
  "nested managed third-party payloads should be routed to the local helper",
);
assert.equal(
  JSON.parse(managedNestedPayload.fetchCalls.at(-1).init.body).model,
  "deepseek-v4-flash:deepseek-v4-flash",
  "nested managed payload should be promoted to the helper request body",
);

const managedUnknownUrl = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: [],
  sources: [],
});
await managedUnknownUrl.sandbox.fetch("https://chatgpt.com/some-new-api/turn/start", {
  body: JSON.stringify({ model: "deepseek-v4-flash:deepseek-v4-flash", input: "hello" }),
});
assert.equal(
  managedUnknownUrl.fetchCalls.at(-1).input,
  "http://127.0.0.1:57321/v1/responses",
  "managed third-party requests should route regardless of endpoint path",
);

const managedXhr = await createHarness({
  status: "ok",
  model: "",
  default_model: "",
  model_provider: "codexmate",
  provider_name: "CodexMate",
  models: [],
  sources: [],
});
const xhr = new managedXhr.sandbox.XMLHttpRequest();
xhr.open("POST", "https://chatgpt.com/backend-api/codex/turn/start");
xhr.send(JSON.stringify({ model: "deepseek-v4-flash:deepseek-v4-flash", input: "hello" }));
assert.equal(
  managedXhr.xhrCalls.at(-2).url,
  "http://127.0.0.1:57321/v1/responses",
  "managed third-party XHR requests should be reopened against the local helper",
);
