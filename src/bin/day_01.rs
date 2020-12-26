use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let f = File::open("../../input/day01").expect("file not found");
    let reader = BufReader::new(f);
    let mut data = HashSet::new();

    for line in reader.lines() {
        data.insert(line?.parse::<i32>().unwrap());
    }

    let mut s1 = 0;
    let mut s2 = 0;
    for i in &data {
        let n: i32 = 2020 - i;
        if data.contains(&n) {
            s1 = i * n
        }

        for j in &data {
            for k in &data {
                if i + j + k == 2020 {
                    s2 = i * j * k
                }
            }
        }
    }

    println!("solution 1: {}", s1);
    println!("solution 2: {}", s2);

    Ok(())
}
