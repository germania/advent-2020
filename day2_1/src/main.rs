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

    let instances = words
        .next()
        .unwrap()
        .chars()
        .fold(0, |a, ch| if c == &ch { a + 1 } else { a });

    if instances < lo || instances > hi {
        None
    } else {
        Some(true)
    }
}

fn main() {
    let lines = read_arg_file().unwrap();
    let ret = lines.filter_map(|x| valid(&x.unwrap())).count();

    println!("Found {} valid", ret);
}
