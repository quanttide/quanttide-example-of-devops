# TODO

## 已完成

### Iteration 0-5

全部完成，详见 git log `6d388be..47a0dd2`。

关键交付：
- `kse` CLI — 14 个子命令，`--dry-run` 全局预览
- Tauri 桌面应用 — 12 个后端命令 + 完整 Web UI
- SQLite 操作历史 — `.git/kse/history.db`
- CI 导出 — shell / GitHub Actions / GitLab CI
- 76 个测试（44 unit + 32 integration）

---

## Iteration 6：规范合规补齐

### 6.1 Orphaned 检测逻辑

- [x] **6.1.1** 实现 `is_orphaned()` — merge_base 判定 parent_pointer reachability（inline in `scan()`）
- [x] **6.1.2** 插入判定分支 — Dirty > Orphaned > Detached（`bb058e6`）
- [ ] **6.1.3** 单元测试 — rebase 后 orphaned 场景（需 git 仓库 fixture）
- [x] 优先级排序已对齐 — `Dirty=0 > Orphaned=1 > Detached=2 > Uninitialized=3 > BehindRemote=4 > AheadOfParent=5 > Clean=6`

### 6.2 离线场景处理

- [ ] **6.2.1** `Submodule` 新增 `remote_unreachable: bool`
  - [ ] 更新结构体定义 + 所有构造位置 + `SubmoduleInfo` Tauri 结构体
  - [ ] `RepoState::scan()` 中 `find_reference` 失败时标记 `true`
- [ ] **6.2.2** 远程不可达时判定降级
  - [ ] 跳过 Orphaned 判定分支
  - [ ] 跳过 BehindRemote 判定分支
  - [ ] `ahead_count` / `behind_count` 置 0
- [ ] **6.2.3** UI 层展示"状态不确定"提示
  - [ ] 状态列显示 `?` 或"离线"标记
  - [ ] 详情面板显示"远程仓库不可达"横幅

### 6.3 AggregateStatus + health_check

- [ ] **6.3.1** 定义 `AggregateStatus` 结构体
  - [ ] `total: usize` + 7 种状态计数
  - [ ] `Default` + `from_submodules(&[Submodule]) -> Self`
- [ ] **6.3.2** 实现 `scan_all()` — 委托 `RepoState::scan()` + 聚合 `AggregateStatus`
- [x] **6.3.3** `health_check()` — 过滤 `status != Clean` + `describe_issue()` 建议操作（`editor.rs`）
- [ ] **6.3.4** 更新 CLI 和 Tauri 绑定
  - [ ] `kse health-check` 输出聚合统计
  - [ ] Tauri command 返回 `AggregateStatus`
  - [ ] Web UI 概览展示各状态计数

---

## 待完成（需本地环境）

| 任务 | 命令 |
|------|------|
| 本地编译验证 | `cargo build && cargo test && cargo clippy -- -D warnings` |
| CI 触发验证 | `git push origin main` |
| Tauri 桌面应用启动 | `cargo tauri dev` |
| Tauri 跨平台打包 | `cargo tauri build` |
| GitHub Release | 创建 GitHub Release + 上传安装包 |

## 低优先级

| 任务 | 原因 |
|------|------|
| URL 可达性验证 | 需要异步网络请求（`reqwest` 或 `curl`），与 6.2 离线场景正交 |
