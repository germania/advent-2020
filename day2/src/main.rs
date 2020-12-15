use util::read_arg_file;

struct Line {
    lo: usize,
    hi: usize,
    c: char,
    pass: String,
}

impl Line {
    fn from(line: &str) -> Result<Line, Box<dyn std::error::Error>> {
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
        let pass = &words.next().unwrap();

        Ok(Line {
            lo,
            hi,
            c: *c,
            pass: pass.to_string(),
        })
    }
}

fn valid_ex1(line: &str) -> Option<bool> {
    let l = Line::from(&line).unwrap();

    let instances = l
        .pass
        .chars()
        .fold(0, |a, ch| if l.c == ch { a + 1 } else { a });

    if instances < l.lo || instances > l.hi {
        None
    } else {
        Some(true)
    }
}

fn valid_ex2(line: &str) -> Option<bool> {
    let l = Line::from(&line).unwrap();
    let pass: Vec<char> = l.pass.chars().collect();

    let lo_c = pass[l.lo - 1];
    let hi_c = pass[l.hi - 1];

    if lo_c == l.c && hi_c == l.c {
        return None;
    }

    if lo_c == l.c || hi_c == l.c {
        return Some(true);
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = read_arg_file()?.map(|x| x.unwrap()).collect();

    let ret = &lines.iter().filter_map(|x| valid_ex1(&x)).count();

    println!("Exercise 1: found {} valid", ret);

    let ret = &lines.iter().filter_map(|x| valid_ex2(&x)).count();

    println!("Exercise 2: found {} valid", ret);

    Ok(())
}
