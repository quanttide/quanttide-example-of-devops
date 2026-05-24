# qtcloud-devops — 软件发布生命周期管理

`qtcloud-devops` 是一个 Rust CLI 工具，提供基于状态机的软件发布生命周期管理。

## 状态机

```
Staged → Published → Retired
   ↓
Cancelled → (可重新 Staged)
```

| 状态 | 含义 |
|------|------|
| Staged | 版本已标记，准备发布 |
| Published | 版本已正式上线（标签推送 + GitHub Release） |
| Cancelled | 发布被取消，可重新 Staged |
| Retired | 版本已退役，**终态**不可逆 |

## 安装

```bash
cd examples/default
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

## CLI 快速参考

```
qtcloud-devops <COMMAND> [选项]
```

### stage — 标记版本

```bash
qtcloud-devops stage -v v1.0.0
```

- 版本号必须符合 `vX.Y.Z` 或 `pkg/vX.Y.Z` 格式
- 若版本已取消（Cancelled），会生成新 UUID 重新 Staged
- 若版本已发布（Published）或已退役（Retired），拒绝操作

### publish — 发布上线

```bash
qtcloud-devops publish -v v1.0.0
qtcloud-devops publish -v v1.0.0 -y   # 跳过确认
```

- 仅允许 Staged → Published 转换
- 执行流程：创建本地标签 → 推送远程 → 创建 GitHub Release（从 CHANGELOG 自动提取 Release Notes）
- 任一步骤失败自动回滚标签

### cancel — 取消发布

```bash
qtcloud-devops cancel -v v1.0.0
```

- 仅允许 Staged → Cancelled 转换
- 自动删除远程标签和 GitHub Release（若存在）

### retire — 退役版本

```bash
qtcloud-devops retire -v v1.0.0
```

- 仅允许 Published → Retired 转换
- **终态操作**，退役后不可重新 Staged

## 数据存储

所有操作追加记录到 `.quanttide/devops/release-journal.jsonl`（JSONL 格式）。启动时回放事件重建当前状态。

## 故障排除

```bash
# 查看帮助
qtcloud-devops --help
qtcloud-devops stage --help
qtcloud-devops publish --help

# 版本号格式错误
错误: 版本号格式错误: 1.0

# 状态转换被拒绝
错误: 版本 v1.0.0 不处于 Staged 状态
错误: 版本 v1.0.0 已发布，不可重复 stage
错误: 版本 v1.0.0 已退役，不可重复 stage

# 版本不存在
错误: 版本 v9.9.9 不存在，请先执行 stage

# 发布中断
qtcloud-devops cancel -V v1.0.0  # 回滚标签后重新 stage → publish
```
