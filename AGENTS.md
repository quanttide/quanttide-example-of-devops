# AGENTS.md

## 项目概览

Release 发布管理 CLI 示例项目。

- **核心语言**：Rust
- **CLI 框架**：clap
- **CLI 名称**：`qtcloud-devops-code`

## 目录结构

```
├── src/
│   └── main.rs         # CLI 入口（Release 命令）
├── Cargo.toml          # 包配置
├── CHANGELOG.md        # 变更日志
├── ROADMAP.md          # 迭代计划
├── TODO.md             # 可执行任务清单
└── AGENTS.md           # 本文件
```

## 开发规范

- **错误处理**：所有操作返回 `Result<T, Error>`，统一在主入口处理错误。
- **幂等性**：标签已存在时跳过创建，不影响后续。

## 子模组信息

本项目作为 `quanttide-devops` 仓库的子模组，路径为 `examples/default`。

**操作规范：**
- 在 `examples/default/` 目录下提交修改
- 回到 `quanttide-devops` 根目录执行 `git add examples/default` 更新父仓库指针
- 拉取更新：`git submodule update --remote examples/default`
