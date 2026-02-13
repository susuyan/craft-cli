# craft-cli

Craft.do API 的命令行工具，专为 AI Agent 和自动化脚本设计。

## 特性

- **JSON 优先**: 所有输入输出均为结构化 JSON
- **无状态**: 每次调用独立完成，不依赖会话
- **管道友好**: 支持 stdin 输入、stdout 输出
- **多连接**: 一个 CLI 管理多个 API 端点
- **调试支持**: `--verbose` 参数输出详细日志

## 安装

```bash
cargo build --release
# 二进制文件在 target/release/craft-cli
```

## 快速开始

### 1. 初始化配置

```bash
craft-cli config init
```

### 2. 添加 API 连接

```bash
# 公开访问（Token 在 URL 中）
craft-cli config add daily "https://connect.craft.do/links/XXXX/api/v1"

# 密钥访问
craft-cli config add secure "https://connect.craft.do/links/YYYY/api/v1" \
  --key "your-secret-key"
```

### 3. 设置默认连接

```bash
craft-cli config default daily
```

### 4. 开始使用

```bash
# 获取今日笔记
craft-cli blocks get --date today

# 插入内容
echo "## 新笔记" | craft-cli blocks insert --date today --stdin

# 搜索内容
craft-cli search "关键词"
```

## 全局选项

```
--url <URL>      API 端点 URL（或 CRAFT_API_URL 环境变量）
--key <KEY>      API 密钥（或 CRAFT_API_KEY 环境变量）
--config <PATH>  指定配置文件路径
--conn <NAME>    选择配置文件中的连接名
--verbose, -v    启用调试日志输出
--help, -h       显示帮助信息
--version, -V    显示版本信息
```

## 命令参考

### config - 配置管理

```bash
craft-cli config init                          # 初始化配置文件
craft-cli config add <name> <url> [--key <key>]  # 添加连接
craft-cli config remove <name>                 # 删除连接
craft-cli config list                          # 列出所有连接
craft-cli config default <name>                # 设置默认连接
```

### blocks - 块操作

```bash
# 获取块内容
craft-cli blocks get [--date <d>] [--id <id>] [--depth <n>]

# 插入块
craft-cli blocks insert [--date <d>] [--page-id <id>] [--pos start|end] <markdown|--stdin>

# 更新块
craft-cli blocks update <id> <markdown|--stdin>

# 删除块
craft-cli blocks delete <id>...

# 移动块
craft-cli blocks move <id>... [--date <d>] [--page-id <id>] [--pos start|end]

# 搜索块
craft-cli blocks search <pattern> [--date <d>] [--case-sensitive]
```

### tasks - 任务管理

```bash
# 列出任务
craft-cli tasks list <scope>  # scope: active, inbox, upcoming, logbook

# 添加任务
craft-cli tasks add <text> [--schedule <d>] [--deadline <d>] [--to inbox|daily]

# 更新任务
craft-cli tasks update <id> [--text <t>] [--schedule <d>] [--state <s>]

# 完成任务
craft-cli tasks done <id>...

# 删除任务
craft-cli tasks delete <id>...
```

### search - 全局搜索

```bash
craft-cli search <query> [--from <d>] [--to <d>] [--regex]
```

### collections - 集合操作

```bash
craft-cli collections list [--from <d>] [--to <d>]
craft-cli collections schema <id>
craft-cli collections items <id> [--depth <n>]
craft-cli collections add-item <id> --title <t> [--prop key=val]...
craft-cli collections update-item <id> <item-id> [--prop key=val]...
craft-cli collections delete-item <id> <item-id>...
```

### upload - 文件上传

```bash
craft-cli upload <file> [--date <d>] [--page-id <id>] [--pos start|end]
```

### connection - 连接信息

```bash
craft-cli connection info
```

## 使用示例

### 日常笔记工作流

```bash
# 获取今日笔记内容
craft-cli blocks get --date today

# 添加今日计划
cat <<'EOF' | craft-cli blocks insert --date today --stdin
## 今日计划

- [ ] 完成项目文档
- [ ] 代码审查
- [ ] 团队会议
EOF

# 完成任务
craft-cli tasks done "task-id-1" "task-id-2"
```

### 自动化脚本

```bash
#!/bin/bash
# daily-sync.sh - 每日笔记同步

export CRAFT_API_URL="https://connect.craft.do/links/XXXX/api/v1"

# 获取今日任务
TASKS=$(craft-cli tasks list active)

# 生成日报
cat <<EOF | craft-cli blocks insert --date today --stdin
## $(date +%Y-%m-%d) 日报

### 今日任务
$(echo "$TASKS" | jq -r '.items[].markdown')

### 备注
- 同步时间: $(date)
EOF
```

### 带调试输出

```bash
# 启用详细日志
craft-cli --verbose --conn test blocks get --date today
```

## 配置文件

配置文件位置:
- macOS: `~/Library/Application Support/craft-cli/config.toml`
- Linux: `~/.config/craft-cli/config.toml`

示例配置:

```toml
default = "daily"

[connections.daily]
url = "https://connect.craft.do/links/XXXX/api/v1"

[connections.work]
url = "https://connect.craft.do/links/YYYY/api/v1"
secret_key = "your-secret-key"
```

## 退出码

| 码 | 含义 |
|----|------|
| 0 | 成功 |
| 1 | API 错误 |
| 2 | 配置错误 |
| 3 | 参数错误 |
| 4 | IO 错误 |
| 5 | 解析错误 |
| 6 | JSON 错误 |
| 10 | 网络错误 |

## 开发

```bash
# 构建
cargo build --release

# 测试
cargo test

# 运行（开发模式）
cargo run -- --verbose connection info
```

## 许可证

MIT
