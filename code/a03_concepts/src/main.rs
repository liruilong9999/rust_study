//! 第 3 章：常用编程概念示例。
//!
//! 学习方式：
//! - 本文件把变量、数据类型、函数、注释、控制流等示例集中放在一起。
//! - 平时练习时，建议一次只取消注释一个 `main` 示例运行，避免出现多个入口函数。
//! - 当前真正参与编译的是文件最下方的 `main`，用于演示 `for` 循环和 `rev()` 倒序迭代。

// 1. 变量与可变性

// fn main() {
//     let mut x = 5;
//     println!("x 的值是：{x}");
//     x = 6;
//     println!("x 修改后的值是：{x}");
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//
//     {
//         // 这里再次使用同名变量，属于遮蔽（Shadowing，遮蔽）。
//         // 遮蔽会创建一个新的变量绑定，而不是在原地修改旧变量。
//         let x = x * 2;
//         println!("内部作用域中的 x：{x}");
//     }
//
//     println!("外部作用域中的 x：{x}");
// }

// fn main() {
//     let spaces = "      ";
//     // 遮蔽允许把字符串重新绑定为数字类型。
//     let spaces = spaces.len();
//     println!("空格数量：{spaces}");
// }

// fn main() {
//     let mut spaces = "      ";
//     // 这里会报错：`mut` 允许修改值，但不允许把变量直接改成另一种类型。
//     spaces = spaces.len();
// }

// 2. 数据类型
// fn main() {
//     let _guess: u32 = "42".parse().expect("不是有效的数字");
//     let _a1: u8 = 255;
//     let _a2: i128 = 3;
//     let _a3: usize = 6;
//
//     // 整数类型速查：
//     // 长度    有符号    无符号
//     // 8 位    i8       u8
//     // 16 位   i16      u16
//     // 32 位   i32      u32
//     // 64 位   i64      u64
//     // 128 位  i128     u128
//     // 架构相关 isize    usize
// }

// fn main() {
//     let x = 2.0; // 默认是 f64
//     let y: f32 = 3.0;
//     println!("x = {x}, y = {y}");
// }

// fn main() {
//     // 加法
//     let sum = 5 + 10;
//
//     // 减法
//     let difference = 95.5 - 4.3;
//
//     // 乘法
//     let product = 4 * 30;
//
//     // 除法
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // 整数除法会直接截断，结果是 -1
//
//     // 取余
//     let remainder = 43 % 5;
//
//     println!(
//         "sum={sum}, difference={difference}, product={product}, quotient={quotient}, truncated={truncated}, remainder={remainder}"
//     );
// }

// fn main() {
//     let t = true;
//     let f: bool = false;
//     println!("t = {t}, f = {f}");
// }

// fn main() {
//     let _c = 'z';
//     let _z: char = 'ℤ'; // 显式写出字符类型
//     let _heart_eyed_cat = '😻';
//     let _text = "这是字符串切片";
// }

// fn main() {
//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("解构后的值：x = {x}, y = {y}, z = {z}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
//     println!("{five_hundred} {six_point_four} {one}");
// }

// fn main() {
//     let _a = [1, 2, 3, 4, 5];
//     let _months: [&str; 12] = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//     let _typed_array: [i32; 5] = [1, 2, 3, 4, 5];
//     let repeated = [3; 5];
//     let first = repeated[0];
//     let second = repeated[1];
//     println!("数组前两个元素：{first}, {second}");
// }

// use std::io;
//
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     println!("请输入数组下标：");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("读取输入失败");
//
//     let index: usize = index.trim().parse().expect("输入的下标不是数字");
//
//     // 如果下标越界，程序会在运行时 panic（程序异常终止）。
//     let element = a[index];
//
//     println!("下标 {index} 对应的元素是：{element}");
// }

// 3. 函数
// fn main() {
//     println!("准备调用另一个函数。");
//     another_function();
// }
//
// fn another_function() {
//     println!("这里是另一个函数。");
// }

// fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("参数 x 的值是：{x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("测量值是：{value}{unit_label}");
// }

// fn main() {
//     // 这里会报错：`let y = 6` 是声明语句，不是表达式，不能直接放进圆括号里赋值。
//     let x = (let y = 6);
//     println!("{x}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1; // 这一行以分号结尾，是语句，值会被丢弃。
//         x + 1 // 最后一行没有分号，所以它是块表达式的返回值。
//     };
//
//     println!("y 的值是：{y}");
// }

// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//     println!("x 的值是：{x}");
// }

// fn main() {
//     let x = plus_one(5);
//     println!("x 的值是：{x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1 // 返回表达式末尾不要加分号，否则函数会变成返回 `()`
// }

// 4. 注释

/* 这是块注释示例。
fn main() {
    let x = plus_one(5);
    println!("x 的值是：{x}");
}

fn plus_one(x: i32) -> i32 {
    /* Rust 支持嵌套块注释。 */
    x + 1
}
*/

/// 这是文档注释，常用于说明紧随其后的函数、结构体或模块。
// //! 这是模块级文档注释，通常写在文件开头说明整个模块用途。

// 5. 控制流
// fn main() {
//     let number = 3;
//
//     if number < 5 {
//         println!("条件为真");
//     } else {
//         println!("条件为假");
//     }
// }

// fn main() {
//     let number = 3;
//
//     if number {
//         // 这里会报错：Rust 不会像某些语言那样自动把整数转换成布尔值。
//         println!("number 是三");
//     }
// }

// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         println!("number 不是 0");
//     }
// }

// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("number 可以被 4 整除");
//     } else if number % 3 == 0 {
//         println!("number 可以被 3 整除");
//     } else if number % 2 == 0 {
//         println!("number 可以被 2 整除");
//     } else {
//         println!("number 不能被 4、3、2 整除");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("number 的值是：{number}");
// }

// fn main() {
//     let condition = true;
//
//     // 这里会报错：`if` 是表达式，两个分支返回的类型必须一致。
//     let number = if condition { 5 } else { "six" };
//     println!("number 的值是：{number}");
// }

// 6. Rust 中的循环
// fn main() {
//     loop {
//         println!("再次执行"); // 无限循环，可用 Ctrl + C 中断
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             // `loop` 本身是表达式，`break 表达式` 可以把值返回给外层。
//             // 这里真正返回的是 `break` 后面的 `counter * 2`。
//             // 分号只是结束这条 `break` 语句，不会吞掉返回值。
//             break counter * 2;
//         }
//     };
//
//     println!("循环返回值是：{result}");
// }

// fn main() {
//     let mut count = 0;
//
//     // `'counting_up` 是循环标签（Label，标签），用于给外层循环起名字。
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 // `break 'counting_up` 会直接跳出被命名的外层循环。
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//
//     println!("最终 count = {count}");
// }

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//
//     println!("发射！");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5 {
//         println!("当前元素是：{}", a[index]);
//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     // `for` 会直接遍历集合元素，写法更简洁，也更不容易写出越界错误。
//     for element in a {
//         println!("当前元素是：{element}");
//     }
// }

/// 当前启用的示例：使用 `for` 循环配合 `rev()` 做倒计时。
fn main() {
    // `1..4` 表示左闭右开区间，即 1、2、3。
    // `rev()` 会把迭代顺序反转，所以输出顺序会变成 3、2、1。
    for number in (1..4).rev() {
        println!("{number}！");
    }

    println!("发射！");
}
