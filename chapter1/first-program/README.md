## command
```shell
cargo run 
caergo run --bin new-second-program
```

## 二进制入口配置
```toml
[[bin]]
name = "new-second-program"
path = "src/second.rs"
```