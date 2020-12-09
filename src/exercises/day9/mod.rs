#[cfg(test)]
mod test {
    use crate::utils::read_input;

    fn parse_input(contents: &str) -> Vec<usize> {
        let mut to_ret = vec![];
        for line in contents.lines() {
            to_ret.push(str::parse::<usize>(line).expect("Valid usize"));
        }

        to_ret
    }

    /// The first step of attacking the weakness in the XMAS data is to find
    /// the first number in the list (after the `preamble`) which is not the sum
    /// of two of the `check_back` numbers before it.
    /// Returns the first number that does not have this property.
    fn find_number(xmas: &Vec<usize>, preamble: usize, check_back: usize) -> usize {
        for index in 0..xmas.len() {
            if index < preamble {
                continue;
            }
            let num = xmas.get(index).expect("Valid index");
            let slice = &xmas[index - check_back..index];
            let mut valid = false;
            for index in 0..check_back {
                for check in 0..check_back {
                    if check == index {
                        continue;
                    }
                    if slice[index] + slice[check] == *num {
                        valid = true;
                    }
                }
            }
            if !valid {
                return *num;
            }
        }
        0
    }

    #[test]
    fn example_a() {
        let contents = read_input("src/exercises/day9/example.txt");
        let xmas = parse_input(&contents);
        let value = find_number(&xmas, 5, 5);

        assert_eq!(127, value);
    }

    #[test]
    fn input_a() {
        let contents = read_input("src/exercises/day9/input.txt");
        let xmas = parse_input(&contents);
        let value = find_number(&xmas, 25, 25);

        assert_eq!(393911906, value);
    }

    type LowHigh = (usize, usize);
    fn find_range_than_sums_to(xmas: &Vec<usize>, our_number: usize) -> LowHigh {
        let mut numbers = xmas.clone();
        numbers.reverse();

        let our_index = numbers
            .iter()
            .position(|&x| x == our_number)
            .expect("Xmas contains our number.");

        let mut start_index = 0;
        let mut sum_numbers = vec![];
        let mut sum = 0;
        while sum == 0 {
            start_index += 1;
            for num in &numbers[our_index + start_index..] {
                sum += num;
                sum_numbers.push(num);
                if sum == our_number {
                    break;
                }
                if sum > our_number {
                    sum = 0;
                    sum_numbers.clear();
                    break;
                }
            }
        }
        let mut high = 0;
        let mut low = std::usize::MAX;
        for num in sum_numbers {
            if *num < low {
                low = *num;
            }
            if *num > high {
                high = *num;
            }
        }

        (low, high)
    }

    #[test]
    fn input_b() {
        let contents = read_input("src/exercises/day9/input.txt");
        let xmas = parse_input(&contents);
        let our_number = find_number(&xmas, 25, 25);
        let value = find_range_than_sums_to(&xmas, our_number);

        assert_eq!(59341885, value.0 + value.1);
    }
}
