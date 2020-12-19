use util::read_arg_file;

fn find_missing(numbers: &[u64], window_len: usize) -> Option<u64> {
    let mut windows = numbers.windows(window_len);

    for sum in &numbers[window_len..] {
        let window = windows.next().unwrap();
        let mut found = false;

        for a in window {
            for b in window {
                if a + b == *sum {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return Some(*sum); // hur
        }
    }

    None
}

fn find_sequence(numbers: &[u64], sum: u64, window_len: usize) -> Option<&[u64]> {
    let windows = numbers.windows(window_len);

    for window in windows {
        let tally: u64 = window.iter().sum();
        if tally == sum {
            return Some(window);
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers: Vec<u64> = read_arg_file()?
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect();

    let missingno = find_missing(&numbers, 25).unwrap();
    println!("Exercise 1: couldn't find {}", missingno);

    for window_len in 2..numbers.len() - 1 {
        if let Some(window) = find_sequence(&numbers, missingno, window_len) {
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            let answer = min + max;

            println!("Exercise 2: sum of first and last is {}", answer);

            break;
        }
    }

    Ok(())
}
