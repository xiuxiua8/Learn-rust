//任务八：编写程序，实现 “石头剪刀布”游戏，游戏采用三局两胜
//制，请实现上述功能，列出每一局的输赢情况及最后的输赢情况。
use std::io;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Choice {
    Stone,
    Scissor,
    Paper,
}

fn game() -> i32 {
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..3);
    let computer_choice = match choice {
        0 => Choice::Stone,
        1 => Choice::Scissor,
        _ => Choice::Paper,
    };  
    println!("请输入你的选择：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    let user_choice = match input.trim() {
        "1" => Choice::Stone,
        "2" => Choice::Scissor,
        _ => Choice::Paper,
    };
    println!("电脑选择：{:?}", computer_choice);
    println!("你选择：{:?}", user_choice);
    if computer_choice == user_choice {
        println!("平局");
        return 0;
    } else if computer_choice == Choice::Stone && user_choice == Choice::Scissor || 
    computer_choice == Choice::Scissor && user_choice == Choice::Paper || 
    computer_choice == Choice::Paper && user_choice == Choice::Stone {
        println!("你输了");
        return 0;
    } else {
        println!("你赢了");
        return 1;
    }  
}

fn main() {
    let mut win_count = 0;
    for _ in 0..3 {
        let result = game();
        match result {
            1 => win_count += 1,
            _ => (),
        }
    }
    if win_count >= 2 {
        println!("你赢了");
    } else {
        println!("你输了");
    }
}