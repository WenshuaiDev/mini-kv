# mini-kv

**mini-kv** 是一个用 **Rust** 写的简易 **命令行键值存储工具**（KV Store CLI）。  
项目目标是练习 Rust **工程级项目结构** + **CLI 工具开发**。

---

## 🚀 功能

- `set <key> <value>` ：设置键值对
- `get <key>` ：获取 key 对应的值
- `del <key>` ：删除指定 key
- `-h | --help` ：显示帮助信息
- 所有数据持久化存储在 `data.json`

---

## ⚡ 安装 / 运行

1. Clone 仓库：

```bash
git clone <你的仓库地址>
cd mini-kv
```

2. 构建并运行：
```bash
cargo build --release
cargo run -- set name dange
cargo run -- get name
cargo run -- del name
cargo run -- -h
```

## 🗂 项目结构
```txt
mini-kv
├── Cargo.toml
└── src
    ├── main.rs      # 程序入口
    ├── command.rs   # 命令解析
    ├── store.rs     # KV 存储逻辑
    └── error.rs     # 统一错误处理
```

## 💻 使用示例
```bash
# 设置 key
cargo run -- set username dange
# 输出: Set successfully!

# 获取 key
cargo run -- get username
# 输出: Value: dange

# 删除 key
cargo run -- del username
# 输出: Deleted successfully!

# 获取不存在 key
cargo run -- get username
# 输出: Key not found

# 查看帮助
cargo run -- -h
```
## 🔮 下一步改进
- 支持多文件或目录存储 
- 增加 list 命令显示所有 key 
- 优化 CLI 参数解析（可以用 clap） 
- 写单元测试覆盖功能