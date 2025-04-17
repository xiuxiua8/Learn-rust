///任务五：编写程序，为小学生随机出加、减、乘、除法的题目，即每
///次出的题目不仅两个数是随机的，计算方法也是随机的，即可能是加，
///也可能是减，或乘，或除。数在 20 以内，减法要保证被减数大，除
///法要保证被除数大且能整除

use rand::Rng;

fn generate_question() {
    let mut rng = rand::thread_rng();
    loop {
        let a = rng.gen_range(1..=20);
        let b = rng.gen_range(1..=20);
        //println!("suijishu: {}", a);

        let op = match rng.gen_range(0..=3) {
            0 => '+',
            1 => '-',
            2 => 'x',
            _ => '/',
        };
        if op == '/' {
            if a % b != 0 {
                continue;
            }
        } else if op == '-' {
            if a < b {
                continue;
            }
        }
        println!("{} {} {} = ", a, op, b);
        break;
    }
}

fn main() {
    for _ in 0..10 {
        generate_question();
    }
}

