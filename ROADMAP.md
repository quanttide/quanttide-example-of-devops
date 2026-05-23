# Git Submodule 专用编辑器 — 迭代计划

## 已完成迭代

| 迭代 | 提交 | 交付 |
|------|------|------|
| Iter 0-8 | 22 个提交 | CLI + Tauri + PyO3 + 全部测试通过 |

---

## Iteration 9：命令重命名与 sync 族重构

### 9.1 `health-check` → `status`

`health-check` 暗示"出问题了才看"，改为 `status` 使其心理模型与 `git status` 一致——日常查看，无论好坏。

| 现命令 | 新命令 | 影响范围 |
|--------|--------|----------|
| `kse health-check` | `kse status` | CLI 子命令 + --help |
| `kse health-check --help` | `kse status --help` | — |
| `health_check()` trait 方法 | `status()` | `commands/mod.rs` trait 定义 |
| `GitSubmoduleEditor::health_check()` | `status()` | `editor.rs` 实现 |
| Tauri command `health_check` | `status` | `src-tauri/src/main.rs` |
| Web UI `invoke('health_check')` | `invoke('status')` | `web-ui/src/app.js` |
| `export.rs` CI 脚本引用 | `kse status` | CI 模板 |
| 测试中的 `health_check()` 调用 | `status()` | 测试文件 |
| `docs/user-guide.md` | 更新命令名 | 文档 |

**兼容策略**：`kse health-check` 作为 alias 保留，输出重定向信息 "`health-check` 已重命名为 `status`，请使用 `kse status`"。

### 9.2 `sync` 命令族重新设计

当前 `sync` 只做一件事：子模块 → 父仓库的 commit 指针更新。重新设计为三子命令：

| 新命令 | 方向 | 说明 |
|--------|------|------|
| `kse sync parent <name>` | 子模块 → 父仓库 | 更新父仓库指针（当前 `sync` 的行为） |
| `kse sync remote <name>` | 远程 → 本地 | 拉取子模块远程最新代码（当前 `update` 的行为） |
| `kse sync platform [--env]` | 跨环境对齐 | CI/CD 中确保子模块版本与环境一致 |

保留 `kse sync` 作为 `kse sync parent` 的快捷别名。

命令变更：

| 现命令 | 新命令 |
|--------|--------|
| `kse sync <name>` | `kse sync parent <name>` |
| `kse sync-all` | `kse sync parent --all` |
| `kse update <name>` | `kse sync remote <name>` |
| `kse update-all` | `kse sync remote --all` |
| — | `kse sync platform [--env prod]` |

### 9.3 受影响文件清单

| 文件 | 变更内容 |
|------|----------|
| `specification/git-submodule.md` | 更新命令名和 sync 族定义 |
| `docs/dev.md` | 更新开发蓝图中接口定义 |
| `docs/user-guide.md` | 全部用法示例重写 |
| `src/commands/mod.rs` | trait 方法重命名 + sync 族新方法 |
| `src/commands/editor.rs` | 实现新方法，保留旧方法代理 |
| `src/main.rs` | CLI 子命令重新注册 |
| `src-tauri/src/main.rs` | Tauri command 重命名 + 新增 |
| `web-ui/index.html` | 按钮文案 |
| `web-ui/src/app.js` | invoke 名更新 |
| `tests/integration.rs` | 调用更新 |
| `src/model/mod.rs` | 如有需要 |

### 9.4 任务分解

| 任务 | 预估 |
|------|------|
| 9.1.1 `health-check` → `status` — Rust 代码层（trait + impl + CLI + Tauri） | 0.3d |
| 9.1.2 `health-check` → `status` — Web UI + 文档 | 0.2d |
| 9.1.3 `health-check` → `status` — 保留别名 + 迁移提示 | 0.1d |
| 9.2.1 `sync parent` 实现（当前 `sync` 的行为） | 0.2d |
| 9.2.2 `sync remote` 实现（当前 `update` 的行为） | 0.2d |
| 9.2.3 `sync parent --all` / `sync remote --all` | 0.1d |
| 9.2.4 `sync platform` 骨架（占位 + --env 参数） | 0.3d |
| 9.2.5 旧命令 `sync`/`update` 保留为 alias | 0.1d |
| 9.2.6 测试更新 + 集成测试 | 0.3d |
| 9.2.7 `docs/user-guide.md` sync 章节重写 | 0.2d |
| 9.3 编译验证（`cargo test` + `clippy`） | 0.1d |

### 9.5 向后兼容保证

- `kse health-check` → 保留，输出 "`health-check` 已重命名为 `status`"
- `kse sync` → 保留，等价于 `kse sync parent`
- `kse update` → 保留，等价于 `kse sync remote`
- `kse update-all` → 保留，等价于 `kse sync remote --all`
- `kse sync-all` → 保留，等价于 `kse sync parent --all`

---

## 待完成（需本地环境）

| 任务 | 命令 |
|------|------|
| Iter 9 编译验证 | `cargo build && cargo test && cargo clippy` |
| Tauri 系统依赖安装 | `sudo apt install libsoup2.4-dev libwebkit2gtk-4.0-dev` |
| CI 触发验证 | `git push origin main` |
| GitHub Release | `gh release create v1.0.0 ...` |

详细开发蓝图见 [docs/dev.md](docs/dev.md)。
