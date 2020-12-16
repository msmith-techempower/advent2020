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
            // eprintln!("Truth table");
            // eprintln!("{}", &mask);
            // eprintln!("-----------");
            let mut to_ret = vec![];
            //   000000000000000000000000000000X1001X ->
            // [
            //   000000000000000000000000000000010010
            //   000000000000000000000000000000110010
            //   000000000000000000000000000000010011
            //   000000000000000000000000000000110011
            // ]
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
                // eprintln!("addr: {}", mask.iter().collect::<String>());
                to_ret.push(
                    u64::from_str_radix(mask.iter().collect::<String>().as_str(), 2)
                        .expect("Valid binary"),
                )
            }

            // for num in &to_ret {
            //     eprintln!("{}", format!("{:0>36}", format!("{:b}", num)));
            // }

            to_ret
        }

        fn write(contents: &str, memory: &mut HashMap<u64, u64>) {
            let mut mask = String::new();
            for line in contents.lines() {
                if line.starts_with("mask") {
                    for part in line.split(" = ") {
                        if part == "mask" {
                            continue;
                        }
                        mask = part.to_string();
                    }
                } else {
                    let mut _instruction_index = 0;
                    for (index, part) in line.split(" = ").enumerate() {
                        if index == 0 {
                            let mut instr = part.chars().rev().collect::<String>();
                            // strip ']'
                            instr.remove(0);
                            instr = instr.chars().rev().collect::<String>();
                            _instruction_index =
                                usize::from_str_radix(&instr[4..], 10).expect("Valid decimal");
                            let chars = format!("{:0>36}", format!("{:b}", _instruction_index))
                                .chars()
                                .collect::<Vec<char>>();
                            // eprintln!(
                            //     "address: {}",
                            //     format!("{:0>36}", format!("{:b}", instruction_index))
                            // );
                            // eprintln!("mask:    {}", mask);
                            let mut mask_ord = vec![];
                            for (index, char) in chars.iter().enumerate() {
                                if mask.as_bytes()[index] as char == 'X' {
                                    mask_ord.push('X');
                                } else if mask.as_bytes()[index] as char == '1' || *char == '1' {
                                    mask_ord.push('1');
                                } else {
                                    mask_ord.push('0');
                                }
                            }
                            mask = mask_ord.iter().collect::<String>();

                        // eprintln!("result:  {}", mask);
                        } else {
                            // eprintln!("mask   : {}", mask);
                            let addresses = find_addresses(&mask);
                            // eprintln!("{:?}", addresses.len());
                            // eprintln!("mem[{}] = {}", instruction_index, part);
                            // eprintln!("----------------------------");
                            for address in addresses {
                                // eprintln!("mem[{}] = {}", address, part);
                                memory.insert(
                                    address,
                                    str::parse::<u64>(part).expect("Valid decimal"),
                                );
                            }
                            // eprintln!();
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

            eprintln!("{:?}", memory);

            eprintln!(
                "count: {}",
                memory.values().fold(0, |count, val| count + *val)
            );
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day14/input.txt");
            let mut memory = HashMap::new();
            write(&contents, &mut memory);

            // 2_449_384_529_160 - too low
            eprintln!("{}", memory.values().fold(0, |count, val| count + *val));
        }
    }
}
