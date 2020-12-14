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

    let pass: Vec<char> = words.next()?.chars().collect();

    let lo_c = &pass[lo - 1];
    let hi_c = &pass[hi - 1];

    if lo_c == c && hi_c == c {
        return None;
    }

    if lo_c == c || hi_c == c {
        return Some(true);
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_arg_file()?;
    let ret = lines.filter_map(|x| valid(&x.unwrap())).count();

    println!("Found {} valid", ret);
    Ok(())
}
