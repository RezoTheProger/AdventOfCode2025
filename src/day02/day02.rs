use std::{fs, path::Path};

fn main() {
    let path = Path::new("/home/rezo/Workspace/AOC2025/src/day02/input.txt");

    match path.exists() {
        true => function2(path),
        false => println!("File does NOT exist!"),
    }
}

fn function1(path: &Path) {
    let file: String = fs::read_to_string(path).expect("lmao something went wrong");
    let mut store: i64 = 0;
    let intervals: Vec<[&str; 2]> = file
        .split(',')
        .map(|p| {
            let mut parts = p.trim().split('-').map(|x| x.trim());
            [parts.next().unwrap(), parts.next().unwrap()]
        })
        .collect();
    for [start, end] in &intervals {
        let first: i64 = start.parse().unwrap();
        let last: i64 = end.parse().unwrap();

        for num in first..=last {
            let string: &str = &num.to_string();
            if string.len() % 2 == 0 {
                let (part1, part2) = string.split_at(string.len() / 2);
                if part1 == part2 {
                    store += num;
                }
            }
        }
    }
    println!("store: {}", store);
}

//UNOPTIMIZED, I KNOW ITS SHIT
fn function2(path: &Path) {
    let file: String = fs::read_to_string(path).expect("lmao something went wrong");
    let mut store: i64 = 0;
    let intervals: Vec<[&str; 2]> = file
        .split(',')
        .map(|p| {
            let mut parts = p.trim().split('-').map(|x| x.trim());
            [parts.next().unwrap(), parts.next().unwrap()]
        })
        .collect();
    for [start, end] in &intervals {
        let first: i64 = start.parse().unwrap();
        let last: i64 = end.parse().unwrap();

        for num in first..=last {
            let string = num.to_string(); // OWN the string
            let s = string.as_str();
            let length = s.len();
            let divisors: Vec<usize> = list_factors(length);
            let chunks_per_divisor: Vec<Vec<&str>> = divisors
                .iter()
                .map(|&d| {
                    (0..length)
                        .step_by(d)
                        .map(|i| &s[i..i + d])
                        .collect::<Vec<&str>>()
                })
                .collect();
            for chunks in &chunks_per_divisor {
                if chunks.len() > 1 && chunks.iter().all(|x| *x == chunks[0]) {
                    store += num;
                    break;
                }
            }
        }
    }
    println!("store: {}", store);
}

fn list_factors(number: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();
    for i in 1..=number {
        if number % i == 0 {
            factors.push(i);
        }
    }
    factors
}
