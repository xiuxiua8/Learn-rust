//任务七：编写程序，从键盘输入 n，计算级数的前 n 项的和并输出。
//S = 1 + 1/2^2 + 1/3^2 + 1/4^2 + ⋯ + 1/n^2

use std::io;

fn main() {
    let mut input = String::new();   
    println!("请输入n：");
    io::stdin().read_line(&mut input).expect("读取失败");
    let n: i32 = input.trim().parse().expect("请输入一个数字");

    let mut sum = 0.0;
    for i in 1..=n {
        sum += 1.0 /( (i as f64) * (i as f64));
    }
    println!("级数的前{}项的和为：{}", n, sum);
}

