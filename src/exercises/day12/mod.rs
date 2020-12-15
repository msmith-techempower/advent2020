pub enum Cardinal {
    North,
    South,
    East,
    West,
}

pub mod a {
    #[cfg(test)]
    mod tests {
        use crate::exercises::day12::Cardinal;
        use crate::exercises::day12::Cardinal::{East, North, South, West};
        use crate::utils::read_input;

        fn rotate_clockwise(facing: Cardinal, turns: usize) -> Cardinal {
            let mut to_ret = facing;
            for _ in 0..turns {
                to_ret = match to_ret {
                    North => East,
                    East => South,
                    South => West,
                    West => North,
                }
            }

            to_ret
        }

        fn rotate_counter_clockwise(facing: Cardinal, turns: usize) -> Cardinal {
            let mut to_ret = facing;
            for _ in 0..turns {
                to_ret = match to_ret {
                    North => West,
                    West => South,
                    South => East,
                    East => North,
                }
            }

            to_ret
        }

        fn manhattan_distance(contents: &str) -> usize {
            let mut east_west: i32 = 0;
            let mut north_south: i32 = 0;
            let mut facing = East;
            for line in contents.lines() {
                let units = str::parse::<usize>(&line[1..]).expect("Valid integer");
                match &line[0..1] {
                    "R" => {
                        let turns = units / 90;
                        facing = rotate_clockwise(facing, turns);
                    }
                    "L" => {
                        let turns = units / 90;
                        facing = rotate_counter_clockwise(facing, turns);
                    }
                    "F" => match facing {
                        North => north_south += units as i32,
                        South => north_south -= units as i32,
                        East => east_west += units as i32,
                        West => east_west -= units as i32,
                    },
                    "N" => {
                        north_south += units as i32;
                    }
                    "S" => {
                        north_south -= units as i32;
                    }
                    "E" => {
                        east_west += units as i32;
                    }
                    "W" => {
                        east_west -= units as i32;
                    }
                    _ => panic!("Invalid direction!"),
                }
            }

            east_west.abs() as usize + north_south.abs() as usize
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day12/example.txt");
            let manhattan_distance = manhattan_distance(&contents);

            assert_eq!(25, manhattan_distance);
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day12/input.txt");
            let manhattan_distance = manhattan_distance(&contents);

            assert_eq!(445, manhattan_distance);
        }
    }
}
pub mod b {
    #[cfg(test)]
    mod tests {
        use crate::utils::read_input;
        use std::ops::Neg;

        struct Waypoint {
            east_west: i32,
            north_south: i32,
        }
        impl Waypoint {
            fn rotate_clockwise(mut self, turns: usize) -> Self {
                for _ in 0..turns {
                    let north_south = self.north_south;
                    let east_west = self.east_west;
                    self.east_west = north_south;
                    self.north_south = east_west.neg();
                }

                self
            }
            fn rotate_counter_clockwise(mut self, turns: usize) -> Self {
                for _ in 0..turns {
                    let north_south = self.north_south;
                    let east_west = self.east_west;
                    self.east_west = north_south.neg();
                    self.north_south = east_west;
                    self.east_west = north_south.neg();
                    self.north_south = east_west;
                }

                self
            }
        }

        fn manhattan_distance(contents: &str) -> usize {
            let mut waypoint = Waypoint {
                east_west: 10,
                north_south: 1,
            };

            let mut north_south = 0;
            let mut east_west = 0;

            for line in contents.lines() {
                let units = str::parse::<usize>(&line[1..]).expect("Valid integer");
                match &line[0..1] {
                    "R" => {
                        let turns = units / 90;
                        waypoint = waypoint.rotate_clockwise(turns);
                    }
                    "L" => {
                        let turns = units / 90;
                        waypoint = waypoint.rotate_counter_clockwise(turns);
                    }
                    "F" => {
                        north_south += waypoint.north_south * units as i32;
                        east_west += waypoint.east_west as i32 * units as i32;
                    }
                    "N" => {
                        waypoint.north_south += units as i32;
                    }
                    "S" => {
                        waypoint.north_south -= units as i32;
                    }
                    "E" => {
                        waypoint.east_west += units as i32;
                    }
                    "W" => {
                        waypoint.east_west -= units as i32;
                    }
                    _ => panic!("Invalid direction!"),
                }
            }

            east_west.abs() as usize + north_south.abs() as usize
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day12/example.txt");
            let manhattan_distance = manhattan_distance(&contents);

            assert_eq!(286, manhattan_distance);
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day12/input.txt");
            let manhattan_distance = manhattan_distance(&contents);

            assert_eq!(42495, manhattan_distance);
        }
    }
}
