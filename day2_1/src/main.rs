use util::read_arg_file;

fn valid(line: &str) -> bool {
    let mut words = line.split(" ");
    if let Some(hilo) = words.next() {
        if let Some(idx) = hilo.find('-') {
            let lo = hilo[0..idx].parse::<i32>().unwrap();
            let hi = hilo[idx + 1..].parse::<i32>().unwrap();

            if let Some(ccolon) = words.next() {
                if let Some(c) = &ccolon[0..&ccolon.len() - 1].chars().next() {
                    if let Some(pass) = words.next() {
                        let instances =
                            pass.chars()
                                .fold(0, |a, ch| if c == &ch { a + 1 } else { a });

                        if instances < lo {
                            return false;
                        } else if instances > hi {
                            return false;
                        }

                        return true;
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
