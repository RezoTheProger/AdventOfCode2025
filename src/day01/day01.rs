use std::{fs, path::Path};

fn main() {
    let path = Path::new("/home/rezo/Workspace/AOC2025/src/day01/input.txt");

    match path.exists() {
        true => function2(path),
        false => println!("File does NOT exist!"),
    }
}
fn function(path: &Path) {
    let file: String = fs::read_to_string(path).expect("lmao something went wrong");
    let mut number: i64 = 50;
    let mut num_zero: u64 = 0;
    for line in file.lines() {
        let first_char = line.chars().next().unwrap();

        let number_part: i64 = line[1..].trim().parse().expect("Failed to parse number");
        match first_char {
            'R' => {
                number += number_part;
                num_zero += ((number % 100) == 0) as u64;
            }
            'L' => {
                number -= number_part;

                num_zero += ((number % 100) == 0) as u64;
            }

            _ => println!("broke ahh guy"),
        }
    }

    println!("{}", num_zero);
}
fn function2(path: &Path) {
    let file: String = fs::read_to_string(path).expect("lmao something went wrong");
    let mut number: i64 = 50;
    let mut num_zero: i64 = 0;
    for line in file.lines() {
        let first_char = line.chars().next().unwrap();

        let number_part: i64 = line[1..].trim().parse().expect("Failed to parse number");
        match first_char {
            'R' => {
                let overflow: i64 = number_part / 100;

                num_zero += overflow;
                let add_ons = number_part % 100;
                println!("number: {}, add:{}", number, add_ons);
                number += add_ons;

                num_zero += ((number % 100) == 0) as i64;

                if number > 100 {
                    num_zero += 1;
                    number %= 100;
                }
                println!("{}", num_zero);
            }

            'L' => {
                let overflow: i64 = number_part / 100;
                num_zero += overflow;
                let add_ons = number_part % 100;
                println!("number: {}, sub:{}", number, add_ons);
                number -= add_ons;

                num_zero += ((number % 100) == 0) as i64;
                if number < 0 && number != 0 {
                    num_zero += 1;
                    number += 100;
                } else if number == 0 {
                    number %= 100;
                }
                println!("{}", num_zero);
            }

            _ => println!("broke ahh guy"),
        }
    }
    println!("{}", num_zero);
}
