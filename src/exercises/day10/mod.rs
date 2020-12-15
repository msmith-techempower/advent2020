pub fn parse_input(contents: &str) -> Vec<u128> {
    let mut to_ret = vec![];
    for line in contents.lines() {
        to_ret.push(str::parse::<u128>(line).expect("Line is a valid usize"));
    }

    to_ret
}

pub mod a {
    #[cfg(test)]
    mod test {
        use crate::exercises::day10::parse_input;
        use crate::utils::read_input;
        use std::collections::HashMap;

        fn order_all_adapters_by_rating(
            adapters: &Vec<u128>,
            current_joltage: u128,
        ) -> Vec<(u128, u128)> {
            let mut to_ret = vec![];

            let mut adapters = adapters.clone();
            let mut remaining = adapters.len();
            let mut index = 0;
            while index < remaining {
                let adapter = adapters.remove(index);
                // Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still
                // produce its rated output joltage.
                if adapter >= current_joltage && current_joltage + 3 >= adapter {
                    // Potentially, this adapter is valid. Check the rest.
                    let differential = adapter - current_joltage;

                    if adapters.is_empty() {
                        to_ret.push((adapter, differential));
                        return to_ret;
                    }
                    let mut rest = order_all_adapters_by_rating(&adapters, adapter);
                    if !rest.is_empty() {
                        to_ret.push((adapter, differential));
                        to_ret.append(&mut rest);
                        return to_ret;
                    }
                }
                index += 1;
                remaining -= 1;
            }

            to_ret
        }

        fn parse_differentials(
            adapters: &Vec<(u128, u128)>,
            ending_differential: u128,
        ) -> (u128, HashMap<u128, u128>) {
            let mut max = 0;
            let mut differentials: HashMap<u128, u128> = HashMap::default();
            for adapter in adapters {
                if adapter.0 > max {
                    max = adapter.0;
                }
                if !differentials.contains_key(&adapter.1) {
                    differentials.insert(adapter.1, 1);
                } else {
                    let differential = *differentials.get(&adapter.1).expect("Valid key");
                    differentials.insert(adapter.1, differential + 1);
                }
            }

            if !differentials.contains_key(&ending_differential) {
                differentials.insert(ending_differential, 1);
            } else {
                let differential = *differentials.get(&ending_differential).expect("Valid key");
                differentials.insert(ending_differential, differential + 1);
            }

            (max, differentials)
        }

        #[test]
        fn example_1a() {
            let contents = read_input("src/exercises/day10/example_1a.txt");
            let mut adapters = parse_input(&contents);

            // Only sort once
            adapters.sort();
            // Treat the charging outlet near your seat as having an effective joltage rating of 0.
            let current_joltage = 0;

            let ending_differential = 3;
            let adapters = order_all_adapters_by_rating(&adapters, current_joltage);
            let differentials = parse_differentials(&adapters, ending_differential);

            assert_eq!(22, differentials.0 + ending_differential);
            assert_eq!(7, *differentials.1.get(&1).expect("Differentials of 1"));
            assert_eq!(5, *differentials.1.get(&3).expect("Differentials of 3"));
        }

        #[test]
        fn example_2a() {
            let contents = read_input("src/exercises/day10/example_2a.txt");
            let mut adapters = parse_input(&contents);

            // Only sort once
            adapters.sort();
            // Treat the charging outlet near your seat as having an effective joltage rating of 0.
            let current_joltage = 0;

            let ending_differential = 3;
            let adapters = order_all_adapters_by_rating(&adapters, current_joltage);
            let differentials = parse_differentials(&adapters, ending_differential);

            assert_eq!(52, differentials.0 + ending_differential);
            assert_eq!(22, *differentials.1.get(&1).expect("Differentials of 1"));
            assert_eq!(10, *differentials.1.get(&3).expect("Differentials of 3"));
        }

        #[test]
        fn input_a() {
            let contents = read_input("src/exercises/day10/input.txt");
            let mut adapters = parse_input(&contents);

            // Only sort once
            adapters.sort();
            // Treat the charging outlet near your seat as having an effective joltage rating of 0.
            let current_joltage = 0;

            let ending_differential = 3;
            let adapters = order_all_adapters_by_rating(&adapters, current_joltage);
            let differentials = parse_differentials(&adapters, ending_differential);

            assert_eq!(
                2_368,
                *differentials.1.get(&1).expect("Differentials of 1")
                    * *differentials.1.get(&3).expect("Differentials of 3")
            )
        }
    }
}

pub mod b {
    #[cfg(test)]
    mod test {
        use crate::exercises::day10::parse_input;
        use crate::utils::read_input;
        use std::collections::HashMap;

        const MAGIC_JOLTAGE: u128 = 3;

        fn count_permutations(adapters: &[u128]) -> u128 {
            let mut hops = HashMap::new();

            for adapter in adapters {
                if hops.is_empty() {
                    hops.insert(*adapter, 1_u128);
                } else {
                    let mut path_count = 0;
                    for hop in *adapter + 1..*adapter + MAGIC_JOLTAGE + 1 {
                        if let Some(prev_path_count) = hops.get(&hop) {
                            path_count += prev_path_count;
                        }
                    }
                    hops.insert(*adapter, path_count);
                }
            }

            *hops.values().max().expect("Hops is non-empty")
        }

        #[test]
        fn example_1b() {
            let contents = read_input("src/exercises/day10/example_1a.txt");
            let mut adapters = parse_input(&contents);

            adapters.push(0);
            adapters.sort();
            adapters.reverse();

            let count = count_permutations(&adapters);

            assert_eq!(8, count);
        }

        #[test]
        fn example_2b() {
            let contents = read_input("src/exercises/day10/example_2a.txt");
            let mut adapters = parse_input(&contents);

            adapters.push(0);
            adapters.sort();
            adapters.reverse();

            let count = count_permutations(&adapters);

            assert_eq!(19_208, count);
        }

        #[test]
        fn day10b() {
            let contents = read_input("src/exercises/day10/input.txt");
            let mut adapters = parse_input(&contents);

            adapters.push(0);
            adapters.sort();
            adapters.reverse();

            let count = count_permutations(&adapters);

            assert_eq!(1_727_094_849_536, count);
        }
    }
}
