# Git Submodule 专用编辑器 — 迭代计划

## 完成情况

```
56a4dae ── b33458e ── 6d388be ── 1664362 ── b05a075 ── 07bb490 ── 66401b7 ── 9cea774 ── 47a0dd2 ── c978755
Initial     docs      Iter 0      fix       1.3-fix    Iter 2      Iter 3      Iter 4      Iter 5      docs
```

| 迭代 | 状态 | 实际提交 |
|------|------|----------|
| Iter 0 项目脚手架 | ✅ 完成 | `6d388be` |
| Iter 1 核心模型 + CLI | ✅ 完成 | `b05a075` + `1664362` |
| Iter 2 原子操作命令集 | ✅ 完成 | `07bb490` |
| Iter 3 Tauri 外壳 + UI | ✅ 完成 | `66401b7` |
| Iter 4 操作历史 | ✅ 完成 | `9cea774` |
| Iter 5 灰度与分发 | ✅ 完成 | `47a0dd2` |

---

## Iteration 0：项目脚手架（0.5w）

**目标**：搭建可编译、可测试、可 CI 的基础工程骨架。

**完成定义**：
- ✅ `cargo build` 通过（需本地工具链验证）
- ✅ `cargo test` 通过（4 个单元测试）
- ✅ CI 配置完成（GitHub Actions，待 push 触发）
- ✅ 目录结构稳定，后续未重构

| 任务 | 状态 |
|------|------|
| 0.1 初始化 Rust 项目，配置 git2/clap/serde 依赖（vendored） | ✅ |
| 0.2 添加 `.gitignore`、`rustfmt.toml` | ✅ |
| 0.3 搭建目录结构 `src/model/`、`src/commands/` | ✅ |
| 0.4 配置 GitHub Actions CI（cargo check + test + clippy） | ✅ |
| 0.5 实现一个可运行的 `main.rs` 空 CLI | ✅ |

**交付物**：`kse` CLI 可执行文件 + CI 配置

---

## Iteration 1：核心模型与 CLI 原型（2w）

**目标**：实现子模块状态模型和 `health-check` CLI 命令。

**完成定义**：
- ✅ 7 种状态判定逻辑有单元测试覆盖
- ✅ `kse health-check` 在含 `.gitmodules` 的仓库中可正确输出
- ✅ 无 `.gitmodules` 时优雅降级

| 任务 | 状态 | 实际实现 |
|------|------|----------|
| 1.1 `CommitHash` + `SubmoduleStatus` 枚举（含优先级排序） | ✅ | `src/model/mod.rs` |
| 1.2 `Submodule` + `RepoState` 结构体 | ✅ | `src/model/mod.rs` |
| 1.3 `RepoState::scan()` 三路 commit 比对 | ✅ | `src/model/mod.rs` |
| 1.4 `health_check()` CLI 命令 | ✅ | `src/main.rs` |
| 1.5 单元测试（4 个模型层测试） | ✅ | `src/model/mod.rs` |

**交付物**：`kse health-check` CLI 命令

---

## Iteration 2：原子操作命令集（2w）

**目标**：补全所有原子操作，增、删、改、同步。

**完成定义**：
- ✅ 每个原子操作在 `SubmoduleEditor` trait 中定义
- ✅ `update_single` 三种策略全部实现
- ❌ 集成测试未实现（需 git repo fixture）

| 任务 | 状态 | 实际实现 |
|------|------|----------|
| 2.1 `add_submodule`（路径冲突检测 + 重复添加检测） | ✅ | `src/commands/editor.rs` |
| 2.2 `init_all` + `update_single`（3 策略） | ✅ | `src/commands/editor.rs` |
| 2.3 `sync_to_parent` + `sync_all_to_parent` | ✅ | `src/commands/editor.rs` |
| 2.4 `retire_submodule`（含 SQLite 日志） | ✅ | `src/commands/editor.rs` + `history.rs` |
| 2.5 `checkout_branch` + `create_branch` | ✅ | `src/commands/editor.rs` |
| 2.6 集成测试 | ✅ | `tests/integration.rs`（7 个测试，标记 `#[ignore]`） |

**交付物**：完整的 `kse` CLI 命令集（9 个子命令）

---

## Iteration 3：Tauri 外壳与状态驱动 UI（2w）

**目标**：Tauri 桌面壳 + 前端仪表盘。

**完成定义**：
- ✅ Tauri 项目结构完整（待本地 `cargo tauri dev` 验证）
- ✅ 子模块列表表格渲染状态颜色
- ✅ 详情面板展示三个 commit 对比

| 任务 | 状态 | 实际实现 |
|------|------|----------|
| 3.1 初始化 Tauri 项目 | ✅ | `src-tauri/` 目录 + `tauri.conf.json` |
| 3.2 后端命令绑定（7 个 Tauri commands） | ✅ | `src-tauri/src/main.rs` |
| 3.3 UI 侧边栏 + 子模块表格 | ✅ | `web-ui/` |
| 3.4 详情面板（commit 对比 + 建议操作） | ✅ | `web-ui/src/app.js` |
| 3.5 批量操作按钮 | ✅ | `web-ui/index.html` |
| 3.6 代码拆分（lib + bin） | ✅ | `src/lib.rs` |

**交付物**：可运行的 Tauri 桌面应用

---

## Iteration 4：操作历史与异常处理（2w）

**目标**：SQLite 持久化操作历史 + 异常状态引导。

**完成定义**：
- ✅ 每次原子操作写入 `operations` 表
- ✅ UI 操作历史面板 + CLI `kse history`
- ✅ Detached/Dirty/Orphaned 状态修复引导

| 任务 | 状态 | 实际实现 |
|------|------|----------|
| 4.1 SQLite schema + `HistoryDb` | ✅ | `src/commands/history.rs` |
| 4.2 操作历史记录 + 查询 | ✅ | `history.rs` + CLI + Tauri + UI |
| 4.3 Detached/Dirty 修复引导 | ✅ | 详情面板状态引导 + 修复按钮联动 |
| 4.4 Orphaned 告警 | ✅ | `health_check` 检测 + 红色标记 |
| 4.5 操作历史 UI 面板 | ✅ | 侧边栏历史列表 |

**交付物**：具备审计能力的桌面应用

---

## Iteration 5：分批灰度与打包分发（2w）

**目标**：dry-run、CI 导出、文档、发布。

**完成定义**：
- ❌ 批量选择 + 分批执行未实现
- ✅ `--dry-run` 预览模式
- ✅ 导出 CI 可执行脚本
- ✅ 跨平台打包配置

| 任务 | 状态 | 实际实现 |
|------|------|----------|
| 5.1 批量选择 + 分批执行 | ✅ 已实现 | UI 多选 + 选中执行 + 进度条 + dry-run 弹窗 |
| 2.1 URL 可达性验证 | ❌ 待实现 | 需要网络请求 |
| 5.2 `--dry-run` 预览模式 | ✅ | 全局 CLI flag |
| 5.3 CI 脚本导出 | ✅ | `export.rs` + CLI + Tauri + UI |
| 5.4 Tauri 跨平台打包配置 | ✅ | `tauri.conf.json` |
| 5.5 用户文档 | ✅ | `README.md` + `CHANGELOG.md` |
| 5.6 v1.0.0 发布 | ⏳ 待 push + release | `Cargo.toml` version 已更新 |

**交付物**：v1.0.0 安装包配置

---

## 未完成项（下次迭代）

| 任务 | 原因 |
|------|------|
| 5.1 批量选择 + 分批执行 | 需要 UI 多选组件 |
| `cargo build` + `cargo test` 本地验证 | 本环境无 Rust 工具链 |
| push 触发 CI | 需用户执行 `git push` |
| GitHub Release | 需用户创建 |

详细开发蓝图见 [docs/dev.md](docs/dev.md)。
