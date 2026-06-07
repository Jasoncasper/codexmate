<h1 align="center">CodexMate</h1>

<p align="center">
  <b>English</b> · <a href="./README.md">中文</a>
</p>

<p align="center">
  <a href="https://github.com/Jasoncasper/CodexMate/releases">
    <img src="https://img.shields.io/github/v/release/Jasoncasper/CodexMate?color=green&label=version" alt="Version">
  </a>
  <a href="https://github.com/Jasoncasper/CodexMate/actions/workflows/release.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/Jasoncasper/CodexMate/release.yml?branch=main&label=CI" alt="CI">
  </a>
  <img src="https://img.shields.io/badge/platform-macOS%2012%2B-lightgrey" alt="macOS">
  <img src="https://img.shields.io/badge/Rust-🦀-orange" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-2-ffc131" alt="Tauri 2">
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT License">
  </a>
  <a href="https://github.com/Jasoncasper/CodexMate/stargazers">
    <img src="https://img.shields.io/github/stars/Jasoncasper/CodexMate?style=social" alt="Stars">
  </a>
</p>

<p align="center">
  <b>A local enhancement desktop manager for OpenAI Codex CLI</b><br>
  GUI management · Smart routing · Multi-model aggregation · CDP injection · Diagnostics
</p>

<p align="center">
  <a href="https://github.com/Jasoncasper/CodexMate/releases">
    <img src="https://img.shields.io/badge/📥_Download_Latest-v1.0.0-green?style=for-the-badge" alt="Download">
  </a>
  <a href="#-building-from-source">
    <img src="https://img.shields.io/badge/🔧_Build_from_Source-Start-orange?style=for-the-badge" alt="Build">
  </a>
</p>

---

## 📸 Screenshots

<p align="center">
  <img src="assets/images/screenshot-main.png" alt="CodexMate Main UI" width="700">
  <br>
  <em>CodexMate main interface — quick launch and management hub</em>
</p>

<p align="center">
  <img src="assets/images/screenshot-routing.png" alt="Smart Routing" width="700">
  <br>
  <em>Smart routing — manage providers and model mappings visually</em>
</p>

<p align="center">
  <img src="assets/images/screenshot-about.png" alt="About Page" width="700">
  <br>
  <em>About — version info and update check</em>
</p>

---

## 🤔 Why CodexMate?

| Scenario | Manual Setup | CodexMate |
|----------|-------------|-----------|
| Using multiple model providers (OpenAI / DeepSeek / local models) | Edit env vars repeatedly, manage API keys by hand | 🎯 Add visually, unified management |
| Model routing & fallback | Write custom scripts | ⚡ Configure fallback strategies visually |
| Protocol compatibility (Responses API / Chat Completions) | Manually tweak request formats | 🔄 Auto protocol adaptation |
| Enhancing Codex (CDP injection) | Manual injection, no management UI | 💉 One-click injection & management |
| Runtime status & diagnostics | Dig through log files | 📊 Visual log viewer & diagnostic report export |
| Direct / proxy mode switching | Modify config and restart manually | 🔀 One-click toggle |

> CodexMate is **not** a replacement for Codex. It is a local companion management tool that makes the Codex experience smoother and more productive.

---

## ✨ Features

- 🧠 **Smart Routing** — Manage providers, model mappings, fallbacks, and multimodal fallback strategies through a visual interface
- 🔗 **Multi-Provider Aggregation** — Unify OpenAI, DeepSeek, local models, and more into a single model picker; switch freely
- 🔄 **Protocol Adapter** — Compatible with `Responses API`, `Chat Completions`, and third-party endpoints
- 💉 **CDP Injection** — Enhance Codex via CDP injection scripts without modifying the Codex app itself
- 🔀 **Mode Switching** — Toggle between direct and proxy modes and restart with one click
- 📋 **Logs & Diagnostics** — View runtime logs, export diagnostic reports, and troubleshoot quickly

---

## 📥 Installation

Download the latest `.dmg` from [Releases](https://github.com/Jasoncasper/CodexMate/releases):

```bash
# Or via Homebrew (coming soon)
# brew install codexmate
```

**Requirements**: macOS 12+

---

## 🚀 Quick Start

1. Download and install CodexMate
2. Launch the app — the management UI opens automatically
3. Add your model providers (OpenAI / DeepSeek / custom, etc.) on the **Routing** tab
4. Manage MCP servers, Skills, and Plugins on the **Context** tab
5. Click **Launch Codex** in the top bar to start using

---

## 🔧 Building from Source

### Prerequisites

- macOS 12+
- Rust stable (Rust 1.80+ minimum)
- Node.js 20+ & npm 10+
- Xcode Command Line Tools

### Steps

```bash
# 1. Clone the repository
git clone https://github.com/Jasoncasper/CodexMate.git
cd CodexMate

# 2. Install frontend dependencies and start dev mode
cd apps/codexmate-manager
npm install
npm run dev
```

### Common Checks

```bash
cd apps/codexmate-manager
npm run check                           # TypeScript type checking

cd /path/to/CodexMate
cargo check --workspace                 # Rust compilation check
cargo test -p codexmate-manager --lib   # Run tests
```

### Building a Release

```bash
cd apps/codexmate-manager
npm run build
```

Artifacts will be at:

- `target/release/bundle/macos/CodexMate.app`
- `target/release/bundle/dmg/CodexMate_<version>_<arch>.dmg`

---

## 📁 Repository Structure

```
CodexMate/
├── apps/
│   ├── codexmate-launcher/     # Background launcher (Rust)
│   └── codexmate-manager/      # Tauri management UI (React + Rust)
├── crates/
│   ├── codexmate-core/         # Core: routing, injection, update, diagnostics
│   └── codexmate-data/         # Data & export capabilities
├── assets/                     # Static assets (injection scripts, images, etc.)
├── scripts/                    # Build & release scripts
└── docs/                       # Documentation
```

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Jasoncasper/CodexMate&type=Date)](https://star-history.com/#Jasoncasper/CodexMate&Date)

---

## 🤝 Contributing

Issues and PRs are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).

We especially welcome contributions in:

- 🐛 Bug reports and fixes
- 🌐 New model provider adapters
- 📖 Documentation improvements
- 🎨 UI/UX suggestions

---


## ☕ Sponsor

If CodexMate helps you, feel free to buy me a coffee ☕️

<p align="center">
  <img src="assets/images/sponsor.jpg" alt="Sponsor QR Code" width="200">
</p>


## 📄 License

This project is licensed under the MIT License — see [LICENSE](./LICENSE).
