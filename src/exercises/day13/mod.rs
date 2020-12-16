pub mod a {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;

        fn parse_input(contents: &str) -> usize {
            let mut lines = contents.lines();
            let departure_time = str::parse::<usize>(lines.next().expect("Valid first line"))
                .expect("Valid integer time");

            let mut times = vec![];
            for time in lines.next().expect("Valid second line").split(',') {
                if let Ok(time) = str::parse::<usize>(time) {
                    times.push(time);
                }
            }

            for i in departure_time.. {
                for time in &times {
                    if i % time == 0 {
                        return (i - departure_time) * time;
                    }
                }
            }

            0
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day13/example.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(295, magic_number);
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day13/input.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(5946, magic_number);
        }
    }
}

pub mod b {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;

        fn parse_input(contents: &str) -> usize {
            let mut lines = contents.lines();
            // "The first line in your input is no longer relevant."
            lines.next();

            let mut times = vec![];
            let mut index = 0;
            for time in lines.next().expect("Valid second line").split(',') {
                if let Ok(time) = str::parse::<usize>(time) {
                    times.push((time, index));
                }
                index += 1;
            }

            eprintln!("{:?}", times);

            // let mut num = times.get(0).expect("Zero index").0;
            // loop {
            //     let mut valid = true;
            //     for (index, time) in times.iter().enumerate() {
            //         if (num + times.get(index).expect("Valid index").1) % time.0 != 0 {
            //             valid = false;
            //         }
            //     }
            //
            //     if valid {
            //         return num;
            //     }
            //
            //     num += times.get(0).expect("Zero index").0;
            // }
            0
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day13/example.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(1_068_781, magic_number);
        }

        #[test]
        fn example2() {
            let contents = read_input("src/exercises/day13/example_2.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(3_417, magic_number);
        }

        #[test]
        fn example3() {
            let contents = read_input("src/exercises/day13/example_3.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(754_018, magic_number);
        }

        #[test]
        fn example4() {
            let contents = read_input("src/exercises/day13/example_4.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(779_210, magic_number);
        }

        #[test]
        fn example5() {
            let contents = read_input("src/exercises/day13/example_5.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(1_261_476, magic_number);
        }

        #[test]
        fn example6() {
            let contents = read_input("src/exercises/day13/example_6.txt");
            let magic_number = parse_input(&contents);

            assert_eq!(1_202_161_486, magic_number);
        }

        #[test]
        fn input() {
            let _contents = read_input("src/exercises/day13/input.txt");
            // 230_000_000_000_000 - too low (according to the website)
            // 460_000_000_000_000 - too low  ^

            // Note: DO NOT RUN THIS; TAKES FOREVER
            // let magic_number = parse_input(&contents, 460_000_000_000_000);
            // eprintln!(645_338_524_823_718, magic_number);
        }
    }
}
