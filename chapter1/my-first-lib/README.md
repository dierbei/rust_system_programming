## 创建 lib
```shell
cargo new --lib my-first-lib

cargo build # 会生成 target/debug/libmy_first_lib.rlib
```

## 创建二进制
#### 第一种
```shell
# 新建 bin 文件夹，下面新建入口文件 hh.rs，键入代码
cargo run --bin hh
```

#### 第二种
```shell
# 配置 Config.toml [bin] 属性，创建 mymain.rs，键入代码
cargo run --bin mymain
```