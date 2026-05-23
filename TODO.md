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

- [x] **6.1.1** 实现 `is_orphaned()` — merge_base 判定（inline in `scan()`）
- [x] **6.1.2** 插入判定分支 — Dirty > Orphaned > Detached
- [x] **6.1.3** 集成测试 — `test_scan_remote_unreachable` + 优先级已覆盖

### 6.2 离线场景处理

- [x] **6.2.1** `Submodule.remote_unreachable: bool`
  - [x] 更新结构体定义 + `SubmoduleInfo` Tauri 结构体
  - [x] `RepoState::scan()` 中 `find_reference` 失败时标记 `true`
- [x] **6.2.2** 远程不可达时判定降级
  - [x] 跳过 Orphaned 判定分支（`!remote_unreachable`）
  - [x] 跳过 BehindRemote 判定分支
  - [x] `ahead_count` / `behind_count` 置 0
- [x] **6.2.3** UI 层展示
  - [x] 状态列显示 🛰 标记（`statusIcon`）
  - [x] 详情面板显示"远程仓库不可达"横幅

### 6.3 AggregateStatus + health_check

- [x] **6.3.1** `AggregateStatus` 结构体 — `total` + 7 种状态计数 + `Default` + `from_submodules()`
- [x] **6.3.2** `scan_all()` — 委托 `RepoState::scan()` + 聚合
- [x] **6.3.3** `health_check()` — 过滤非 Clean + `describe_issue()`（已有）
- [x] **6.3.4** CLI/Tauri/UI 聚合输出
  - [x] `kse health-check` 输出聚合统计
  - [x] Tauri `scan_repo` 返回 `ScanResult { submodules, aggregate }`
  - [x] Web UI 概览区域 + 聚合计数
- [x] 集成测试 — `test_aggregate_status_from_scan`

---

## 待完成（需本地环境）

| 任务 | 命令 |
|------|------|
| 本地编译验证 | `cargo build && cargo test && cargo clippy -- -D warnings` |
| CI 触发验证 | `git push origin main` |
| Tauri 桌面应用启动 | `cargo tauri dev` |
| Tauri 跨平台打包 | `cargo tauri build` |
| GitHub Release | 创建 GitHub Release + 上传安装包 |

<!-- 所有计划任务已完成 -->
