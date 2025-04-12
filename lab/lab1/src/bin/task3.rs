use lab1::utils;

fn jiecheng(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn combination(n: i32, m: i32) -> i32 {
    jiecheng(n) / (jiecheng(m) * jiecheng(n - m))
}   

fn yanghuisanjiao(n: i32) {
    print!("{} ", 1);
    println!();
    for i in 1..=n {
        print!("{} ", 1);
        for j in 1..=i {
            print!("{} ", combination(i, j));
        }
        println!();
    }
}

fn main() {
    println!("Task 3");
    yanghuisanjiao(9);
    //yanghuisanjiao(5);
    //println!("{}", utils::example_function());
}