use util::read_arg_file;

use std::{
    fs::File,
    io::{BufReader, Lines},
};

fn num_between(v: &str, lo: u32, hi: u32) -> i32 {
    let iv = v.parse::<u32>().unwrap();
    if iv > lo && iv < hi {
        1
    } else {
        0
    }
}

fn valid_hex(v: &str) -> i32 {
    let u = &v[..1];
    let n = &v[1..];

    let c = u == "#";
    let x = n
        .chars()
        .filter_map(|z| match "0123456789abcdefABCDEF".contains(z) {
            true => Some(true),
            false => None,
        })
        .count()
        == 6;

    if c && x {
        1
    } else {
        0
    }
}

fn read(lines: &mut Lines<BufReader<File>>) -> Vec<String> {
    let mut ret = vec![];

    loop {
        let mut buf = String::new();
        while let Some(line) = lines.next() {
            match line.unwrap().as_str() {
                "" => break,
                x => {
                    buf.push_str(" ");
                    buf.push_str(x);
                }
            }
        }

        if &buf == "" {
            break;
        }

        ret.push(buf[1..].to_owned());
    }

    ret
}

fn exercise_1(passports: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let tally = passports
        .iter()
        .map(|p| {
            p.split(' ').fold(0, |a, x| {
                a + match x.split(':').next().unwrap() {
                    "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => 1,
                    _ => 0,
                }
            })
        })
        .filter(|x| *x >= 7)
        .count();

    println!("Exercise 1: {} valid passports", tally);

    Ok(())
}

fn exercise_2(passports: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let tally = passports
        .iter()
        .map(|p| {
            p.split(' ').fold(0, |a, x| {
                let mut kv = x.split(':');
                let key = kv.next().unwrap();
                let value = kv.next().unwrap();

                a + match key {
                    "byr" => num_between(value, 1919, 2003),
                    "iyr" => num_between(value, 2009, 2021),
                    "eyr" => num_between(value, 2019, 2031),
                    "hgt" => {
                        let l = value.len();
                        let n = &value[..l - 2];
                        let u = &value[l - 2..];

                        match u {
                            "cm" => num_between(n, 149, 194),
                            "in" => num_between(n, 58, 77),
                            _ => 0,
                        }
                    }
                    "hcl" => valid_hex(value),
                    "ecl" => {
                        if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                            1
                        } else {
                            0
                        }
                    }
                    "pid" => match value.chars().filter(|x| x.is_numeric()).count() {
                        9 => 1,
                        _ => 0,
                    },
                    _ => 0,
                }
            })
        })
        .filter(|x| *x >= 7)
        .count();

    println!("Exercise 2: {} valid passports", tally);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passports = read(&mut read_arg_file()?);
    exercise_1(&passports)?;
    exercise_2(&passports)
}
