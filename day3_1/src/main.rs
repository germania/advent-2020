use util::read_arg_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines:Vec<String> = read_arg_file()?
        .map(|x| x.unwrap())
        .collect();

    let mut x = 0;
    let mut trees = 0;


    Ok(())
}
