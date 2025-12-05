use std::{fs, path::Path};

fn main() {
    let path = Path::new("/home/rezo/Workspace/AOC2025/src/day01/input.txt");

    match path.exists() {
        true => function(path),
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
