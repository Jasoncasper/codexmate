# CodexMate AGENTS.md

## 版本号管理

修改版本号时，同步更新以下文件：

- `Cargo.toml` — `[workspace.package] version`
- `apps/codexmate-manager/src-tauri/tauri.conf.json` — `"version"`
- `apps/codexmate-manager/package.json` — `"version"`

## 更新日志（CHANGELOG.md）

每次版本发布前，在 `CHANGELOG.md` 顶部追加新条目，格式：

```markdown
## v1.x.x (YYYY-MM-DD)

### 新功能

### Bug 修复

### 其他
```

## Release 构建

版本号确认无误后执行：

```bash
git tag v<version> && git push origin v<version>
```

GitHub Actions 自动编译双架构（aarch64 + x86_64）并上传 DMG 和 latest.json。

## 测试

- 发版前必须运行 `cargo test --workspace`，确保全绿
- `upstream_worktree` 集成测试依赖 git 环境，预存失败需确认是否为环境问题

## 代码规范

- 遵循 Rust 2024 edition 和项目现有风格
- 前端使用 TypeScript + Tailwind CSS + shadcn/ui
- 不引入未使用的新依赖
- 不修改无关文件

## 许可证

All Rights Reserved。代码公开但不授予任何使用、修改或分发权利。
