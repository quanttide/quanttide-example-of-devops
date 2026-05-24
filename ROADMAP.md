# ROADMAP — Release 命令

## 已完成

| 迭代 | 交付 |
|------|------|
| Iter 0-9 | CLI 脚手架 + `code status/sync/retire` 子命令 + 测试 |

## 待规划 P0 — Release 命令

参考 `apps/qtcloud-devops/src/cli/` 中 `release.py` + `cli.py` 的设计，在 examples/default 的 Rust CLI 中实现等价的 `release` 子命令。

### 动机

当前 `code` 子命令管理 Git 子模块生命周期，但缺少发布（tag + GitHub Release）能力。`release` 子命令补齐 DevOps 闭环。

### 功能规格

```
qtcloud-devops-code release --version <VERSION> [OPTIONS]
```

| Flag | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `--version` / `-V` | String | 必填 | 版本号（如 `v0.1.0` 或 `pkg/v0.1.0`） |
| `--changelog` | Path | `CHANGELOG.md` | CHANGELOG 路径 |
| `--dry-run` | bool | false | 仅检查，不执行 |
| `--tag-only` | bool | false | 仅打标签，跳过 GitHub Release |
| `--release-only` | bool | false | 仅 GitHub Release（tag 必须已存在） |
| `--yes` / `-y` | bool | false | 跳过确认提示 |

### 行为

1. **预检查**：版本格式、CHANGELOG 存在且含对应条目、工作区干净、在 main/master/release 分支
2. **提取 Release Notes**：从 CHANGELOG.md 解析 `## [{ver}]` 段落
3. **确认**：展示版本+Release Notes 预览，等待用户确认（`-y` 跳过）
4. **执行**：
   - 创建 Git tag → `git tag <version>`
   - 推送 tag → `git push origin <version>`
   - GitHub Release → `gh release create <version> --title <version> --notes <notes>`
5. **回滚**：GitHub Release 失败时自动删除已推送的 tag

### 实现方案

纯 Rust 实现（`std::process::Command` 调用 git/gh），无需额外依赖。结构：

| 文件 | 新增/修改 | 内容 |
|------|-----------|------|
| `src/commands/mod.rs` | 修改 | trait 新增 `release` 相关方法 |
| `src/commands/release.rs` | **新增** | `ReleaseEditor`、precheck、extract_notes、confirm、create_tag、push_tag、create_release、rollback |
| `src/main.rs` | 修改 | CLI 注册 `release` 子命令 |
| `tests/integration.rs` | 修改 | 新增 release 相关集成测试 |

### 基本假设

- 使用 GitHub 托管仓库
- `gh` CLI 已安装且已认证
- CHANGELOG.md 使用 Keep a Changelog 格式
- 版本号为 semver（前缀 v 或 `pkg/v`）
- 工作区必须干净
- 仅在 main / master / release/* 分支上发布
- git remote origin 可访问
- TTY 交互式环境（`--yes` 参数可跳过）
