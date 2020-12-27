use std::fs::File;
use std::io::{prelude::*, BufReader};

#[inline]
fn count_bits(d: u32) -> i32 {
    let mut n = d;
    let mut c = 0;
    while n > 0 {
        c += n & 1;
        n = n >> 1;
    }
    c as i32
}

pub fn day_06() {
    let f = File::open("input/day06").expect("file not found");
    let reader = BufReader::new(f);
    let lines = reader.lines().into_iter();

    let mut s1 = 0;
    let mut s2 = 0;

    let mut questions: u32 = 0;
    let mut common_questions: u32 = (1 << (1 + b'z' - b'a')) - 1;

    for line in lines {
        let temp = line.unwrap();

        if temp.len() == 0 {
            s1 = s1 + count_bits(questions);
            questions = 0;

            s2 = s2 + count_bits(common_questions);
            common_questions = (1 << (1 + b'z' - b'a')) - 1;
        } else {
            let bytes = temp.as_bytes();

            let mut answers: u32 = 0;
            for c in bytes {
                answers |= 1 << (c - b'a');
            }

            questions |= answers;
            common_questions &= common_questions & answers;
        }
    }

    s1 = s1 + count_bits(questions);
    s2 = s2 + count_bits(common_questions);

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
