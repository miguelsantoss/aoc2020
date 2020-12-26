use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_01() {
    let f = File::open("input/day01").expect("file not found");
    let reader = BufReader::new(f);
    let mut data = Vec::with_capacity(210);

    for line in reader.lines() {
        data.push(line.unwrap().parse::<i32>().unwrap());
    }

    data.sort();

    let mut s1 = 0;
    let mut s2 = 0;

    for i in data.iter() {
        let n: i32 = 2020 - i;
        if data.contains(&n) {
            s1 = i * n;
            break;
        }
    }

    let size = data.len();
    for i in 0..size {
        for j in i..size {
            for k in j..size {
                if data[i] + data[j] + data[k] == 2020 {
                    s2 = data[i] * data[j] * data[k];
                    break;
                }
            }
        }
    }

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
