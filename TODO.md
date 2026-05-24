# TODO

## Iter 2：开发辅助命令

参考 `apps/qtcloud-devops/src/cli/docs/dev/` 下的设计文档实现。

### 1. `release status` 命令

设计文档：`docs/dev/release-status.md`

- [ ] 从 `release-journal.jsonl` 读取发布记录
- [ ] 输出：当前版本号、最新发布记录、预发布版本列表
- [ ] 支持 `--json` 输出格式
- [ ] 单元测试

### 2. `plan` 命令

设计文档：`docs/dev/plan.md`

- [ ] 扫描 BUGS.md / ROADMAP.md / TODO.md 等项目管理文件
- [ ] 输出：BUGS 数量与分布、迭代进度、TODO 完成统计
- [ ] 只读，不修改任何文件
- [ ] 单元测试

### 3. `build` 命令

设计文档：`docs/dev/build.md`

- [ ] 运行 `cargo build`（Rust 构建）
- [ ] 输出构建结果和错误信息
- [ ] 支持 `--release` 参数
- [ ] 单元测试

### 4. `test` 命令

设计文档：`docs/dev/test.md`

- [ ] 运行 `cargo test`
- [ ] 输出：总数 / 通过 / 失败 / 跳过
- [ ] 列出失败用例
- [ ] 支持 `--name <pattern>` 过滤
- [ ] 单元测试

---

## P2 — 体验增强（优先级后置）

- [ ] `--dry-run` 支持所有命令
- [ ] `--json` 输出格式（`stage`/`publish` 已有，推广到全部命令）
- [ ] `stage --ratio <0.0-1.0>` 灰度比例参数
- [ ] 审计日志彩色输出（`--verbose`）
