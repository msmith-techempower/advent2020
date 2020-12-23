#[cfg(test)]
mod test {
    use crate::exercises::day18::test::Op::{Add, Multiply};
    use crate::utils::read_input;

    enum Op {
        Multiply,
        Add,
    }

    fn evaluate(expression: &str) -> u64 {
        let mut stack = 0;
        let mut op = None;
        let mut sub_expression_stack = 0;
        for (index, char) in expression.chars().enumerate() {
            match char {
                ' ' => {} // No-op
                '*' => {
                    if sub_expression_stack == 0 {
                        op = Some(Multiply);
                    }
                }
                '+' => {
                    if sub_expression_stack == 0 {
                        op = Some(Add);
                    }
                }
                '(' => {
                    if sub_expression_stack == 0 {
                        let value = evaluate(&expression[index + 1..]);
                        match op {
                            None => stack = value,
                            Some(Multiply) => stack *= value,
                            Some(Add) => stack += value,
                        }
                        op = None;
                    }
                    sub_expression_stack += 1;
                }
                ')' => {
                    if sub_expression_stack == 0 {
                        // We're done with this evaluation
                        return stack;
                    } else {
                        sub_expression_stack -= 1;
                    }
                }
                digit => {
                    if sub_expression_stack == 0 {
                        let digit = digit.to_digit(10).expect("Valid digit") as u64;
                        match op {
                            None => stack = digit,
                            Some(Multiply) => stack *= digit,
                            Some(Add) => stack += digit,
                        }
                        op = None;
                    }
                }
            }
        }
        stack
    }

    #[test]
    fn example() {
        let easy_example1 = "2 + 2";
        assert_eq!(4, evaluate(easy_example1));
        let easy_example2 = "2 * 3";
        assert_eq!(6, evaluate(easy_example2));
        let easy_example3 = "2 + 2 * 3";
        assert_eq!(12, evaluate(easy_example3));
        let easy_example4 = "2 * 3 + 2";
        assert_eq!(8, evaluate(easy_example4));

        let example1 = "2 * 3 + (4 * 5)";
        assert_eq!(26, evaluate(example1));
        let example2 = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(437, evaluate(example2));
        let example3 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(12240, evaluate(example3));
        let example4 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(13632, evaluate(example4));
    }

    #[test]
    fn input() {
        let contents = read_input("src/exercises/day18/input.txt");
        let mut accumulator = 0;
        for line in contents.lines() {
            accumulator += evaluate(line);
        }

        assert_eq!(4_491_283_311_856, accumulator);
    }

    fn evaluate_split(expression: &str) -> u64 {
        let mut to_ret = 1;

        let mut expression = expression.to_string();
        while expression.contains('(') {
            let mut open_count = 0;
            let mut inner_most_open_paren_index = 0;
            let mut inner_most_open_paren_depth = 0;
            let mut inner_most_close_paren_index = 0;
            for (index, char) in expression.chars().enumerate() {
                if char == '(' && open_count >= inner_most_open_paren_depth {
                    open_count += 1;
                    inner_most_open_paren_index = index;
                    inner_most_open_paren_depth = open_count;
                }
                if char == ')' && inner_most_close_paren_index == 0 {
                    open_count -= 1;
                    inner_most_close_paren_index = index;
                }
            }
            let sub_expression =
                &expression[inner_most_open_paren_index..inner_most_close_paren_index + 1];
            let sub_evaluation = evaluate_split(&sub_expression[1..sub_expression.len() - 1]);
            expression = expression.replacen(sub_expression, &format!("{}", sub_evaluation), 1);
        }
        while expression.contains('+') {
            let mut reduced_expression = expression.clone();
            let tokens = expression.split(' ').collect::<Vec<&str>>();
            for (index, char) in tokens.iter().enumerate() {
                if *char == "+" {
                    let sub_expression = format!(
                        "{} + {}",
                        tokens.get(index - 1).expect("Valid index"),
                        tokens.get(index + 1).expect("Valid index")
                    );
                    let val1 = str::parse::<u64>(tokens.get(index - 1).expect("Valid index"))
                        .expect("Valid integer");
                    let val2 = str::parse::<u64>(tokens.get(index + 1).expect("Valid index"))
                        .expect("Valid integer");
                    reduced_expression = reduced_expression.replacen(
                        &sub_expression,
                        &format!("{}", val1 + val2),
                        1,
                    );
                    break;
                }
            }
            expression = reduced_expression.clone();
        }
        // Reduced expression should just be multiplications now
        for char in expression.split(' ') {
            if char != "*" {
                let val = str::parse::<u64>(&format!("{}", char)).expect("Valid integer");
                to_ret *= val;
            }
        }

        to_ret
    }

    #[test]
    fn part_b() {
        assert_eq!(563, evaluate_split("9 + 5 + 9 + 540"));
        assert_eq!(6, evaluate_split("2 * 3"));
        assert_eq!(5, evaluate_split("2 + 3"));
        assert_eq!(20, evaluate_split("4 * 2 + 3"));
        assert_eq!(12, evaluate_split("4 + 2 * 2"));
        assert_eq!(8, evaluate_split("4 + (2 * 2)"));
        assert_eq!(12, evaluate_split("4 + (2 + 2 * 2)"));
        assert_eq!(12, evaluate_split("2 + 2 * 3"));
        assert_eq!(10, evaluate_split("2 * 3 + 2"));
        assert_eq!(46, evaluate_split("2 * 3 + (4 * 5)"));
        assert_eq!(1445, evaluate_split("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            evaluate_split("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            evaluate_split("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
        assert_eq!(52662422800, evaluate_split("((4 * 5 * 2) + (6 + 4 + 5 + 5 + 7 * 5) * 7 + 5 + 9) + (2 + 8 + (2 + 4) * 8) * 4 + (3 + (7 + 2 * 9) * 2 * 6 + 3 + (2 * 5 + 5 + 4 * 4 + 7)) * (7 + 6 * 4) * 5"));
        assert_eq!(126824333202, evaluate_split("7 * ((8 * 3 + 3 + 9 + 9 * 8) + 3 * 5 + (2 * 3) * 6) + ((3 + 5 + 8 * 8 * 9) + 4 * 7 + 3 * 9 + 6) * 3 * 3 + (2 + 3 + (9 * 7 + 6) * 5 * 9 * 4)"));
        assert_eq!(566, evaluate_split("(9 + 5 + 9 + (5 * 5 + 7 * 9)) + 3"));

        let contents = read_input("src/exercises/day18/input.txt");
        let mut accumulator = 0;
        for line in contents.lines() {
            let val = evaluate_split(line);
            accumulator += val;
        }

        // 98_767_499_909_577 - too high
        // 97_648_473_291_891 - too high
        // 68_904_741_422_845 - too high
        // 68_852_578_642_795 - bad
        // 68_852_578_641_904
        assert_eq!(68_852_578_641_904, accumulator);
    }
}
