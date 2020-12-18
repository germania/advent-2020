use util::read_arg_file;

struct Bag {
    color: String,
    can_contain: Vec<(i32, String)>,
}

impl Bag {
    fn new(line: String) -> Option<Bag> {
        let mut words = line.split(' ');
        let color = vec![words.next()?, words.next()?].join(" ");
        let mut can_contain = vec![];

        words.nth(1);

        loop {
            let n = words.next()?;

            if "no" == n {
                break;
            }

            let amt = n.parse::<i32>().unwrap();
            let my_color = vec![words.next()?, words.next()?].join(" ");

            can_contain.push((amt, my_color));

            let bags = words.next()?;
            if !bags.ends_with(',') {
                break;
            }
        }

        Some(Bag { color, can_contain })
    }
}

fn check_contents(search: &str, bag: &Bag, bags: &[Bag]) -> bool {
    for child in &bag.can_contain {
        if child.1 == search {
            return true;
        }
    }

    let child_bags: Vec<&Bag> = bag
        .can_contain
        .iter()
        .map(|b| bags.iter().find(|bb| bb.color == b.1).unwrap())
        .collect();

    for child_bag in child_bags {
        if check_contents(search, child_bag, bags) {
            return true;
        }
    }

    false
}

fn count_contents(bag: &Bag, bags: &[Bag]) -> i32 {
    bag.can_contain
        .iter()
        .map(|b| {
            (
                b.0,
                bags.iter().find(|bb| bb.color == b.1).unwrap(),
            )
        })
        .fold(0, |a, bb| a + bb.0 * count_contents(bb.1, bags))
        + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bags: Vec<Bag> = read_arg_file()?
        .map(|line| Bag::new(line.unwrap()).unwrap())
        .collect();

    let search = "shiny gold";
    let mut ret = 0;
    for bag in &bags {
        if check_contents(search, bag, &bags) {
            ret += 1;
        }
    }

    println!("Exercise 1: {} bags can contain {}", ret, search);

    let mybag = &bags.iter().find(|bb| bb.color == search).unwrap();
    let total = count_contents(mybag, &bags) - 1;

    println!("Exercise 2: {} bags can fit in a {} bag", total, search);

    Ok(())
}
