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
