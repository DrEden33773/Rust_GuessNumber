use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!();
    println!("猜数游戏");
    println!();
    // generate a random number between 1 and 100
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("猜数游戏开始！");
    loop {
        let mut input = String::new();
        // give value to var_input
        loop {
            println!();
            println!("请输入你的猜测：");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    println!("读取输入失败！请重新输入！");
                    continue;
                }
            }
        }
        // convert input to number
        let var_input: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("你没有输入一个纯数字！请重新输入！");
                continue;
            }
        };
        // match var_input with secret_number
        match var_input.cmp(&secret_number) {
            // if var_input is equal to secret_number
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
            // if var_input is less than secret_number
            Ordering::Less => {
                println!("你猜的数字小了！请重新猜测！");
            }
            // if var_input is greater than secret_number
            Ordering::Greater => {
                println!("你猜的数字大了！请重新猜测！");
            }
        }
    }
    println!();
    println!("游戏结束！");
}
