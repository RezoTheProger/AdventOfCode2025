use std::{fs, path::Path};

fn main() {
    let path = Path::new("/home/rezo/Workspace/AOC2025/src/day02/input.txt");

    match path.exists() {
        true => function1(path),
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
            //UR HERE-----------------
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
