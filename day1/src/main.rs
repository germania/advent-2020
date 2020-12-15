use util::read_arg_file;

fn ex1(numbers: &Vec<(usize, usize)>) {
    for i in numbers {
        for j in &numbers[i.0..] {
            let n = i.1 + j.1;
            if n == 2020 {
                println!("Exercise 1: {} + {} = 2020, answer is {}", i.1, j.1, i.1 * j.1);
                return;
            }
        }
    }
}

fn ex2(numbers: &Vec<(usize, usize)>) {
    for i in numbers {
        for j in &numbers[i.0..] {
            for k in &numbers[j.0..] {
                let n = i.1 + j.1 + k.1;
                if n == 2020 {
                    println!(
                        "Exercise 2: {} + {} + {} = 2020, answer is {}",
                        i.1,
                        j.1,
                        k.1,
                        i.1 * j.1 * k.1
                    );
                    return;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_arg_file()?;
    let numbers: Vec<(usize, usize)> = lines
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .enumerate()
        .collect();

    ex1(&numbers);
    ex2(&numbers);

    Ok(())
}
