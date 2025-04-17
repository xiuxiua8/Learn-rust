//任务六：编写程序将百分制转换成五分制。
//成绩 90~100 分，输出 5 分；
//成绩 80~ 89 分，输出 4 分；
//成绩 70~ 79 分，输出 3 分；
//成绩 60~ 69 分，输出 2 分；
//成绩 0 ~ 59 分，输出 1 分；
//输入一个百分制成绩，输出一个五分制成绩。

use std::io;

fn main() {
    let mut input = String::new();
    println!("请输入一个百分制成绩：");
    io::stdin().read_line(&mut input).expect("读取失败");
    let score: i32 = input.trim().parse().expect("请输入一个数字");
    let grade = match score {
        90..=100 => 5,
        80..=89 => 4,
        70..=79 => 3,
        60..=69 => 2,
        0..=59 => 1,
        _ => {
            println!("输入的成绩无效");
            return;
        }       
    };
    println!("五分制成绩为：{}", grade);
}   
