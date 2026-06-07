# English Promotion Copy

> Channels: Twitter/X · Hacker News (Show HN) · Reddit (r/rust, r/opensource)

---

## Twitter/X 推文

### Short version (280 chars)

> Codex CLI is great, but managing multiple model providers manually? Not so much.
>
> I built CodexMate — a local desktop companion for Codex with visual routing, auto failover, CDP injection, and diagnostics.
>
> Open source, MIT, Rust + Tauri 2.
>
> 🔗 https://github.com/Jasoncasper/CodexMate

### Thread version (recommended)

**Tweet 1:**
> I've been using @OpenAI Codex CLI daily. One thing that kept bugging me: switching between OpenAI / DeepSeek / local models meant editing env vars every time.
>
> So I built a desktop companion for it. 🧵

**Tweet 2:**
> Meet CodexMate — a local enhancement manager for Codex CLI. Not a replacement, a companion.
>
> Built with Rust + Tauri 2. Runs on macOS 12+.
>
> Open source (MIT): https://github.com/Jasoncasper/CodexMate

**Tweet 3:**
> ✨ What it does:
>
> 🎯 GUI provider management — add OpenAI, DeepSeek, local models visually
> ⚡ Smart routing & fallback — configure failover strategies without scripting
> 🔄 Auto protocol adaptation — Responses API ↔ Chat Completions
> 💉 One-click CDP injection — enhance Codex without modifying it
> 📊 Visual diagnostics — logs & reports at a glance

**Tweet 4:**
> The comparison that matters:
>
> Before: edit .env → restart → pray
> After: point and click → done
>
> Before: manual error handling → write fallback scripts
> After: visual fallback tree → auto failover

**Tweet 5:**
> Tech stack: Rust, Tauri 2, React. Tiny bundle, low memory, fast startup.
>
> Everything stays local — no telemetry, no cloud.
>
> ⭐ Star on GitHub: https://github.com/Jasoncasper/CodexMate
> 💬 PRs & issues welcome!

### With screenshot

> [screenshot-main.png]
>
> Just shipped CodexMate — a local desktop manager for Codex CLI.
>
> Visual routing, multi-provider management, CDP injection, log diagnostics.
>
> Open source, MIT. Rust + Tauri 2.
>
> https://github.com/Jasoncasper/CodexMate

---

## Hacker News (Show HN)

**Title:**
> Show HN: CodexMate – A local desktop companion for OpenAI Codex CLI

**Body:**

Hey HN,

I've been using Codex CLI extensively and kept running into the same pain: managing multiple model providers (OpenAI, DeepSeek, local models) is a manual mess of env vars and scripts.

So I built CodexMate – a local enhancement manager that sits alongside Codex CLI.

**What it does:**

- **Multi-provider management** – Add/switch between OpenAI, DeepSeek, local models from a GUI
- **Smart routing & fallback** – Visually configure failover strategies instead of writing scripts
- **Auto protocol adaptation** – Bridges Responses API and Chat Completions automatically
- **CDP injection** – One-click injection to extend Codex without modifying the app itself
- **Visual diagnostics** – Log viewer and diagnostic report export

**Why not just use scripts?**

You can, and I did for a while. But as you add more providers, fallback rules, and protocol variants, the complexity grows fast. A visual tool makes it manageable — especially when you're in flow and just want to switch a model without context-switching to a terminal config.

**Tech stack:**

Rust + Tauri 2 + React. I chose Tauri over Electron because:
- Tiny bundle size (single-digit MB DMG)
- Low memory footprint
- Rust for the backend logic

**Current state:**

- macOS 12+ (DMG download from GitHub Releases)
- MIT licensed
- v1.0.0 — stable for daily use

**What's next:**

- More provider adapters
- Windows support is being explored
- VS Code extension integration

Would love to hear your feedback! https://github.com/Jasoncasper/CodexMate

---

## Reddit

### r/rust

**Title:**
> [Media] I built a desktop companion for Codex CLI with Rust + Tauri 2 — visual routing, multi-provider, CDP injection

**Body:**

Just shipped CodexMate, an open-source local desktop companion for OpenAI's Codex CLI.

**The problem:** Managing multiple model providers (OpenAI, DeepSeek, local models) with Codex CLI means editing env vars and writing fallback scripts by hand. It gets old fast.

**What I built:**
- GUI provider management — add/switch providers visually
- Smart routing with configurable fallback
- Auto protocol adaptation (Responses API ↔ Chat Completions)
- CDP injection for extending Codex capabilities
- Visual log diagnostics

**Tech:** Rust + Tauri 2 + React. Chose Tauri for the bundle size and Rust for the backend logic. Project is MIT licensed.

**Repo:** https://github.com/Jasoncasper/CodexMate

Happy to answer questions about the architecture, Tauri 2 migration experience, or anything else!

### r/opensource

**Title:**
> CodexMate — open-source desktop companion for OpenAI Codex CLI (Rust, Tauri 2, MIT)

**Body:**

CodexMate is a local enhancement manager for Codex CLI. Not a replacement — a companion that makes Codex easier to use day-to-day.

**Key features:**
- Visual multi-provider management (OpenAI, DeepSeek, local models, custom)
- Smart routing & automatic fallback strategies
- Protocol compatibility bridging
- One-click CDP injection
- Log diagnostics and report export

**Why open source?** Because developer tools should be transparent. No telemetry, no cloud dependency — everything stays on your machine.

Built with Rust + Tauri 2 + React. macOS 12+. MIT licensed.

GitHub: https://github.com/Jasoncasper/CodexMate

Would appreciate any feedback or contributions!
