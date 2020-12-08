pub mod day7a {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;
        use std::collections::HashSet;

        type Bags = HashSet<String>;

        fn process_day7a_input(contents: &str, bag_to_find: &str, bags: &mut Bags) {
            // ex:
            // light red bags contain 1 bright white bag, 2 muted yellow bags.
            // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            // faded blue bags contain no other bags.
            // dotted black bags contain no other bags.
            //
            // [type] bags contain ([n] [type] bag)
            for line in contents.lines() {
                if !line.starts_with(bag_to_find) {
                    if line.contains(bag_to_find) {
                        if let Some(bag) = line.split(" bag").collect::<Vec<&str>>().get(0) {
                            bags.insert(bag.to_string());
                            process_day7a_input(contents, bag, bags);
                        }
                    }
                }
            }
        }

        #[test]
        fn day7a_example() {
            let contents = read_input("src/exercises/day7/example.txt");
            let mut bags = Bags::default();
            process_day7a_input(&contents, "shiny gold", &mut bags);

            assert_eq!(4, bags.len());
        }

        #[test]
        fn day7a() {
            let contents = read_input("src/exercises/day7/input.txt");
            let mut bags = Bags::default();
            process_day7a_input(&contents, "shiny gold", &mut bags);

            assert_eq!(233, bags.len());
        }
    }
}

pub mod day7b {

    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;

        #[derive(Default, Debug)]
        struct Bag {
            name: String,
            count: i32,
            bags: Vec<Bag>,
        }

        impl Bag {
            fn count(&self) -> i32 {
                self.bags
                    .iter()
                    .fold(0, |count, bag| count + bag.count_bags())
            }
            fn count_bags(&self) -> i32 {
                let mut sum = self.count;
                for bag in &self.bags {
                    sum += self.count * bag.count_bags();
                }

                sum
            }

            fn insert_bag(&mut self, count: i32, bag: &str) -> &mut Self {
                let bag = Bag {
                    name: bag.to_string(),
                    count,
                    bags: vec![],
                };
                self.bags.push(bag);
                let index = self.bags.len() - 1;
                self.bags.get_mut(index).unwrap()
            }
        }

        fn process_day7b_input(contents: &str, bag_to_find: &str, bags: &mut Bag) {
            // ex:
            // shiny gold bags contain 2 dark red bags.
            // dark red bags contain 2 dark orange bags.
            // dark orange bags contain 2 dark yellow bags.
            // dark yellow bags contain 2 dark green bags.
            // dark green bags contain 2 dark blue bags.
            // dark blue bags contain 2 dark violet bags.
            // dark violet bags contain no other bags.
            for line in contents.lines() {
                if line.starts_with(bag_to_find) {
                    if let Some(bag_or_bags) = line.split("contain ").collect::<Vec<&str>>().get(1)
                    {
                        // Some lines have commas:
                        // ex:
                        //   vibrant lime bags contain 3 dull chartreuse bags, 4 pale crimson bags, 5 dark orange bags, 3 vibrant purple bags.
                        // becomes:
                        //   3 dull chartreuse bags, 4 pale crimson bags, 5 dark orange bags, 3 vibrant purple bags.
                        for bag in bag_or_bags.split(", ") {
                            // 3 dull chartreuse bags
                            // 4 pale crimson bags
                            // 3 vibrant purple bags.
                            let mut bag = bag.replace(" bags", "");
                            bag = bag.replace(" bag", "");
                            bag = bag.replace(".", "");
                            // 3 dull chartreuse
                            // 4 pale crimson
                            // 3 vibrant purple

                            // Terminal case
                            if bag == "no other" {
                                return;
                            }

                            let count = str::parse::<i32>(&bag[..1]).unwrap_or(1);
                            let bags = bags.insert_bag(count, &bag[2..]);

                            process_day7b_input(contents, &bag[2..], bags);
                        }
                    }
                }
            }
        }

        #[test]
        fn day7b_1_example() {
            let contents = read_input("src/exercises/day7/example.txt");
            let mut bags = Bag {
                name: "shiny gold".to_string(),
                count: 1,
                bags: vec![],
            };
            process_day7b_input(&contents, "shiny gold", &mut bags);

            assert_eq!(32, bags.count());
        }

        #[test]
        fn day7b_example() {
            let contents = read_input("src/exercises/day7/example_b.txt");
            let mut bags = Bag {
                name: "shiny gold".to_string(),
                count: 1,
                bags: vec![],
            };
            process_day7b_input(&contents, "shiny gold", &mut bags);

            assert_eq!(126, bags.count());
        }

        #[test]
        fn day7b_example_small() {
            let contents = read_input("src/exercises/day7/example_small.txt");
            let mut bags = Bag {
                name: "shiny gold".to_string(),
                count: 1,
                bags: vec![],
            };
            process_day7b_input(&contents, "shiny gold", &mut bags);

            assert_eq!(137, bags.count());
        }

        #[test]
        fn day7b() {
            let contents = read_input("src/exercises/day7/input.txt");
            let mut bags = Bag {
                name: "shiny gold".to_string(),
                count: 1,
                bags: vec![],
            };
            process_day7b_input(&contents, "shiny gold", &mut bags);

            assert_eq!(421550, bags.count());
        }
    }
}
