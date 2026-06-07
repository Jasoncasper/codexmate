<h1 align="center">CodexMate</h1>

<p align="center">
  <a href="./README.en.md">English</a> · <b>中文</b>
</p>

<p align="center">
  <a href="https://github.com/Jasoncasper/CodexMate/releases">
    <img src="https://img.shields.io/github/v/release/Jasoncasper/CodexMate?color=green&label=版本" alt="版本">
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
  <b>Codex 的本地增强管理桌面工具</b><br>
  图形化管理界面 · 智能路由 · 多模型聚合 · CDP 注入增强 · 运行诊断
</p>

<p align="center">
  <a href="https://github.com/Jasoncasper/CodexMate/releases">
    <img src="https://img.shields.io/badge/📥_下载最新版本-v1.0.0-green?style=for-the-badge" alt="下载">
  </a>
  <a href="#-从源码构建">
    <img src="https://img.shields.io/badge/🔧_从源码构建-开始-orange?style=for-the-badge" alt="构建">
  </a>
</p>

---

## 📸 界面预览

<p align="center">
  <img src="assets/images/screenshot-main.png" alt="CodexMate 主界面" width="700">
  <br>
  <em>CodexMate 主界面 — 快速启动与管理入口</em>
</p>

<p align="center">
  <img src="assets/images/screenshot-routing.png" alt="智能路由界面" width="700">
  <br>
  <em>智能路由 — 管理供应商与模型映射</em>
</p>

<p align="center">
  <img src="assets/images/screenshot-about.png" alt="关于页面" width="700">
  <br>
  <em>关于 — 版本信息与更新检查</em>
</p>

---

## 🤔 为什么需要 CodexMate？

| 场景 | 手动配置 | CodexMate |
|------|---------|-----------|
| 使用多个模型供应商（OpenAI / DeepSeek / 本地模型） | 反复修改环境变量，手动管理 API Key | 🎯 图形化一键盘添加，统一管理 |
| 模型路由与 Fallback | 需要自行编写脚本实现 | ⚡ 可视化配置 fallback 策略 |
| 协议兼容（Responses API / Chat Completions） | 手动调整请求格式 | 🔄 自动协议适配 |
| 增强 Codex 能力（CDP 注入） | 手动注入，无管理界面 | 💉 一键注入与管理 |
| 运行状态与诊断 | 翻日志文件 | 📊 可视化日志查看与诊断报告导出 |
| 直连 / 代理模式切换 | 手动修改配置后重启 | 🔀 一键切换 |

> CodexMate **不是** Codex 的替代品，而是围绕 Codex 的本地配套管理工具，让 Codex 的使用体验更加流畅。

---

## ✨ 核心功能

- 🧠 **智能路由管理** — 在可视化管理界面中维护 provider、模型映射、fallback 和多模态回退策略
- 🔗 **多供应商聚合** — 将 OpenAI、DeepSeek、本地模型等不同上游统一映射到可选模型列表，同一切换
- 🔄 **协议适配** — 兼容 `Responses API`、`Chat Completions` 与部分第三方接口
- 💉 **注入增强** — 通过 CDP 注入脚本增强 Codex 能力，不修改 Codex 应用本体
- 🔀 **模式切换** — 支持直连模式 / 代理模式一键切换与重启
- 📋 **日志诊断** — 查看运行日志、导出诊断报告、快速定位问题

---

## 📥 安装

从 [Releases](https://github.com/Jasoncasper/CodexMate/releases) 下载最新 `.dmg` 安装包：

```bash
# 或通过 Homebrew 安装（即将支持）
# brew install codexmate
```

**系统要求**：macOS 12+

---

## 🚀 快速开始

1. 下载并安装 CodexMate
2. 启动后，管理界面会自动打开
3. 在「路由」页面添加你的模型供应商（OpenAI / DeepSeek / 自定义等）
4. 在「插件」页面管理 MCP / Skills / Plugins
5. 点击顶部「启动 Codex」开始使用

---

## 🔧 从源码构建

### 环境要求

- macOS 12+
- Rust stable（最低支持 Rust 1.80+）
- Node.js 20+ & npm 10+
- Xcode Command Line Tools

### 步骤

```bash
# 1. 克隆仓库
git clone https://github.com/Jasoncasper/CodexMate.git
cd CodexMate

# 2. 安装前端依赖并启动开发模式
cd apps/codexmate-manager
npm install
npm run dev
```

### 常用检查

```bash
cd apps/codexmate-manager
npm run check                           # TypeScript 类型检查

cd /path/to/CodexMate
cargo check --workspace                 # Rust 编译检查
cargo test -p codexmate-manager --lib   # 运行测试
```

### 构建发布包

```bash
cd apps/codexmate-manager
npm run build
```

产物位于：

- `target/release/bundle/macos/CodexMate.app`
- `target/release/bundle/dmg/CodexMate_<version>_<arch>.dmg`

---

## 📁 仓库结构

```
CodexMate/
├── apps/
│   ├── codexmate-launcher/     # 后台启动器（Rust）
│   └── codexmate-manager/      # Tauri 管理界面（React + Rust）
├── crates/
│   ├── codexmate-core/         # 核心逻辑：路由、注入、更新、诊断
│   └── codexmate-data/         # 数据与导出能力
├── assets/                     # 静态资源（注入脚本、图片等）
├── scripts/                    # 打包与发布脚本
└── docs/                       # 文档
```

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Jasoncasper/CodexMate&type=Date)](https://star-history.com/#Jasoncasper/CodexMate&Date)

---

## 🤝 贡献

欢迎提交 issue 和 PR！详见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

我们特别欢迎以下贡献：

- 🐛 Bug 报告与修复
- 🌐 新增模型供应商适配
- 📖 文档改进
- 🎨 UI/UX 优化建议

---


## ☕ 赞助

如果 CodexMate 对你有帮助，欢迎扫码请我喝杯咖啡 ☕️

<p align="center">
  <img src="assets/images/sponsor.jpg" alt="赞助二维码" width="200">
</p>


## 📄 许可证

本项目基于 MIT License 发布，见 [LICENSE](./LICENSE)。
