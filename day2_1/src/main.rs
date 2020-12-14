use util::read_arg_file;

fn valid(line: &str) -> Option<bool> {
    let mut words = line.split(' ');
    let hilo: Vec<usize> = words
        .next()?
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let lo = hilo[0];
    let hi = hilo[1];

    let c = &words.next()?.chars().next()?;

    let instances = words
        .next()?
        .chars()
        .fold(0, |a, ch| if c == &ch { a + 1 } else { a });

    if instances < lo || instances > hi {
        None
    } else {
        Some(true)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_arg_file()?;
    let ret = lines.filter_map(|x| valid(&x.unwrap())).count();

    println!("Found {} valid", ret);
    Ok(())
}
