# 中文推广文案

> 推广渠道：V2EX / 掘金 / 知乎

---

## V2EX 推广帖模板

### 标题选项

**A 版（痛点导向）：**
`Codex CLI 重度用户进 —— 受够了手动配环境变量？这个开源工具拯救你`

**B 版（价值导向）：**
`开源推荐：CodexMate —— OpenAI Codex CLI 的本地增强管理桌面工具`

**C 版（对比导向）：**
`手动配置 vs 图形界面 —— 我用 Rust+Tauri 给 Codex CLI 做了个管理面板`

---

### 正文

`[选择一个标题后，以此模板组织正文]`

**前言**

如果你在用 OpenAI Codex CLI（一个终端里的 AI 编程助手），一定遇到过这些痛点：

- 想切 DeepSeek / 本地模型，得反复改环境变量
- 模型挂了没有 fallback，得自己写脚本
- 想看运行日志，得去翻终端输出
- 协议兼容问题需要手动调整请求格式

我花了几周时间用 Rust + Tauri 2 写了个桌面工具，专门解决这些问题。

**CodexMate 是什么？**

不是 Codex 的替代品，而是围绕 Codex 的**本地配套管理工具**。它在 macOS 上跑，提供图形化界面来管理 Codex 的各项配置与增强。

**核心功能一览**

| 功能 | 之前怎么做？ | 现在怎么做？ |
|------|------------|------------|
| 多供应商管理 | 反复改 `.env`，手动管理 API Key | 🎯 图形化一键盘添加 |
| 模型路由 & Fallback | 自己写脚本 | ⚡ 可视化配置 fallback 策略 |
| 协议适配 | 手动调整请求格式 | 🔄 自动适配 Responses API / Chat Completions |
| CDP 注入增强 | 手动注入，无管理界面 | 💉 一键注入与管理 |
| 运行诊断 | 翻终端日志 | 📊 可视化日志 + 诊断报告导出 |
| 代理切换 | 改配置后重启 | 🔀 一键切换 |

**技术栈**

- 前端：React + 桌面原生体验
- 后端：Rust
- 桌面框架：Tauri 2（比 Electron 轻太多）
- 支持：macOS 12+

**开源 MIT 协议，欢迎 Star & PR**

GitHub：https://github.com/Jasoncasper/CodexMate

**截图预览**

主界面、路由管理、关于页面都在 README 里，挺良心的 UI。

---

## 掘金技术文章大纲

### 标题：《我用 Rust + Tauri 2 给 Codex CLI 做了个管理面板，开源了》

### 目录结构

**1. 引言：为什么会有这个项目？**
   - 我在日常使用 Codex CLI 时遇到的几个具体痛点
   - 从"手动脚本管理"到"做个桌面工具"的心路历程
   - 为什么选择开源

**2. Codex CLI 的生态现状**
   - Codex CLI 是什么？
   - 多模型供应商的需求（OpenAI / DeepSeek / 本地 LLM）
   - 手动配置的痛点
   - 协议兼容性问题（Responses API vs Chat Completions）
   - 缺乏官方管理界面

**3. CodexMate 的设计哲学**
   - 配套工具，非替代品
   - 本地优先：所有数据存本地，不收集遥测
   - 图形化管理 > 命令行配置
   - 可扩展架构

**4. 核心功能详解**

   **4.1 智能路由管理**
   - 可视化维护 provider、模型映射
   - Fallback 策略配置
   - 多模态回退

   **4.2 多供应商聚合**
   - OpenAI / DeepSeek / 本地模型统一管理
   - 一键切换上游

   **4.3 协议自动适配**
   - Responses API → Chat Completions 自动转换
   - 第三方接口兼容

   **4.4 CDP 注入增强**
   - 不修改 Codex 本体
   - 一键注入与管理

   **4.5 日志诊断**
   - 可视化日志查看
   - 诊断报告导出

**5. 技术实现**

   **5.1 为什么选 Tauri 2 而不是 Electron？**
   - 包体小（DMG 几 MB）
   - 内存占用低
   - Rust 性能与安全性

   **5.2 架构概览**
   - apps/codexmate-launcher：后台启动器（Rust）
   - apps/codexmate-manager：Tauri 管理界面（React + Rust）
   - crates/codexmate-core：核心逻辑
   - crates/codexmate-data：数据与导出

   **5.3 关键技术决策**
   - 进程间通信设计
   - 配置持久化方案
   - CDP 注入机制

**6. 开发体验与踩坑**
   - Tauri 2 相比 Tauri 1 的变化
   - macOS 平台的兼容性处理
   - Rust + React 的协作模式

**7. 安装与使用**
   - 直接下载 DMG（macOS 12+）
   - 从源码构建（需 Rust + Node.js）

**8. 开源与社区**
   - MIT 许可证
   - 欢迎贡献的领域（适配新供应商 / UI 优化 / 文档）

**9. 结语与展望**
   - 未来规划（Windows 支持？VSCode 插件？）
   - 呼吁 Star & PR

---

## 知乎回答/文章框架

### 问题锚点示例

> **《有哪些好用的 OpenAI Codex CLI 配套工具？》**
> **《如何优雅地管理 Codex CLI 的多模型供应商？》**
> **《你的 AI 编程工具栈里还缺什么？推荐一款开源桌面板》**

### 回答结构

1. **一句话定位**
   "如果你在用 OpenAI Codex CLI 写代码，且需要同时使用多个模型供应商，CodexMate 可能是目前最顺手的开源配套工具。"

2. **使用场景（引发共鸣）**
   - 场景 A：白天用 OpenAI 干活，晚上切到 DeepSeek 省点钱 → 以前改 `.env`，现在点一下
   - 场景 B：API 挂了需要自动 fallback → 以前写脚本，现在可视化配置
   - 场景 C：想知道 Codex 最近为什么变慢了 → 以前翻日志，现在看诊断面板

3. **它与同类工具的区别**
   - 不是 Codex 替代品，是增强配套
   - 可视化管理界面（不是又一个 CLI 工具）
   - 本地优先，隐私安全
   - 开源 MIT，可自建

4. **客观评价**
   - 优点：直观、轻量、功能集中解决痛点
   - 不足：目前仅支持 macOS 12+，生态刚起步
   - 推荐人群：Codex CLI 日常用户、多供应商使用者

5. **链接与呼吁**
   - GitHub：https://github.com/Jasoncasper/CodexMate
   - 欢迎 Star、Issue、PR
