//任务四：编写程序，输入一个字符串，将字符串中的小写字母转换为大写字母
//大写字母转换为小写字母，其他字符不变。

use std::io;

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();
    for c in input.chars() {
        if c.is_lowercase() {
            output.push(c.to_uppercase().next().unwrap());
        } else if c.is_uppercase() {
            output.push(c.to_lowercase().next().unwrap());
        } else {
            output.push(c);
        }   
    }   
    println!("{}", output);
}