use std::env;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

pub fn file_arg() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    if let Some(arg) = args.last() {
        buf.push_str(arg);
        return Some(buf)
    }
    None
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_arg_file() -> io::Result<io::Lines<io::BufReader<File>>> {
    match file_arg() {
        Some(file) => read_lines(file),
        None => Err(Error::new(ErrorKind::Other, "No file"))
    }
}
