# TODO

## Iteration 9：命令重命名与 sync 族重构

### 9.1 `health-check` → `status`

- [ ] **9.1.1** Rust 代码层
  - [ ] `commands/mod.rs`: trait 方法 `health_check()` → `status()`，保留旧方法代理
  - [ ] `commands/editor.rs`: `GitSubmoduleEditor` 实现 `status()`，`health_check()` 调用 `status()`
  - [ ] `src/main.rs`: CLI `HealthCheck` → `Status` 子命令，保留 `HealthCheck` 作为隐藏 alias
  - [ ] `src-tauri/src/main.rs`: Tauri command `health_check` → `status`，保留 `health_check` 作为 alias
- [ ] **9.1.2** Web UI + 文档
  - [ ] `web-ui/src/app.js`: `invoke('health_check')` → `invoke('status')`
  - [ ] `web-ui/index.html`: 按钮文案更新
  - [ ] `docs/user-guide.md`: 所有 `kse health-check` → `kse status`
- [ ] **9.1.3** 别名 + 迁移提示
  - [ ] CLI alias: `HealthCheck` 作为隐藏子命令，输出迁移信息后调用 `Status`
  - [ ] Tauri alias: `health_check` 作为别名调用 `status`

### 9.2 `sync` 命令族重新设计

- [ ] **9.2.1** `sync parent` — 子模块 → 父仓库指针更新（当前 `sync` 的行为）
  - [ ] trait 新增 `sync_parent(name)`
  - [ ] `editor.rs` 实现（代码复用 `sync_to_parent`）
  - [ ] CLI 注册 `kse sync parent <name>`
- [ ] **9.2.2** `sync remote` — 远程 → 本地拉取（当前 `update` 的行为）
  - [ ] trait 新增 `sync_remote(name, strategy)`
  - [ ] `editor.rs` 实现（代码复用 `update_single`）
  - [ ] CLI 注册 `kse sync remote <name> [-s strategy]`
- [ ] **9.2.3** `sync parent --all` / `sync remote --all`
  - [ ] `sync_all_to_parent` → `sync parent --all`
  - [ ] `update_all` → `sync remote --all`
- [ ] **9.2.4** `sync platform` 骨架
  - [ ] trait 新增 `sync_platform(env)`
  - [ ] 基础实现：扫描状态 + 输出差异报告（不执行变更）
  - [ ] CLI 注册 `kse sync platform [--env prod]`
- [ ] **9.2.5** `kse sync` 顶级命令结构
  - [ ] `Commands::Sync` 改为带子命令：`SyncParent`, `SyncRemote`, `SyncPlatform`
  - [ ] `kse sync` 默认等价于 `kse sync parent`
  - [ ] `kse sync-all` → 保留别名指向 `sync parent --all`
  - [ ] `kse update` → 保留别名指向 `sync remote`
  - [ ] `kse update-all` → 保留别名指向 `sync remote --all`
- [ ] **9.2.6** 测试
  - [ ] 更新现有测试中的 `sync_to_parent` / `update_single` 调用
  - [ ] 新增 `sync_platform` 测试

### 9.3 文档更新

- [ ] `docs/user-guide.md` sync 章节重写
- [ ] `docs/dev.md` 接口定义更新
- [ ] `docs/user-guide.md` status 章节重写

---

## 待完成（需本地环境）

| 任务 | 命令 |
|------|------|
| Iter 9 编译验证 | `cargo build && cargo test && cargo clippy -- -D warnings` |
| CI 触发验证 | `git push origin main` |
| Tauri 桌面应用 | `cargo tauri dev` |
| GitHub Release | `gh release create v1.0.0 ...` |
