use std::{fs::File, io::Read};

fn main() {
    //Read the file data.txt
    let mut file = File::open("data.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    //Split the file data into lines
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut score : usize = 0;
    for line in lines {
        //Get the first character of the line
        let first_char = line.chars().next().unwrap();
        //Get the last character of the line
        let last_char = line.chars().last().unwrap();
        //match last_char

        match last_char {
            'X' => { //rock or lose
                if first_char == 'B' {//paper, it's a rock
                    score += 1;
                }else if first_char == 'C' {//scissors it's a paper
                    score += 2;
                }else {//it's a scissors
                    score += 3;
                }
                /*score += 1;
                if first_char == 'A' { //rock
                    score += 3;
                }else if first_char == 'C' { //scissors
                    score += 6;
                }*/

            },
            'Y' => {//paper or draw
                score += 3;
                if first_char == 'B' {//paper, it's a paper
                    score += 2;
                }else if first_char == 'C' {//scissors it's a scissors
                    score += 3;
                }else {//it's a rock
                    score += 1; //rock
                }
                /*if first_char == 'A' { //rock
                    score += 6;
                }else if first_char == 'B' {//paper
                    score += 3;
                }*/
                
            },
            'Z' => {//scissors or win
                if first_char == 'B' {//paper, it's a scissors
                    score += 3;
                }else if first_char == 'C' {//scissors it's a rock
                    score += 1;
                }else {//it's a paper
                    score += 2; //rock
                }
                score += 6;
                /*if first_char == 'B' {//paper
                    score += 6;
                }else if first_char == 'C' {//scissors
                    score += 3;
                }*/
            },
            _ => ()
        }
    }
    println!("Score: {}", score);

}
