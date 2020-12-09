#[cfg(test)]
mod tests {
    use crate::exercises::day8::tests::Op::{ACC, JMP, NOP};
    use crate::utils::read_input;

    #[derive(PartialEq, Debug, Clone, Copy)]
    enum Op {
        NOP, // no-op
        ACC, // increase global accumulator by X
        JMP, // relative jump (1-indexed)
    }
    impl Op {
        fn from(str: &str) -> Self {
            match str {
                "nop" => NOP,
                "acc" => ACC,
                "jmp" => JMP,
                _ => panic!("Invalid operation: {}", str),
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Operation {
        op: Op,
        argument: i32,
        seen: bool,
    }

    fn process_input_into_operations(contents: &str) -> Vec<Operation> {
        let mut to_ret = vec![];
        for line in contents.lines() {
            let parts = line.split(" ").collect::<Vec<&str>>();
            to_ret.push(Operation {
                op: Op::from(parts.get(0).expect("Operation part")),
                argument: str::parse::<i32>(parts.get(1).expect("Argument part"))
                    .expect("Argument is valid i32"),
                seen: false,
            });
        }

        to_ret
    }

    fn run_operations(operations: &mut Vec<Operation>) -> (i32, bool) {
        let mut accumulator = 0;

        let mut index = 0;
        let mut exited_successfully = false;
        loop {
            if index == operations.len() {
                exited_successfully = true;
                break;
            }
            if let Some(operation) = operations.get_mut(index) {
                // dbg!(&operation);
                if operation.seen {
                    break;
                }
                operation.seen = true;
                match operation.op {
                    NOP => {}
                    ACC => accumulator += operation.argument,
                    JMP => index = (index as i32 + operation.argument - 1) as usize,
                }
                index += 1;
            } else {
                break;
            }
        }

        (accumulator, exited_successfully)
    }

    fn test_operation_permutations(
        operations: &mut Vec<Operation>,
        op_from: Op,
        op_to: Op,
    ) -> Option<i32> {
        // Very dumb way
        for i in 0..operations.len() {
            let mut permutation = operations.clone();
            let mut changed = false;
            if let Some(operation) = permutation.get_mut(i) {
                if operation.op == op_from {
                    operation.op = op_to;
                    changed = true;
                }
            }
            if changed {
                let value = run_operations(&mut permutation);
                if value.1 {
                    return Some(value.0);
                }
                let operation = permutation.get_mut(i).expect("It is still a valid index");
                // Change it back and continue the loop.
                operation.op = op_from;
            }
        }

        None
    }

    #[test]
    fn example() {
        let contents = read_input("src/exercises/day8/example.txt");
        let mut operations = process_input_into_operations(&contents);
        let value = run_operations(&mut operations);

        assert_eq!(5, value.0);
    }

    #[test]
    fn input() {
        let contents = read_input("src/exercises/day8/input.txt");
        let mut operations = process_input_into_operations(&contents);
        let value = run_operations(&mut operations);

        assert_eq!(1200, value.0);
    }

    #[test]
    fn example_b() {
        let contents = read_input("src/exercises/day8/example.txt");
        let mut operations = process_input_into_operations(&contents);
        let value = {
            if let Some(value) = test_operation_permutations(&mut operations, JMP, NOP) {
                value
            } else if let Some(value) = test_operation_permutations(&mut operations, NOP, JMP) {
                value
            } else {
                panic!("No valid transformation program");
            }
        };

        assert_eq!(8, value);
    }

    #[test]
    fn input_b() {
        let contents = read_input("src/exercises/day8/input.txt");
        let mut operations = process_input_into_operations(&contents);
        let value = {
            if let Some(value) = test_operation_permutations(&mut operations, JMP, NOP) {
                value
            } else if let Some(value) = test_operation_permutations(&mut operations, NOP, JMP) {
                value
            } else {
                panic!("No valid transformation program");
            }
        };

        assert_eq!(1023, value);
    }
}
