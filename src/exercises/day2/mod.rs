#[derive(Debug)]
pub struct Day2APasswordPolicy {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

pub fn count_valid_day_2a_passwords(policies: &[Day2APasswordPolicy]) -> u32 {
    let mut to_ret: u32 = 0;
    for policy in policies {
        let count = policy.password.matches(policy.char).count();
        if count <= policy.max && count >= policy.min {
            to_ret += 1;
        }
    }

    to_ret
}

pub fn count_valid_day_2b_passwords(policies: &[Day2BPasswordPolicy]) -> u32 {
    let mut to_ret: u32 = 0;
    for policy in policies {
        let pos_1 = policy.password.chars().nth(policy.pos_1 - 1).expect("String is long enough");
        let pos_2 = policy.password.chars().nth(policy.pos_2 - 1).expect("String is long enough");
        if pos_1 == policy.char && pos_2 != policy.char {
            to_ret += 1;
        } else if pos_1 != policy.char && pos_2 == policy.char {
            to_ret += 1;
        }
    }

    to_ret
}

#[derive(Debug)]
pub struct Day2BPasswordPolicy {
    pos_1: usize,
    pos_2: usize,
    char: char,
    password: String,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::exercises::day2::{Day2APasswordPolicy, count_valid_day_2a_passwords, Day2BPasswordPolicy, count_valid_day_2b_passwords};

    fn read_input() -> String {
        let mut file = File::open("src/exercises/day2/input.txt").expect("File exists and can be opened");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can read and is valid text");

        contents
    }

    fn process_day_2a_input(contents: &str) -> Vec<Day2APasswordPolicy> {
        let mut policies = vec![];
        // each line will be of the format:
        //  [min]-[max] [char]: [password]
        for line in contents.lines() {
            let min_rest: Vec<&str> = line.split_terminator('-').collect();
            let max_char_password: Vec<&str> = min_rest.get(1).expect("Index 1 is valid").split(' ').collect();
            policies.push(Day2APasswordPolicy {
                min: str::parse::<usize>(min_rest.get(0).expect("Index 0 is valid")).expect("It is a u8"),
                max: str::parse::<usize>(max_char_password.get(0).expect("Index 0 is valid")).expect("It is a u8"),
                char: str::parse::<char>(max_char_password.get(1).expect("Index 1 is valid").strip_suffix(':').expect("It ends with a colon")).expect("It is a valid char"),
                password: max_char_password.get(2).expect("Index 2 is valid").to_string(),
            });
        }

        policies
    }

    fn process_day_2b_input(contents: &str) -> Vec<Day2BPasswordPolicy> {
        let mut policies = vec![];
        // each line will be of the format:
        //  [min]-[max] [char]: [password]
        for line in contents.lines() {
            let min_rest: Vec<&str> = line.split_terminator('-').collect();
            let max_char_password: Vec<&str> = min_rest.get(1).expect("Index 1 is valid").split(' ').collect();
            policies.push(Day2BPasswordPolicy {
                pos_1: str::parse::<usize>(min_rest.get(0).expect("Index 0 is valid")).expect("It is a u8"),
                pos_2: str::parse::<usize>(max_char_password.get(0).expect("Index 0 is valid")).expect("It is a u8"),
                char: str::parse::<char>(max_char_password.get(1).expect("Index 1 is valid").strip_suffix(':').expect("It ends with a colon")).expect("It is a valid char"),
                password: max_char_password.get(2).expect("Index 2 is valid").to_string(),
            });
        }

        policies
    }

    #[test]
    fn day2a_examples_work() {
        let policies = vec![Day2APasswordPolicy {
            min: 1,
            max: 3,
            char: 'a',
            password: "abcde".to_string(),
        }, Day2APasswordPolicy {
            min: 1,
            max: 3,
            char: 'b',
            password: "cdefg".to_string(),
        }, Day2APasswordPolicy {
            min: 2,
            max: 9,
            char: 'c',
            password: "ccccccccc".to_string(),
        }];

        assert_eq!(2, count_valid_day_2a_passwords(&policies));
    }

    #[test]
    fn day_2a_works() {
        let contents = read_input();
        let policies = process_day_2a_input(&contents);

        let count = count_valid_day_2a_passwords(&policies);
        assert_eq!(474, count);
    }

    #[test]
    fn day2b_examples_work() {
        let policies = vec![Day2BPasswordPolicy {
            pos_1: 1,
            pos_2: 3,
            char: 'a',
            password: "abcde".to_string(),
        }, Day2BPasswordPolicy {
            pos_1: 1,
            pos_2: 3,
            char: 'b',
            password: "cdefg".to_string(),
        }, Day2BPasswordPolicy {
            pos_1: 1,
            pos_2: 3,
            char: 'c',
            password: "ccccccccc".to_string(),
        }];

        assert_eq!(1, count_valid_day_2b_passwords(&policies));
    }

    #[test]
    fn day_2b_works() {
        let contents = read_input();
        let policies = process_day_2b_input(&contents);

        let count = count_valid_day_2b_passwords(&policies);
        assert_eq!(745, count);
    }
}