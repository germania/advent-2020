use util::read_arg_file;

fn id(code: &str) -> u32 {
    let mut ret = 0;

    for ch in code.chars() {
        ret <<= 1;
        ret |= match ch {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("wat"),
        };
    }

    ret
}

fn seat_id(code: &str) -> u32 {
    let row = id(&code[..7]);
    let seat = id(&code[7..]);

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
