# KSE 用户指南

KSE（Kernel Submodule Editor）是一个面向多仓库项目的 Git 子模块管理工具。KSE **不做** `git` 已有的事（添加、初始化、更新、切换分支等直接用 `git` 命令），只做 `git` **做不到**的事：**三路 commit 比对 + 7 种状态分类** 和 **子模块 → 父仓库指针同步**。

## 安装

### 前置依赖

```bash
# Ubuntu / Debian
sudo apt install libgit2-dev pkg-config cmake
# macOS
brew install libgit2
```

### 从源码构建

```bash
git clone <repo-url>
cd examples/default
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Tauri 桌面应用（可选）

```bash
cargo install tauri-cli
cargo tauri dev      # 开发模式
cargo tauri build    # 构建安装包
```

### 卸载

```bash
rm $(which kse)
rm -rf .git/kse/history.db          # 清理历史数据
rm -rf ~/.local/share/com.kse.kse   # Tauri 应用数据
```

---

## CLI 快速参考

```
kse <COMMAND> [选项] [参数]
```

所有命令支持 `--dry-run` 预览模式。

| 选项 | 说明 |
|------|------|
| `--dry-run` | 预览模式：仅输出操作计划，不执行 |
| `--help` | 显示帮助信息 |

---

## 命令详解

### `kse status` — 查看子模块状态

KSE 的核心贡献。扫描仓库中的所有子模块，通过**三路 commit 比对**（父仓库指针 / 本地 HEAD / 远程 HEAD）判定每个子模块的状态。

```bash
# 扫描当前目录
kse status

# 扫描指定仓库
kse status /path/to/repo

# 输出示例
# 仓库: /home/user/my-project
# 子模块总数: 3
# 干净: 1
# 需要关注: lib-a, lib-b
# 聚合统计:
#   总数: 3
#   ✅ Clean: 1
#   ⬇ BehindRemote: 1
#   🔴 Dirty: 1
#
# 名称                    状态             分支         差异
# lib-a                  BehindRemote     main        -3
# lib-b                  Dirty            dev
# lib-c                  Clean            main
```

**7 种状态（KSE 独有，`git submodule status` 做不到的分类粒度）**：

| 状态 | 含义 | 建议 |
|------|------|------|
| Clean | 三方 commit 一致 | 无需操作 |
| AheadOfParent | 本地有父仓库未记录的新提交 | `kse sync parent <name>` |
| BehindRemote | 远程有更新，本地落后 | `git submodule update --remote <name>` |
| Detached | 游离 HEAD | `git checkout <name> <branch>` |
| Dirty | 有未提交的修改 | 手动 commit 或 stash |
| Orphaned | 父仓库记录的 commit 在远程已不存在 | 手动干预 |
| Uninitialized | 尚未初始化 | `git submodule update --init <name>` |

当远程仓库不可达时，状态列显示 🛰 标记，跳过 Orphaned/BehindRemote 判定避免误报。

---

### `kse sync parent` — 同步子模块指针到父仓库

KSE 的核心贡献。Git 没有"子模块有更新了，帮我把父仓库指针更新一下"这条命令，`kse sync parent` 把这个流程封装成了一个原子操作。

```bash
# 同步单个子模块
kse sync parent lib-a

# 同步所有子模块
kse sync parent --all
# 或使用快捷别名
kse sync lib-a
kse sync-all
```

执行的操作：
1. 将子模块路径添加到父仓库索引
2. 创建 commit（消息："chore: 更新子模块 'name' 指针"）
3. 记录操作历史

---

### `kse sync platform` — 跨环境版本对齐（CI 场景）

检查指定子模块在当前环境的目标版本状态。适用于 CI/CD 中确保各环境子模块版本一致。

```bash
kse sync platform lib-a --env production
kse sync platform lib-b -e staging
```

输出当前子模块的三个 commit + 状态 + 建议操作，不做变更。

---

### `kse retire` — 退役子模块

`git submodule deinit` 只清理工作区，`.gitmodules` 条目和 index 还得手动处理。`kse retire` 把"反注册"完整自动化了。

```bash
kse retire lib-old
kse retire lib-old /path/to/project
```

执行的操作：
1. `git submodule deinit -f <name>`
2. 从 `.gitmodules` 移除配置段
3. 从索引中移除
4. 记录退役信息到 SQLite 数据库

---

### `kse history` — 查看操作历史

```bash
kse history                          # 最近 20 条
kse history -n 50                    # 最近 50 条
kse history -m lib-a                 # 按子模块筛选
kse history --start 2024-01-01       # 按起始日期筛选
kse history --end 2024-12-31         # 按结束日期筛选

# 组合
kse history -n 10 -m lib-b --start 2024-06-01
```

历史数据存储在 `.git/kse/history.db`（SQLite），可直接查询：

```bash
sqlite3 .git/kse/history.db "SELECT * FROM operations ORDER BY id DESC LIMIT 10;"
```

---

### `kse export-ci` — 导出 CI 脚本

将当前子模块状态导出为可执行的 CI 配置（仅导出非 Clean 的子模块）：

```bash
kse export-ci                        # 输出 shell 脚本
kse export-ci -f github              # GitHub Actions YAML
kse export-ci -f gitlab              # GitLab CI YAML
kse export-ci -f shell -o update.sh  # 写入文件
```

---

## Web UI 使用指南

### 启动

```bash
cargo tauri dev
```

### 界面

```
┌──────────────────────────────────────────────────────┐
│  KSE  [仓库路径: ________________] [刷新]             │
├────────────┬─────────────────────────────────────────┤
│  概览      │  子模块列表                              │
│  总数: 3   │  ☑ 名称    │ 状态 │ 分支  │ 操作        │
│  干净: 1   │  ☐ lib-a   │ ●领先 │ main │ [同步] [退役]│
│  关注: 2   │  ☑ lib-b   │ ●脏   │ dev  │ [退役]      │
│            │  ☐ lib-c   │ ●干净 │ main │             │
│  同步操作  ├─────────────────────────────────────────┤
│  [全部同步]│  详情面板                                 │
│            │  lib-b ●脏                               │
│  导出 CI   │  差异: 同步                               │
│  [Shell]   │  有未提交的修改。建议: 手动 commit...     │
│  [GitHub]  │  ┌──────┬──────┬──────┐                  │
│  [GitLab]  │  │父指针 │ HEAD │ 远程 │                  │
│            │  └──────┴──────┴──────┘                  │
│  操作历史  │  [同步到父仓库] [退役]                    │
│  ✓ sync   ├─────────────────────────────────────────┤
│  ✓ sync   │  操作历史                                 │
└────────────┴─────────────────────────────────────────┘
```

详情面板显示三个 commit 对比 + commit 差异数 + 状态引导。KSE 只提供"同步到父仓库"和"退役"两个原生操作，其余操作会给出建议文案引导用户使用原生 git 命令。

---

## 撤销操作

```bash
# 查看父仓库 reflog
git reflog
git reset --hard HEAD@{1}

# 查看子模块 reflog
cd <子模块路径>
git reflog
```

> ⚠️ `git reset --hard` 会丢弃所有未提交的修改，操作前请确认工作区干净或已 stash。

---

## 常见场景

### 日常开发

```bash
# 1. 查看状态
kse status

# 2. 更新子模块到最新（使用原生 git）
git submodule update --remote --merge

# 3. 在子模块中工作
cd libs/lib-a
git checkout -b my-feature
# ... 修改代码 ...
git add . && git commit -m "feat: ..."

# 4. 同步子模块指针到父仓库（使用 KSE）
cd ..
kse sync parent lib-a

# 5. 提交父仓库
git add libs/lib-a
git commit -m "chore: update lib-a"
```

### 批量同步

```bash
kse status                              # 查看哪些需要同步
kse sync parent --all                   # 全部同步到父仓库
```

### 清理子模块

```bash
kse status                              # 确认要退役的子模块
kse retire lib-old                      # 退役
git add .gitmodules
git commit -m "chore: retire submodule lib-old"
```

---

## 故障排除

| 问题 | 原因 | 解决 |
|------|------|------|
| `无法打开 Git 仓库` | 路径不是 Git 仓库 | 确认存在 `.git` 目录 |
| `找不到子模块 XX` | 子模块名称错误 | 运行 `kse status` 查看正确名称 |
| 状态显示 🛰 | 远程仓库不可达 | 运行 `git fetch` 后再试 |
| 状态显示 Orphaned | 父仓库记录的 commit 已不存在 | 远程分支可能被 rebase 或删除，需手动修复 |
| `cargo build` 失败 | git2 编译需要 libgit2 | 使用 `vendored-libgit2` 特性（已配置） |

```bash
kse --help          # 全部命令概览
kse <COMMAND> --help  # 单个命令详情
```

---

## 相关文档

- [迭代计划](../ROADMAP.md) — 版本规划
- [变更日志](../CHANGELOG.md) — 版本历史
