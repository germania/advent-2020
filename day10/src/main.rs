use util::read_arg_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut adapters: Vec<u64> = read_arg_file()?
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect();

    &adapters.sort();

    let mut incrementsby1 = 1;
    let mut incrementsby3 = 1;

    for (i, x) in adapters[1..].iter().enumerate() {
        let y = adapters[i];
        match x - y {
            1 => incrementsby1 += 1,
            3 => incrementsby3 += 1,
            _ => ()
        }
    }

    let answer = incrementsby1 * incrementsby3;

    println!("{}x{}={}", incrementsby1, incrementsby3, answer);

    Ok(())

}
