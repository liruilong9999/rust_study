# a00_install 学习笔记

## 1. 阶段目标

这一阶段还不写正式 Rust 示例代码，先把环境和工具链准备好，保证后续每个阶段都能顺利创建、编译、运行和管理项目。

本阶段重点掌握：

- Rust 安装方式
- Rustup（Rust 工具链管理器）的作用
- Cargo（Rust 包管理与构建工具）的基本用法
- 项目创建、编译、运行、检查、格式化等常用命令

## 2. 需要安装什么

学习 Rust 时，通常会一起安装下面几样工具：

1. `rustup`
2. `rustc`
3. `cargo`

说明：

- `rustup`：Rust 工具链管理器，负责安装和切换 Rust 版本
- `rustc`：Rust 编译器
- `cargo`：Rust 官方包管理与构建工具

一般情况下，安装 `rustup` 后，`rustc` 和 `cargo` 会一起可用。

## 3. Windows 安装步骤

### 3.1 安装 Rust

打开官网下载安装器：

- 官方地址：`https://www.rust-lang.org/tools/install`

Windows 通常下载并运行 `rustup-init.exe`，然后按提示完成安装。

### 3.2 验证安装

安装完成后，在终端执行：

```powershell
rustc --version
cargo --version
rustup --version
```

如果都能正常显示版本号，说明安装成功。

## 4. Cargo 是什么

Cargo（Rust 包管理与构建工具）是 Rust 开发里最常用的命令行工具，主要负责：

- 创建项目
- 下载与管理依赖
- 编译代码
- 运行程序
- 执行测试
- 检查代码
- 生成发布版本

可以把它理解成 Rust 项目的统一入口。

## 5. 常用 Cargo 命令

### 5.1 创建新项目

```powershell
cargo new hello_rust
```

作用：

- 创建一个新的二进制项目
- 默认会生成 `src/main.rs`
- 自动生成 `Cargo.toml`

如果想创建库项目：

```powershell
cargo new my_lib --lib
```

### 5.2 编译项目

```powershell
cargo build
```

作用：

- 编译当前项目
- 生成的文件默认放在 `target/` 目录

### 5.3 运行项目

```powershell
cargo run
```

作用：

- 先编译，再直接运行程序

### 5.4 仅做语法检查

```powershell
cargo check
```

作用：

- 只做编译检查，不生成完整可执行产物
- 速度通常比 `cargo build` 更快

### 5.5 运行测试

```powershell
cargo test
```

作用：

- 运行项目中的测试代码

### 5.6 格式化代码

```powershell
cargo fmt
```

作用：

- 按 Rust 官方风格整理代码格式

### 5.7 代码静态检查

```powershell
cargo clippy
```

作用：

- 使用 Clippy（Rust 代码风格与潜在问题检查工具）分析代码

第一次使用前，可能需要安装组件：

```powershell
rustup component add clippy
rustup component add rustfmt
```

### 5.8 构建发布版本

```powershell
cargo build --release
```

作用：

- 生成优化后的发布版本
- 产物通常位于 `target/release/`

## 6. Cargo.toml 是什么

`Cargo.toml` 是 Rust 项目的配置文件，通常包含：

- 项目名称
- 版本号
- Rust 版本或 Edition（语言版本规范）
- 依赖列表

示例：

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.1"
```

说明：

- `[package]`：项目基础信息
- `[dependencies]`：依赖配置
- `rand = "0.10.1"`：引入随机数库

## 7. 包管理基础

在 Rust 里，依赖通常通过 `Cargo.toml` 管理。

更推荐直接使用 `cargo add` 命令添加依赖，例如：

```powershell
cargo add rand
```

执行后，Cargo 会自动把依赖写入 `Cargo.toml`。

例如添加 `rand` 依赖后，`Cargo.toml` 中通常会出现：

```toml
[dependencies]
rand = "0.10.1"
```

下次执行 `cargo build` 或 `cargo run` 时，Cargo 会自动下载并编译依赖。

如果某一章使用了第三方库，建议在该章文档里同步写出对应的 `cargo add xxx` 命令，方便按章复现。

常见概念：

- Crate（Rust 包单元）：Rust 里的编译单元或包
- Dependency（依赖）：项目引用的外部库
- Registry（包注册中心）：默认通常来自 `crates.io`

## 8. 常见目录说明

Rust 项目里常见这些文件或目录：

- `src/main.rs`：二进制程序入口
- `src/lib.rs`：库项目入口
- `Cargo.toml`：项目配置
- `Cargo.lock`：依赖锁定文件
- `target/`：编译产物目录

其中：

- `target/` 一般不提交到 Git
- `Cargo.lock` 对可执行项目通常建议提交

## 9. 建议的日常开发命令顺序

在学习阶段，建议按下面顺序使用命令：

1. `cargo check`
2. `cargo run`
3. `cargo fmt`
4. `cargo clippy`

这样可以先确认语法，再运行，再整理格式，最后做更细的静态检查。
