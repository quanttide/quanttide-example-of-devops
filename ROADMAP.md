# ROADMAP — 软件发布生命周期管理

examples/default 实现量潮发布规范的参考示例。

## Iter 1：状态机核心命令 ✓

`stage` / `publish` / `cancel` / `retire` 四个命令 + 事件溯源持久化。

详见 [已完成交付物](#)。

## Iter 2：开发辅助命令（当前）

从平台仓库 `apps/qtcloud-devops` 的开发设计文档提取，在实验室实现原型。

| 命令 | 设计文档 | 说明 |
|------|---------|------|
| `release status` | `docs/dev/release-status.md` | 从 journal 查询发布状态 |
| `plan` | `docs/dev/plan.md` | 扫描 BUGS/ROADMAP/TODO 等生成摘要 |
| `build` | `docs/dev/build.md` | 统一构建入口（cargo build / maturin build） |
| `test` | `docs/dev/test.md` | 统一测试入口（cargo test） |

设计风格：
- Rust 实现
- 状态驱动 / 原子操作
- 与现有 `stage`/`publish`/`cancel`/`retire` 一致的 CLI 接口

## P2 — 体验增强（优先级后置）

- `--dry-run` 支持所有命令
- `--json` 输出格式
- `stage --ratio <0.0-1.0>` 灰度比例参数
- 审计日志彩色输出（`--verbose`）
