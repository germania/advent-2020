use util::{read_arg_file};

fn main() {
    if let Ok(lines) = read_arg_file() {
        let mut numbers = vec![];

        for line in lines {
            if let Ok(n) = line {
                numbers.push(n.parse::<usize>().unwrap());
            }
        }

        for (idx, i) in (&numbers[..]).iter().enumerate() {
            for j in &numbers[idx + 1..] {
                let n = i + j;
                if n == 2020 {
                    println!("{} + {} = 2020, answer is {}", i, j, i * j);
                    return;
                }
            }
        }
    } else {
        println!("Uh oh");
    }
}
