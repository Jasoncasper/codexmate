# CodexMate UI 改造方案

---

## 一、现状

- App.tsx 1818 行，65 个函数定义
- 左侧边栏 6 个菜单（概览/智能路由/工具与插件/安装维护/关于/设置）
- 2042 行自定义 CSS
- 组件目录只有 shadcn/ui 原子组件 + RoutingPage

---

## 二、布局

侧边栏 → 顶部栏。

```
┌──────────────────────────────────────────────────────────────────┐
│  CM CodexMate    路由   插件   维护   关于       🚀 🔄 🌙 ＋    │  ← 56px
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  [内容区：单列或自适应]                                            │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

- 顶栏右侧：启动 Codex / 刷新 / 主题切换 / 添加模型
- 无功能开关（不存在的交互不设计）
- macOS 标题栏可选（后期 P5 阶段）

---

## 三、Tab 与功能映射

| Tab | 覆盖原菜单 | 内容 |
|-----|-----------|------|
| 路由（首页） | 智能路由 | 模型供应商卡片列表 + 路由规则 + 高级路由配置 |
| 插件 | 工具与插件 | MCP / Skills / Plugins 网格，toggle 启用 |
| 维护 | 安装维护 | 状态检查、入口管理、Watcher、手动启动、启动参数 |
| 关于 | 概览 + 关于 | 见下方合并方案 |

移除：设置（已清理）、概览（合并到关于）

### 3.1 关于页合并方案

原概览 + 原关于 → 合并为关于页，5 个面板去重：

| 面板 | 来源 | 内容 |
|------|------|------|
| 版本信息 | 关于.CodexMate + 概览.Codex版本 | CodexMate 版本、Codex 版本、Codex 应用路径 |
| 状态概览 | 概览.健康检查 精简 | 静默启动入口状态 → 刷新状态 / 打开维护 |
| 更新 | 关于.GitHub Release | 检查/下载更新 |
| 日志 | 关于.日志 | 查看/复制日志 |
| 诊断 | 关于.诊断 | 生成/复制诊断报告 |

移除：原概览「最近启动」面板（启动 Codex 已移至顶栏全局操作）

---

## 四、目录结构

```
src/
├── main.tsx
├── App.tsx                          # ~100 行：Tab 路由 + 布局
├── styles.css                       # 变量 + 动画，逐步迁移 Tailwind
│
├── components/
│   ├── layout/
│   │   └── Topbar.tsx               # 顶栏（品牌 + Tab + 启动/刷新/主题/添加模型）
│   │
│   ├── routing/
│   │   ├── ProviderCard.tsx         # 供应商卡片
│   │   ├── ProviderList.tsx         # 卡片列表容器
│   │   └── ProviderEditor.tsx       # 编辑表单
│   │
│   ├── context/
│   │   ├── ContextPage.tsx          # 插件页主组件
│   │   ├── ContextCard.tsx          # 单项卡片 + toggle
│   │   └── ContextEditor.tsx        # TOML 编辑
│   │
│   ├── maintenance/
│   │   ├── MaintenancePage.tsx      # 维护页
│   │   ├── HealthCheck.tsx          # 状态检查面板
│   │   ├── EntrypointMgmt.tsx       # 入口管理
│   │   ├── WatcherMgmt.tsx          # Watcher
│   │   └── LaunchPanel.tsx          # 手动启动 + 参数
│   │
│   ├── about/
│   │   ├── AboutPage.tsx            # 关于页（含原概览）
│   │   ├── VersionInfo.tsx          # 版本 + Codex 状态
│   │   ├── HealthStatus.tsx         # 状态概览（入口状态）
│   │   ├── UpdateCheck.tsx          # GitHub Release 更新
│   │   ├── LogsPanel.tsx            # 日志
│   │   └── DiagnosticsPanel.tsx     # 诊断
│   │
│   ├── shared/
│   │   ├── NoticeToast.tsx
│   │   ├── ToggleSwitch.tsx
│   │   ├── StatusBadge.tsx
│   │   └── Toolbar.tsx
│   │
│   └── ui/                          # shadcn/ui 保留
│
├── hooks/
│   ├── useOverview.ts
│   ├── useSettings.ts
│   ├── useContext.ts
│   ├── useTheme.ts
│   └── useNavigation.ts
│
└── lib/
    ├── api.ts
    ├── toml.ts
    └── types.ts
```

---

## 五、实施阶段

| 阶段 | 内容 | 状态 |
|------|------|------|
| P0 | `types.ts` + `api.ts` + `toml.ts` 抽取共享代码 | ⬜ |
| P1 | `Topbar.tsx` + `App.tsx` 布局重构（Tab 路由） | ⬜ |
| P1 | `ProviderCard` + `ProviderList` 重写路由页 | ⬜ |
| P2 | `ContextPage` + `ContextCard` 插件页拆分 | ⬜ |
| P2 | `MaintenancePage` 各子组件拆分 | ⬜ |
| P3 | `AboutPage`（含原概览功能）拆分，5 面板去重 | ⬜ |
| P4 | CSS 迁移 Tailwind | ⬜ |
| P5 | macOS 标题栏 + 过渡动画 | ⬜ |

### 约束
- 每阶段确保 `cargo check` + `tsc --noEmit` 通过
- 不设计不存在的功能
- 不改动后端逻辑，只重构前端
