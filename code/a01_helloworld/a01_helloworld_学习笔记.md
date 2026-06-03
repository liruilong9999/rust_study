# a01_helloworld 学习笔记

## 1. 阶段目标

这一阶段是 Rust 入门第一步，目标是能看懂并写出最基础的可运行程序。

当前示例代码核心内容非常简单：

```rust
fn main() {
    println!("hello world!");
}
```

## 2. 本阶段学习的核心语法

### 2.1 `fn main`

Rust 程序通常从 `main` 函数开始执行：

```rust
fn main() {
}
```

说明：

- `fn` 表示定义函数
- `main` 是程序默认入口
- `{}` 是函数体

### 2.2 `println!`

输出文本最常见的方式是：

```rust
println!("hello world!");
```

说明：

- `println!` 是宏（Macro，宏）
- 它会把内容输出到控制台
- 输出后会自动换行

### 2.3 字符串字面量

`"hello world!"` 是字符串字面量，写在双引号中，适合表示固定文本。

## 3. 如何运行本阶段代码

如果当前目录就是 `a01_helloworld`，可以执行：

```powershell
cargo run
```

因为这个项目的 `Cargo.toml` 已经配置了：

```toml
[[bin]]
name = "a01_helloworld"
path = "main.rs"
```

这表示：

- 当前项目是可执行程序
- 入口文件是根目录下的 `main.rs`

## 4. 依赖前置知识

建议先掌握 `a00_install` 中的内容，至少要会：

- 检查 Rust 是否安装成功
- 知道 `cargo run` 是什么
- 知道 `Cargo.toml` 的基本作用
