pub mod a {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;

        fn write(contents: &str, memory: &mut Vec<u64>) {
            // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            let mut or_mask = 0;
            let mut and_mask = 0;
            for line in contents.lines() {
                if line.starts_with("mask") {
                    for part in line.split(" = ") {
                        if part == "mask" {
                            continue;
                        }
                        or_mask = u64::from_str_radix(part.replace('X', "0").as_str(), 2)
                            .expect("Valid binary");
                        and_mask = u64::from_str_radix(part.replace('X', "1").as_str(), 2)
                            .expect("Valid binary");
                    }
                } else {
                    let mut instruction_index = 0;
                    for (index, part) in line.split(" = ").enumerate() {
                        if index == 0 {
                            let mut instr = part.chars().rev().collect::<String>();
                            // strip ']'
                            instr.remove(0);
                            instr = instr.chars().rev().collect::<String>();
                            instruction_index =
                                usize::from_str_radix(&instr[4..], 10).expect("Valid decimal");
                        } else {
                            let mut val = u64::from_str_radix(part, 10).expect("Valid decimal");
                            val = val | or_mask;
                            val = val & and_mask;

                            memory[instruction_index] = val;
                        }
                    }
                }
            }
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day14/example.txt");
            let mut memory = vec![0; 100_000];
            write(&contents, &mut memory);

            assert_eq!(165, memory.iter().sum::<u64>());
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day14/input.txt");
            let mut memory = vec![0; 100_000];
            write(&contents, &mut memory);

            assert_eq!(15919415426101, memory.iter().sum::<u64>());
        }
    }
}

pub mod b {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;
        use std::collections::HashMap;

        fn find_addresses(mask: &str) -> Vec<u64> {
            let mut to_ret = vec![];

            let mask = mask.chars().collect::<Vec<char>>();
            let mut qbit_positions = vec![];
            for (index, char) in mask.iter().enumerate() {
                if *char == 'X' {
                    qbit_positions.push(index);
                }
            }

            let mut truth_table = vec![];
            let max = 2_i32.pow(qbit_positions.len() as u32);
            for i in 0..max {
                let bin_str = format!("{:0>36}", format!("{:b}", i));
                let chars = bin_str.as_str().chars().rev().collect::<Vec<char>>();
                truth_table.push(chars);
            }

            for val in truth_table {
                let mut mask = mask.clone();
                for index in 0..qbit_positions.len() {
                    mask[qbit_positions[index]] = val[index];
                }
                let addr = u64::from_str_radix(mask.iter().collect::<String>().as_str(), 2)
                    .expect("Valid binary");
                to_ret.push(addr)
            }

            to_ret
        }

        fn write(contents: &str, memory: &mut HashMap<u64, u64>) {
            let mut mask = String::new();
            let mut mask_ord = String::new();
            for line in contents.lines() {
                if line.starts_with("mask") {
                    for part in line.split(" = ") {
                        if part == "mask" {
                            continue;
                        }
                        mask = part.to_string();
                    }
                } else {
                    for (index, part) in line.split(" = ").enumerate() {
                        if index == 0 {
                            let mut instr = part.chars().rev().collect::<String>();
                            instr.remove(0); // strip ']'
                            instr = instr.chars().rev().collect::<String>();
                            let instruction_index = format!(
                                "{:0>36}",
                                format!(
                                    "{:b}",
                                    u64::from_str_radix(&instr[4..], 10).expect("Valid decimal")
                                )
                            );
                            mask_ord = String::new();
                            for (index, char) in instruction_index.chars().enumerate() {
                                if mask.as_bytes()[index] as char == 'X' {
                                    mask_ord.push('X');
                                } else if mask.as_bytes()[index] as char == '0' {
                                    mask_ord.push(char);
                                } else {
                                    mask_ord.push('1');
                                }
                            }
                        } else {
                            let mut addresses = find_addresses(&mask_ord);
                            addresses.sort();
                            for address in addresses {
                                memory.insert(
                                    address,
                                    str::parse::<u64>(part).expect("Valid decimal"),
                                );
                            }
                        }
                    }
                }
            }
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day14/example_b.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            assert_eq!(208, memory.values().fold(0, |count, val| count + *val));
        }

        #[test]
        fn example2() {
            let contents = read_input("src/exercises/day14/example_b2.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            assert_eq!(12544, memory.values().fold(0, |count, val| count + *val));
        }

        #[test]
        fn example3() {
            let contents = read_input("src/exercises/day14/example_b3.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            assert_eq!(
                249_893_444_096,
                memory.values().fold(0, |count, val| count + *val)
            );
        }

        #[test]
        fn example4() {
            let contents = read_input("src/exercises/day14/example_b4.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            assert_eq!(
                1_952_292_532,
                memory.values().fold(0, |count, val| count + *val)
            );
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day14/input.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            // 2_449_384_529_160 - too low
            assert_eq!(
                3_443_997_590_975,
                memory.values().fold(0, |count, val| count + *val)
            );
        }
    }
}
