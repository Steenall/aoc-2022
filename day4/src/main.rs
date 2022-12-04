use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut count = 0;
    let mut count2 = 0;

    for line in lines {

        let values = line.split(",").collect::<Vec<&str>>();

        let first_bound_to_parse = values[0].split("-").collect::<Vec<&str>>();
        let mut first_value: [i32; 2] = [0, 0];
        first_value[0] = first_bound_to_parse[0].parse().unwrap();
        first_value[1] = first_bound_to_parse[1].parse().unwrap();

        let second_bound_to_parse = values[1].split("-").collect::<Vec<&str>>();
        let mut second_value: [i32; 2] = [0, 0];
        second_value[0] = second_bound_to_parse[0].parse().unwrap();
        second_value[1] = second_bound_to_parse[1].parse().unwrap();

        if first_value[0] <= second_value[0] && first_value[1] >= second_value[1] || first_value[0] >= second_value[0] && first_value[1] <= second_value[1] {
            count+=1;
        }
        else if first_value[0] <= second_value[1] && first_value[0] >= second_value[0] {
            count2+=1;
        }
        else if first_value[1] <= second_value[1] && first_value[0] >= second_value[0] {
            count2+=1;
        }
        else if second_value[0] <= first_value[1] && second_value[0] >= first_value[0] {
            count2+=1;
        }
        else if second_value[1] <= first_value[1] && second_value[1] >= first_value[0] {
            count2+=1;
        }

    }
    println!("Solution 1 : {} - Solution 2 : {}", count, count2+count);

}
