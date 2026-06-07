<h1 align="center">CodexMate</h1>

<p align="center">
  <a href="./README.en.md">English</a> · <b>中文</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-green" alt="Version 1.0.0">
  <img src="https://img.shields.io/badge/platform-macOS-lightgrey" alt="macOS">
  <img src="https://img.shields.io/badge/Rust-🦀-orange" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-2-ffc131" alt="Tauri 2">
</p>

<p align="center">
  Codex 的本地增强管理工具 —— 图形化管理界面 · 智能路由 · 模型聚合 · 运行诊断
</p>

---

## ✨ 功能

- **智能路由管理** — 在可视化管理界面中维护 provider、模型映射、fallback 和多模态回退策略
- **多供应商聚合** — 将 OpenAI、DeepSeek、本地模型等不同上游统一映射到可选模型列表，同一切换
- **协议适配** — 兼容 `Responses API`、`Chat Completions` 与部分第三方接口
- **注入增强** — 通过 CDP 注入脚本增强 Codex 能力，不修改 Codex 应用本体
- **模式切换** — 支持直连模式 / 代理模式一键切换与重启
- **日志诊断** — 查看运行日志、导出诊断报告、快速定位问题

> CodexMate 不是 Codex 的替代品，而是围绕 Codex 的本地配套工具。

---


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


## 📥 安装

从 [Releases](https://github.com/Jasoncasper/CodexMate/releases) 下载最新 `.dmg` 安装包：

```bash
# 或通过 Homebrew 安装（后续支持）
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
- Rust stable
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

## 🤝 贡献

欢迎提交 issue 和 PR。详见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

---

## 📄 许可证

本项目基于 MIT License 发布，见 [LICENSE](./LICENSE)。
