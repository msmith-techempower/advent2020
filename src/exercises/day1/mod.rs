use std::fs::File;
use std::io::Read;

fn read_input() -> Vec<u32> {
    let mut file = File::open("src/exercises/day1/input.txt").expect("File exists and can be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can read and is valid text");

    let mut numbers = vec![];
    for input in contents.lines() {
        numbers.push(str::parse::<u32>(input).expect("It is a u32"));
    }

    numbers
}

/// `fn find_two_that_sum_to_2020() -> (num_a, num_b, sum)`
/// where `num_a * num_b == 2020`
fn find_two_that_sum_to_2020() -> (u32, u32, u32) {
    let numbers = read_input();

    let mut index = 0;
    while index < numbers.len() {
        let number_a = numbers[index];
        // No reason to check the beginning again.
        let rest = &numbers[index+1..];
        // // SAFETY: no number in the input is > 2020, so this won't underflow
        for number_b in rest {
            if 2020 - number_a == *number_b {
                return (number_a, *number_b, number_a * number_b);
            }
        }
        index += 1;
    }

    panic!("Found no numbers that sum to 2020!");
}

/// `fn find_two_that_sum_to_2020() -> (num_a, num_b, num_c, sum)`
/// where `num_a + num_b + num_c == 2020`
fn find_three_that_sum_to_2020() -> (u32, u32, u32, u32) {
    let numbers = read_input();

    // Performance is bad and I should feel bad.
    for number_a in &numbers {
        for number_b in &numbers {
            for number_c in &numbers {
                if number_a + number_b + number_c == 2020 {
                    // ... but I don't.
                    return (*number_a, *number_b, *number_c, number_a * number_b * number_c);
                }
            }
        }
    }



    panic!("Found no numbers that sum to 2020!");
}

#[cfg(test)]
mod tests {
    use crate::exercises::day1::{find_two_that_sum_to_2020, find_three_that_sum_to_2020};

    #[test]
    fn day_1a_works() {
        let output = find_two_that_sum_to_2020();
        assert_eq!(output.0 + output.1, 2020);
        assert_eq!(output.0 * output.1, output.2);
        eprintln!("Output 1a: {}", output.2);
    }

    #[test]
    fn day_1b_works() {
        let output = find_three_that_sum_to_2020();
        assert_eq!(output.0 + output.1 + output.2, 2020);
        assert_eq!(output.0 * output.1 * output.2, output.3);
        eprintln!("Output 1b: {}", output.3);
    }
}
