use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn _read_input() -> HashSet<u32> {
    let mut file = File::open("src/exercises/day1/input.txt").expect("File exists and can be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can read and is valid text");

    let mut numbers = HashSet::default();
    for input in contents.lines() {
        numbers.insert(str::parse::<u32>(input).expect("It is a u32"));
    }

    numbers
}

fn _find_two_that_sum_to(numbers: &HashSet<u32>, sum_to: u32) -> Option<(u32, u32, u32)> {
    for number in numbers {
        if number > &sum_to {
            continue;
        }
        let pair = sum_to - number;
        if numbers.contains(&pair) {
            return Some((*number, pair, number * pair));
        }
    }

    None
}

fn _find_three_that_sum_to(numbers: &HashSet<u32>, sum_to: u32) -> Option<(u32, u32, u32, u32)> {
    for number in numbers {
        let rest = sum_to - number;
        if let Some((number_b, number_c, product)) = _find_two_that_sum_to(&numbers, rest) {
            return Some((*number, number_b, number_c, product * number));
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::exercises::day1::{_find_two_that_sum_to, _read_input, _find_three_that_sum_to};

    #[test]
    fn day_1a_works() {
        let numbers = _read_input();
        let output = _find_two_that_sum_to(&numbers, 2020).expect("There to be two such numbers");
        assert_eq!(output.0 + output.1, 2020);
        assert_eq!(output.0 * output.1, output.2);
        eprintln!("Output 1a: {}", output.2);
    }

    #[test]
    fn day_1b_works() {
        let numbers = _read_input();
        let output = _find_three_that_sum_to(&numbers, 2020).expect("There to be three such numbers");
        assert_eq!(output.0 + output.1 + output.2, 2020);
        assert_eq!(output.0 * output.1 * output.2, output.3);
        eprintln!("Output 1b: {}", output.3);
    }
}
