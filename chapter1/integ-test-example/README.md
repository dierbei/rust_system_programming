## command
```shell
# 创建项目
cargo new --lib integ-test-example && cd integ-test-example
```

## 集成测试
#### 第一种
```text
rust 默认会读取 tests 下面的测试
```

#### 第二种
```toml
# 将测试代码放在 tests 文件夹中
# 注意：这是默认情况，可以不写
[[test]]
name = "unit_tests"
path = "tests/unit.rs"

# 将测试代码放在其他文件夹中
[[test]]
name = "integration_tests"
path = "tests/integration.rs"
```