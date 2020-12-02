type TwoNumbersAndTheirProduct = (u32, u32, u32);
/// Finds two numbers in `numbers` that sum to `sum_to` and returns those two
/// numbers and their product (in that order).
pub fn find_two_that_sum_to(numbers: &[u32], sum_to: u32) -> Option<TwoNumbersAndTheirProduct> {
    for (index, number) in numbers.iter().filter(|num| num < &&sum_to).enumerate() {
        let difference = sum_to - number;
        // check to see if `difference` is in `numbers` with this `number` ignored.
        if numbers[index+1..].contains(&difference) {
            return Some((*number, difference, number * difference));
        }
    }

    None
}

type ThreeNumbersAndTheirProduct = (u32, u32, u32, u32);
/// Finds three numbers in `numbers` that sum to `sum_to` and returns those
/// three numbers and their product (in that order).
pub fn find_three_that_sum_to(numbers: &[u32], sum_to: u32) -> Option<ThreeNumbersAndTheirProduct> {
    for (index, number) in numbers.iter().filter(|num| num < &&sum_to).enumerate() {
        let difference = &sum_to - number;
        // check to see if `difference` is in `numbers` with this `number` ignored.
        if let Some((number_b, number_c, product)) = find_two_that_sum_to(&numbers[index+1..], difference) {
            return Some((*number, number_b, number_c, product * number));
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::exercises::day1::{find_two_that_sum_to, find_three_that_sum_to};
    use std::fs::File;
    use std::io::Read;

    fn read_input() -> Vec<u32> {
        let mut file = File::open("src/exercises/day1/input.txt").expect("File exists and can be opened");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can read and is valid text");

        let mut numbers =  vec![];
        for input in contents.lines() {
            numbers.push(str::parse::<u32>(input).expect("It is a u32"));
        }

        numbers
    }

    #[test]
    fn day_1a_works() {
        let numbers = read_input();
        let output = find_two_that_sum_to(&numbers, 2020).expect("There to be two such numbers");
        assert_eq!(output.0 + output.1, 2020);
        assert_eq!(output.0 * output.1, output.2);
        assert_eq!(646779, output.2);
    }

    #[test]
    fn day_1b_works() {
        let numbers = read_input();
        let output = find_three_that_sum_to(&numbers, 2020).expect("There to be three such numbers");
        assert_eq!(output.0 + output.1 + output.2, 2020);
        assert_eq!(output.0 * output.1 * output.2, output.3);
        assert_eq!(246191688, output.3);
    }
}
