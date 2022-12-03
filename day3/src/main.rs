mod main_2;
mod main_1;
use std::io;

use crate::{main_2::main2, main_1::main1};

fn main() {
    //let the user choose wich part of the problem to solve
    let mut input = String::new();
    println!("Which part of the problem do you want to solve? (1 or 2)");
    let mut input_u: u32 = 0;
    while input_u !=1 && input_u != 2 {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input_u = input.trim().parse().expect("Please type a number!");
        if input_u == 1 {
            main1();
        }else if input_u == 2 {
            main2();
        }else {
            println!("Please type 1 or 2");
        }
    }

}