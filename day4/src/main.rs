use util::read_arg_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_arg_file()?;
    let mut ret = 0;

    loop {
        let mut buf = "".to_string();
        while let Some(line) = lines.next() {
            match line.unwrap().as_str() {
                "" => break,
                x => {
                    buf.push_str(" ");
                    buf.push_str(x);
                }
            }
        }

        if &buf == "" {
            break;
        }

        let tally = buf[1..].split(' ').fold(0, |a, x| {
            a + match x.split(':').next().unwrap() {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => 1,
                "cid" => 0,
                _ => 0,
            }
        });

        if tally >= 7
        {
            ret += 1;
        }
    }

    println!("Exercise 1: {} valid passports", ret);

    Ok(())
}
