use util::read_arg_file;

struct Instruction {
    code: String,
    arg: i32,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut words = line.split(' ');
        let code = words.next().unwrap().to_owned();
        let arg_s = words.next().unwrap();

        let sign = &arg_s[..1];
        let mut arg = arg_s[1..].parse::<i32>().unwrap();

        if "-" == sign {
            arg = -arg;
        }

        Instruction { code, arg }
    }
}

fn run_prog(lines: &[Instruction], tweak_idx: Option<usize>) -> bool {
    let mut accumulator = 0;
    let mut cursor: i32 = 0;
    let mut history = vec![];

    let mut current_tweak = 0;

    loop {
        if history.contains(&cursor) {
            println!(
                "Repeating command on line {}, accumulator is {}",
                cursor, accumulator
            );
            return false;
        }

        if cursor as usize >= lines.len() {
            println!("Program exiting cleanly, accumulator is {}", accumulator);
            return true;
        }

        let instruction = &lines[cursor as usize];
        history.push(cursor);

        let mut inc = 1;
        let mut code = instruction.code.as_str();

        match tweak_idx {
            Some(idx) if idx == current_tweak => match code {
                "jmp" => code = "nop",
                "nop" => code = "jmp",
                _ => (),
            },
            _ => (),
        }

        current_tweak += 1;

        match code {
            "jmp" => inc = instruction.arg,
            "acc" => accumulator += instruction.arg,
            _ => (),
        }

        cursor += inc;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<Instruction> = read_arg_file()?
        .map(|x| Instruction::new(&x.unwrap()))
        .collect();

    print!("Exercise 1: ");
    run_prog(&lines, None);

    let mut tweak_idx = 0;

    print!("Exercise 2: ");
    loop {
        if run_prog(&lines, Some(tweak_idx)) {
            break;
        }

        tweak_idx += 1;
    }

    Ok(())
}
