use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_02() {
    let f = File::open("input/day02").expect("file not found");
    let reader = BufReader::new(f);

    let mut s1 = 0;
    let mut s2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut pass = line.split(" ");

        let mut rule1 = pass
            .next()
            .unwrap()
            .split("-")
            .map(|x| x.parse::<usize>().unwrap());

        let rule2 = pass.next().unwrap().chars().next().unwrap();
        let to_test = pass.next().unwrap();

        let count = to_test.chars().filter(|&x| x == rule2).count();
        let min = rule1.next().unwrap();
        let max = rule1.next().unwrap();

        if count >= min && count <= max {
            s1 = s1 + 1;
        }

        let first = to_test.chars().nth(min - 1).unwrap() == rule2;
        let last = to_test.chars().nth(max - 1).unwrap() == rule2;

        if first ^ last {
            s2 = s2 + 1;
        }
    }

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
