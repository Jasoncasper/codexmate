# CodexMate GitHub 发布清单

这份清单按“第一次把项目作为全新开源仓库发布”的顺序编排，适合不熟悉 GitHub 的场景。

## 1. 先准备本地目标目录

你后续希望本地 Git 工作目录固定为：

```bash
/Users/admins/CodeX/CodexMate
```

建议先创建上层目录：

```bash
mkdir -p /Users/admins/CodeX
```

## 2. 在 GitHub 网页创建新仓库

1. 登录 GitHub
2. 右上角点击 `+`
3. 选择 `New repository`
4. Repository name 填：`CodexMate`
5. Description 可填：`Local enhancement manager for Codex`
6. 选择 `Public`
7. 不要勾选：
   - `Add a README file`
   - `Add .gitignore`
   - `Choose a license`
8. 点击 `Create repository`

创建后你会看到一个类似下面的仓库地址：

```text
https://github.com/<your-github-username>/CodexMate
```

## 3. 把本地项目移动到最终目录

如果你要保留当前目录作为备份，推荐复制而不是直接移动：

```bash
rsync -a \
  --exclude node_modules \
  --exclude target \
  /Users/admins/admin/codexmate/ \
  /Users/admins/CodeX/CodexMate/
```

然后进入新目录：

```bash
cd /Users/admins/CodeX/CodexMate
```

## 4. 回填仓库地址

在正式首次提交前，把根目录 `Cargo.toml` 中的占位仓库地址替换成真实地址：

```toml
[workspace.package]
repository = "https://github.com/<your-github-username>/CodexMate"
```

说明：

- 这个字段会被应用内更新检查读取
- 如果不填，更新检查会明确报“未配置 GitHub 仓库地址”
- 这是为了避免代码继续指向旧仓库

## 5. 初始化本地 git 仓库

如果目录里还没有 `.git`，执行：

```bash
git init
git branch -M main
git add .
git commit -m "chore: initial open source release of CodexMate"
```

## 6. 关联 GitHub 远程仓库

把下面的用户名替换成你的真实 GitHub 用户名：

```bash
git remote add origin https://github.com/<your-github-username>/CodexMate.git
```

验证远程地址：

```bash
git remote -v
```

## 7. 首次推送

```bash
git push -u origin main
```

如果 GitHub 要求登录：

- 按提示在浏览器完成登录
- 或使用 GitHub Desktop / GitHub CLI 先登录后再推送

## 8. 发布前本地检查

```bash
cd apps/codexmate-manager
npm install
npm run check
npm run build

cd /Users/admins/CodeX/CodexMate
cargo check --workspace
cargo test -p codexmate-manager --lib
cargo test -p codexmate-core --test cdp_bridge
```

## 9. 创建首个 Release

建议首个版本 tag 为：

```bash
git tag v1.0.6
git push origin v1.0.6
```

仓库里已有 `.github/workflows/release.yml`，推送 tag 后会自动：

- 构建 macOS release
- 生成 `.dmg`
- 生成 `latest.json`
- 上传到 GitHub Release

## 10. Release 后检查

发布完成后，在 GitHub 仓库页面确认：

- `Releases` 页面存在对应版本
- 资产里有 `.dmg`
- 资产里有 `latest.json`
- README 中链接正常

## 11. 后续我可以继续帮你做的事

你创建好 GitHub 仓库后，把下面任一信息发给我：

- 仓库 URL
- 你的 GitHub 用户名

我可以继续帮你：

- 把 `Cargo.toml` 的 `repository` 改成真实地址
- 检查 Release workflow 是否完全对齐
- 初始化 git 并推送首个提交
- 帮你起草首个 Release note
