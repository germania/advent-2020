fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exercise 1: {} trees", day3::count_trees(3, 1)?);

    let part2 = day3::count_trees(1, 1)?
        * day3::count_trees(3, 1)?
        * day3::count_trees(5, 1)?
        * day3::count_trees(7, 1)?
        * day3::count_trees(1, 2)?;

    println!("Exercise 2: {} trees", part2);

    Ok(())
}
