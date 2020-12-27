use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_05() {
    let f = File::open("input/day05").expect("file not found");
    let reader = BufReader::new(f);
    let lines = reader.lines().into_iter();

    let mut s1 = 0;
    let mut s2 = 0;

    for line in lines {
        let temp = line.unwrap();

        let row = temp.get(..7).unwrap().as_bytes();
        let col = temp.get(7..).unwrap().as_bytes();

        let curr_row = ((row[0] ^ 0x46) << 4)
            | ((row[1] ^ 0x46) << 3)
            | ((row[2] ^ 0x46) << 2)
            | ((row[3] ^ 0x46) << 1)
            | ((row[4] ^ 0x46) << 0)
            | ((row[5] ^ 0x46) >> 1)
            | ((row[6] ^ 0x46) >> 2);

        let curr_col =
            (((col[0] ^ 0x4C) >> 4) << 2) | (((col[1] ^ 0x4C) >> 4) << 1) | ((col[2] ^ 0x42) >> 4);

        let id: i32 = ((curr_row as i32) << 3) + curr_col as i32;

        s1 = s1 ^ ((s1 ^ id) & -((s1 < id) as i32));
        s2 = s2 ^ id;
    }

    for i in 1..(s1 + 1) {
        s2 = s2 ^ i;
    }

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
