# a02_guessing_name 学习笔记

## 1. 阶段目标

这一阶段开始从“只会输出”进入“可以和用户交互”的程序编写。

当前示例主要围绕“猜数字”展开，练习下面这些能力：

- 导入模块
- 生成随机数
- 读取用户输入
- 使用循环不断重试
- 把字符串解析为数字
- 使用 `match` 处理比较结果和错误结果

## 2. 本阶段核心语法

### 2.1 `use` 导入模块

示例中有类似写法：

```rust
use std::io;
use std::cmp::Ordering;
use rand::RngExt;
```

说明：

- `use` 用于导入模块、类型或特征（Trait，特征）
- `std::io` 用来处理输入输出
- `std::cmp::Ordering` 用来表示比较结果
- `rand::RngExt` 让随机数生成器具备扩展方法能力

### 2.2 本章第三方库

这一章使用了 `rand` 第三方库来生成随机数。

如果要手动补充依赖，可以执行：

```powershell
cargo add rand@0.10.1
```

添加完成后，`Cargo.toml` 中会出现对应的依赖配置。

### 2.3 变量与可变变量

示例里会出现：

```rust
let secret_number = rand::rng().random_range(1..=100);
let mut guess = String::new();
```

说明：

- `let` 用来声明变量
- Rust 默认变量不可变
- `mut` 表示变量可以被修改

### 2.4 创建字符串

```rust
let mut guess = String::new();
```

说明：

- `String::new()` 创建一个空字符串
- 后续输入内容会写入这个字符串

### 2.5 读取输入

```rust
io::stdin().read_line(&mut guess).expect("读取输入失败");
```

说明：

- `io::stdin()` 获取标准输入
- `read_line` 读取一整行内容
- `&mut guess` 表示传入可变引用（Reference，引用）
- `expect(...)` 表示出错时直接终止并打印提示

### 2.6 循环 `loop`

```rust
loop {
    println!("请输入数字：");
}
```

说明：

- `loop` 会无限执行
- 通常结合 `break` 或 `continue` 使用

### 2.7 类型转换

```rust
let guess_text = guess.trim();
let guess: u32 = match guess_text.parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

当前阶段重点不是 `?` 运算符，而是先理解字符串清理、数字解析和 `match` 错误处理这条流程：

- `trim()` 去掉空白和换行
- `parse()` 把字符串解析成数字
- `u32` 表示无符号 32 位整数

### 2.8 `match` 分支匹配

处理解析结果时，常见写法：

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

说明：

- `Ok(num)` 表示解析成功
- `Err(_)` 表示解析失败
- `continue` 表示直接进入下一轮循环

### 2.9 比较结果 `Ordering`

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("小了"),
    Ordering::Greater => println!("大了"),
    Ordering::Equal => println!("猜对了"),
}
```

说明：

- `cmp` 用于比较两个值
- 返回值是 `Ordering`
- 三种结果分别是小于、大于、等于

## 3. 依赖前置知识

建议先掌握：

- `a00_install` 中的环境与 `cargo` 基本命令
- `a01_helloworld` 中的 `fn main` 与 `println!`

## 4. 运行与检查建议

建议在本阶段目录中尝试下面命令：

```powershell
cargo check
cargo run
cargo fmt
```

如果后续安装了 Clippy，也可以执行：

```powershell
cargo clippy
```
