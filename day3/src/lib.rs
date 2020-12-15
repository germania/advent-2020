use util::read_arg_file;

pub fn count_trees(col: usize, row: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let lines: Vec<String> = read_arg_file()?.map(|x| x.unwrap()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < lines.len() {
        trees += match lines[y].chars().nth(x % lines[y].len()).unwrap() {
            '#' => 1,
            _ => 0,
        };

        x += col;
        y += row;
    }

    Ok(trees)
}
