use std::{fs::File, io::Read};

fn main() {
    //Read the file data.txt
    let mut file = File::open("data.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    //Split the file data into lines
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut res : Vec<u32> = Vec::new();
    res.push(0);
    let mut max = 0;
    let mut idx_max = 0;
    let mut i:usize = 0;
    for line in lines {
        if line.trim().is_empty() || line == "\r" {
            if res[i] > max {
                max = res[i];
                idx_max = i;
            }
            i = i + 1;
            res.push(0);
        }else {
            let num = line.parse::<u32>().unwrap();
            res[i] = res[i] + num;
        }
    }
    print!("The array : {:?}", res);
    println!("The maximum is {}", max);
    println!("The index of the maximum is {}", idx_max);
    let mut sum = max;
    //Remove the maximum
    res.remove(idx_max);
    //Find the maximum
    max = 0;
    idx_max = 0;
    for i in 0..res.len() {
        if res[i] > max {
            max = res[i];
            idx_max = i;
        }
    }
    println!("{}",max);
    sum += max;
    //Remove the maximum
    res.remove(idx_max);
    //Find the maximum
    max = 0;
    for i in 0..res.len() {
        if res[i] > max {
            max = res[i];
        }
    }
    println!("{}",max);
    sum += max;
    print!("sum = {}",sum);

}
