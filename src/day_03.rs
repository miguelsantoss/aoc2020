use std::io::{prelude::*, BufReader};
use std::{fs::File, vec};

pub fn day_03() {
    let f = File::open("input/day03").expect("file not found");
    let reader = BufReader::new(f);
    let mut lines = reader.lines().into_iter();

    let slopes: Vec<i32> = vec![1, 3, 5, 7, 1];
    let mut counts: Vec<i64> = vec![0, 0, 0, 0, 0];
    // let mut curr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut curr: Vec<i32> = vec![1, 3, 5, 7, 1];

    let size = lines.next().unwrap().unwrap().len();
    let mut last_counter = 1;

    for line in lines {
        let temp = line.unwrap();

        let vec: Vec<char> = temp.chars().collect();

        for i in 0..4 {
            let inc = (curr[i] as usize) % size;
            let curr_char = vec[inc];

            if curr_char == '#' {
                counts[i] = counts[i] + 1;
            }

            curr[i] = curr[i] + slopes[i];
        }

        if last_counter == 2 {
            let i = 4;
            let inc = (curr[i] as usize) % size;
            let curr_char = vec[inc];

            if curr_char == '#' {
                counts[i] = counts[i] + 1;
            }

            curr[i] = curr[i] + slopes[i];
            last_counter = 0;
        }

        last_counter = last_counter + 1;
    }

    let s2 = counts.iter().fold(1, |acc, &x| acc * x);

    println!("s1 = {}", counts[1]);
    println!("s2 = {}", s2);
}
