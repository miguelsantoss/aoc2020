use std::fs::File;
use std::io::{prelude::*, BufReader};

#[inline]
fn add_if_all_fields(uu: i8) -> i32 {
    ((((uu >> 1) ^ 0x7F) << 1) == 0) as i32
}

#[inline]
fn compare(r: u32, min: u32, max: u32) -> i8 {
    (r >= min && r <= max) as i8
}

pub fn day_04() {
    let f = File::open("input/day04").expect("file not found");
    let reader = BufReader::new(f);
    let lines = reader.lines().into_iter();

    let mut s1 = 0;
    let mut s2 = 0;

    let mut fields1: i8 = 0;
    let mut fields2: i8 = 0;
    for line in lines {
        let temp = line.unwrap();

        if temp.len() == 0 {
            s1 = s1 + add_if_all_fields(fields1);
            s2 = s2 + add_if_all_fields(fields2);
            fields1 = 0;
            fields2 = 0;
        } else {
            for f in temp.split(" ") {
                let flag1 = match &f[..3] {
                    "byr" => 1 << 7,
                    "iyr" => 1 << 6,
                    "eyr" => 1 << 5,
                    "hgt" => 1 << 4,
                    "hcl" => 1 << 3,
                    "ecl" => 1 << 2,
                    "pid" => 1 << 1,
                    "cid" => 1,
                    _ => 0x00,
                };

                let flag2 = match &f[..3] {
                    "byr" => {
                        let mut result = 0;
                        let bytes = f.get(4..).unwrap().as_bytes();
                        result += (bytes[0] - b'0') as u32 * 1000;
                        result += (bytes[1] - b'0') as u32 * 100;
                        result += (bytes[2] - b'0') as u32 * 10;
                        result += (bytes[3] - b'0') as u32;
                        let d = compare(result, 1920, 2002);
                        d << 7
                    }
                    "iyr" => {
                        let mut result = 0;
                        let bytes = f.get(4..).unwrap().as_bytes();
                        result += (bytes[0] - b'0') as u32 * 1000;
                        result += (bytes[1] - b'0') as u32 * 100;
                        result += (bytes[2] - b'0') as u32 * 10;
                        result += (bytes[3] - b'0') as u32;
                        let d = compare(result, 2010, 2020);
                        d << 6
                    }
                    "eyr" => {
                        let mut result = 0;
                        let bytes = f.get(4..).unwrap().as_bytes();
                        result += (bytes[0] - b'0') as u32 * 1000;
                        result += (bytes[1] - b'0') as u32 * 100;
                        result += (bytes[2] - b'0') as u32 * 10;
                        result += (bytes[3] - b'0') as u32;
                        let d = compare(result, 2020, 2030);
                        d << 5
                    }
                    "hgt" => {
                        let size = f.len();
                        let unit = f.get(size - 2..).unwrap();
                        let val = f.get(4..size - 2).unwrap();
                        let d: i8 = match unit {
                            "cm" => {
                                let result = val.parse::<u32>().unwrap();
                                compare(result, 150, 193)
                            }
                            "in" => {
                                let result = val.parse::<u32>().unwrap();
                                compare(result, 59, 76)
                            }
                            _ => 0,
                        };

                        d << 4
                    }
                    "hcl" => {
                        let bytes = f.get(4..).unwrap().as_bytes();
                        let first = bytes[0] == b'#';
                        let tail = bytes.get(1..).unwrap();
                        let mut valid = true;
                        for c in tail {
                            if !((*c >= b'0' && *c <= b'9') || (*c >= b'a') && (*c <= b'f')) {
                                valid = false;
                                break;
                            }
                        }

                        ((first && valid) as i8) << 3
                    }
                    "ecl" => {
                        let valid = match f.get(4..).unwrap() {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                            _ => false,
                        };
                        (valid as i8) << 2
                    }
                    "pid" => {
                        let digits = f.get(4..).unwrap().as_bytes();
                        let mut valid = digits.len() == 9;
                        if valid {
                            for c in digits {
                                if *c ^ b'0' > 9 {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        (valid as i8) << 1
                    }
                    "cid" => 1,
                    _ => 0x00,
                };
                fields1 |= flag1;
                fields2 |= flag2;
            }
        }
    }

    s1 = s1 + add_if_all_fields(fields1);
    s2 = s2 + add_if_all_fields(fields2);

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
