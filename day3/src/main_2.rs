use std::{fs::File, io::Read};

pub fn main2() {
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

    for i in (0..lines.len()).step_by(3) {
        let first = lines[i];
        let second = lines[i+1];
        let third = lines[i+2];
        for (_i, c) in first.chars().enumerate() {
            if second.contains(c) && third.contains(c) {
                println!("{} - {} - {} : {:?}", first, second, third, c);
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
