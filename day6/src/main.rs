use util::read_arg_file;

use std::collections::HashMap;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Lines},
};

fn exercise_1(lines: &mut Lines<BufReader<File>>) -> Result<(), Box<dyn Error>> {
    let mut ret = 0;

    loop {
        let mut buf = String::new();
        while let Some(line) = lines.next() {
            match line.unwrap().as_str() {
                "" => break,
                x => {
                    buf.push_str(x);
                }
            }
        }

        if &buf == "" {
            break;
        }

        let mut seen: HashMap<char, u32> = HashMap::new();

        for ch in buf.chars() {
            if ch.is_alphabetic() {
                let i = match seen.get(&ch) {
                    Some(i) => i + 1,
                    None => 0,
                };
                seen.insert(ch, i + 1);
            }
        }

        ret += seen.values().fold(0, |a, _| a + 1);
    }

    println!("Exercise 1: {}", ret);

    Ok(())
}

fn exercise_2(lines: &mut Lines<BufReader<File>>) -> Result<(), Box<dyn Error>> {
    let mut ret = 0;

    loop {
        let mut members = 0;
        let mut seen: HashMap<char, u32> = HashMap::new();

        while let Some(line) = lines.next() {
            match line.unwrap().as_str() {
                "" => break,
                x => {
                    members += 1;
                    for ch in x.chars().filter(|ch| ch.is_alphabetic()) {
                        let x = seen.get(&ch);
                        let i = match x {
                            Some(&n) => n,
                            None => 0,
                        };
                        seen.insert(ch, i + 1);
                    }
                }
            }
        }

        if members == 0 {
            break;
        }

        ret += seen
            .values()
            .filter(|&x| *x == members)
            .fold(0, |a, _| a + 1);
    }

    println!("Exercise 2: {}", ret);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = read_arg_file()?;
    exercise_1(&mut lines)?;

    let mut lines = read_arg_file()?;
    exercise_2(&mut lines)
}
