# TODO — Release 命令

## P0 — 核心实现

### 1. 新建 `src/commands/release.rs`

- [ ] `precheck(version, changelog, release_only) -> Result<Vec<String>>`
  - [ ] 版本号格式校验（vX.Y.Z 或 pkg/vX.Y.Z）
  - [ ] CHANGELOG.md 存在性检查
  - [ ] CHANGELOG 内容包含对应版本条目
  - [ ] `--release-only` 时 tag 已存在
  - [ ] 工作区干净（`git status --porcelain`）
  - [ ] 在 main/master/release/* 分支上
- [ ] `extract_notes(version, changelog) -> Option<String>`
  - [ ] 解析 `## [{ver}]` 段落内容
- [ ] `confirm_release(version, notes, yes) -> bool`
  - [ ] 版本 + Release Notes 预览
  - [ ] 用户交互确认 / `-y` 跳过
- [ ] `create_tag(version) -> bool`
  - [ ] `git tag <version>`
- [ ] `push_tag(version) -> bool`
  - [ ] `git push origin <version>`
- [ ] `get_remote_repo() -> Option<String>`
  - [ ] 从 `git remote get-url origin` 解析 owner/name
- [ ] `create_release(version, notes, repo) -> bool`
  - [ ] `gh release create <version> --title <version> --notes <notes> --repo <repo>`
- [ ] `rollback_tag(version)`
  - [ ] 本地删除 + 远程删除已推送的 tag
- [ ] `run(version, changelog, dry_run, tag_only, release_only, yes) -> i32`
  - [ ] 编排：precheck → confirm → tag+push → release → rollback

### 2. 修改 `src/commands/mod.rs`

- [ ] `pub mod release;` 声明

### 3. 修改 `src/main.rs`

- [ ] 新增 `Commands::Release` 变体
- [ ] 解析 `release` 子命令参数
- [ ] 调用 `release::run()`

### 4. 测试 `tests/integration.rs`

- [ ] `test_release_dry_run` — dry-run 模式不执行任何操作
- [ ] `test_release_precheck_dirty_workspace` — 脏工作区拒绝
- [ ] `test_release_precheck_wrong_branch` — 非发布分支拒绝
- [ ] `test_release_tag_only` — 仅创建 tag 模式
- [ ] `test_release_release_only_needs_existing_tag` — release-only 需要 tag 已存在

### 5. 编译验证

- [ ] `cargo build` 通过
- [ ] `cargo test` 通过
- [ ] `cargo clippy -- -D warnings` 通过

## P1 — 增强

- [ ] CHANGELOG.md 路径自动检测（从 git 根目录）
- [ ] 非 semver 版本号支持
- [ ] 配置化的分支限制
