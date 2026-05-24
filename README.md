# quanttide-example-of-devops — Release 发布管理 CLI

> 本示例项目演示了 `release` 命令的设计与实现。
> 原 Git 子模块管理代码（`code status/sync/retire`）已迁移至 `apps/qtcloud-devops/src/cli/`。
> 参考实现：`apps/qtcloud-devops/src/cli/src/qtcloud_devops_cli/release.py`

## 开发

```bash
cargo build                    # 编译
cargo test                     # 运行测试
cargo clippy -- -D warnings    # 代码检查
```

## 用法

```bash
qtcloud-devops-code release --version v0.1.0                   # 标签 + GitHub Release
qtcloud-devops-code release --version v0.1.0 --tag-only        # 仅创建 Git 标签
qtcloud-devops-code release --version v0.1.0 --release-only    # 仅创建 GitHub Release
qtcloud-devops-code release --version v0.1.0 --dry-run         # 仅检查
qtcloud-devops-code release --version v0.1.0 -y                # 跳过确认
```
