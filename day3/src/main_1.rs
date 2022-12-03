use std::{fs::File, io::Read};

pub fn main1() {
    //Read the file data.txt
    let mut file = File::open("data.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    //Split the file data into lines
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut score : usize = 0;
    let a_value = 'a' as usize;
    let z_value = 'z' as usize;
    let a_uppercase_value = 'A' as usize;

    for line in lines {
        let half = line.len() / 2;
        let first = &line[..half];
        let second = &line[half..];

        for (_i, c) in first.chars().enumerate() {
            if second.contains(c) {
                println!("{} - {} : {:?}", first, second, c);
                let c_value = c as usize;
                if c_value >= a_value && c_value <= z_value {
                    score += c_value - a_value + 1;
                }else {
                    score += c_value - a_uppercase_value + 27;
                }
                break;
            }
        }

        println!("Score: {}", score);
    }

}
