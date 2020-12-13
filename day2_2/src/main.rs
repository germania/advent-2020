use util::read_arg_file;

fn valid(line: &str) -> bool {
    let mut words = line.split(" ");
    if let Some(hilo) = words.next() {
        if let Some(idx) = hilo.find('-') {
            let lo = hilo[0..idx].parse::<usize>().unwrap();
            let hi = hilo[idx + 1..].parse::<usize>().unwrap();

            if let Some(ccolon) = words.next() {
                if let Some(c) = &ccolon[0..&ccolon.len() - 1].chars().next() {
                    if let Some(pass) = words.next() {

                        let lo_c = pass.chars().nth(lo - 1).unwrap();
                        let hi_c = pass.chars().nth(hi - 1).unwrap();

                        if &lo_c == c && &hi_c == c {
                            return false;
                        }

                        if &lo_c == c || &hi_c == c {
                            return true;
                        }

                        return false;
                    }
                }
            }
        }
    }

    false
}

fn main() {
    if let Ok(lines) = read_arg_file() {
        let ret = lines
            .map(|x| match x {
                Ok(x) => valid(&x),
                _ => panic!("Empty line"),
            })
            .fold(0, |a, c| {
                a + match c {
                    true => 1,
                    false => 0,
                }
            });

        println!("Found {} valid", ret);
    }
}
