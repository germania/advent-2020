use util::read_arg_file;

fn seat_id(code: &str) -> u32 {
    let mut row = 0;
    let mut seat = 0;

    for ch in code[..7].chars() {
        row <<= 1;
        row |= match ch {
            'F' => 0,
            'B' => 1,
            _ => panic!("wat"),
        };
    }

    for ch in code[7..].chars() {
        seat <<= 1;
        seat |= match ch {
            'L' => 0,
            'R' => 1,
            _ => panic!("wat"),
        }
    }

    row * 8 + seat
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ids: Vec<u32> = read_arg_file()?.map(|x| seat_id(&x.unwrap())).collect();

    ids.sort();

    println!("Exercise 1: {}", ids.last().unwrap());

    for (i, id) in ids[1..].iter().enumerate() {
        if ids[i] != id - 1 {
            println!("Exercise 2: {} is missing", id - 1);
            break;
        }
    }

    Ok(())
}
