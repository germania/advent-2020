use util::read_arg_file;

fn valid(line: &str) -> Option<bool> {
    let mut words = line.split(' ');
    let hilo: Vec<usize> = words
        .next()
        .unwrap()
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let lo = hilo[0];
    let hi = hilo[1];

    let c = &words.next().unwrap().chars().next().unwrap();

    let pass: Vec<char> = words.next().unwrap().chars().collect();

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

fn main() {
    let lines = read_arg_file().unwrap();
    let ret = lines.filter_map(|x| valid(&x.unwrap())).count();

    println!("Found {} valid", ret);
}
