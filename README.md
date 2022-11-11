# 这是 Rust 官方的 猜数字游戏小项目

## 1. 创建项目

`Rust` 的项目管理工具是 `Cargo` ，所以创建项目也要用 `Cargo` 工具:
```shell
$ cargo new guessing_game
```

生成的项目结构为：

```shell
-> % tree          
.
├── Cargo.toml
├── README.md
└── src
    └── main.rs

1 directory, 3 files
```

`README.md` 是创建项目后手动添加的。

## 猜字数字游戏的规则

1. 猜测的数字范围为 `1~100`
2. 猜测部队时循环让用户猜测，直到才对为止
3. 输出才对用了多少次和正确的结果

## 获取用户输入

要获取用户输入,就要先定义一个变量来存储输入的变量。我们给这个遍历起一个名字：
```rust
fn main() {
    // 定义一个可变的 String 类型的变量
    let mut guess = String::new();

    // 打印输出帮助信息
    println!("请输入您的幸运数字：");

    std::io::stdin()
        .read_line(&mut guess)
        .expect("输入出错！！！");

    println!("您的幸运数字是：{guess}");
}
```

上面定义的 `guess` 是字符串类型，需要通过相应的方法转换成整型类型，如 i32\i64，才可以使用，下面来看看怎么实现类型转换。

```rust
fn main() {
    // 定义一个可变的 String 类型的变量
    let mut guess = String::new();

    // 打印输出帮助信息
    println!("请输入您的幸运数字：");

    std::io::stdin()
        .read_line(&mut guess)
        .expect("输入出错！！！");

    println!("您的幸运数字是：{guess}");

    // 以上 guess 是 String 类型

    // guess 在上面已经定义过了，但是 Rust 允许重复定义，在 Rust 中叫做 shadow (隐藏)
    // 新定义的 guess 类型为 i32，可以和上面的类型不同
    let guess: i32 = guess.trim().parse().expect("类型解析失败");

    println!("Guess: {guess}");
}
```

上面代码中使用 `String` 的 `.parse()` 方法可以把字符串解析成相应的类型，这里指定的类型为 `i32`;在调用 `.parse()` 前需要先调用 `.trim()` 把多余的非打印和空格过滤掉，否则会解析失败。

## 实现用户循环判断

```rust
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;
    // 打印输出帮助信息
    println!("请输入您的幸运数字：");

    loop {
        // 定义一个可变的 String 类型的变量
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("输入出错！！！");

        // 以上 guess 是 String 类型

        // guess 在上面已经定义过了，但是 Rust 允许重复定义，在 Rust 中叫做 shadow (隐藏)
        // 新定义的 guess 类型为 i32，可以和上面的类型不同
        // 使用模式匹配来处理错误
        let guess: i32 =  match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("输入格式不对，请重新输入：");
                continue;
            }
        };

        count+=1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("您的数字偏小了")
            }
            Ordering::Greater => {
                println!("您的数字偏大了")
            }
            Ordering::Equal => {
                println!("恭喜，您才对了！，幸运数字是 {guess}，您共猜了 {count} 次。");
                break;
            }
        }
    }
}
```