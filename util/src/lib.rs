use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn file_arg() -> Option<String> {
    Some(env::args().last()?)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_arg_file() -> io::Result<io::Lines<io::BufReader<File>>> {
    read_lines(file_arg().unwrap())
}
