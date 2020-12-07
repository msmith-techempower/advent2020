use std::collections::HashSet;

pub type Group = HashSet<char>;
pub type Groups = Vec<Group>;

#[cfg(test)]
mod tests {
    use crate::exercises::day6::{Group, Groups};
    use crate::utils::read_input;
    use std::collections::HashSet;

    fn process_6a_input(contents: &str) -> Groups {
        let mut groups = vec![];
        let mut group = Group::default();
        for line in contents.lines() {
            if line.is_empty() {
                groups.push(group);
                group = Group::default();
            } else {
                for char in line.chars() {
                    group.insert(char);
                }
            }
        }
        if !group.is_empty() {
            groups.push(group);
        }

        groups
    }

    fn process_6b_input(contents: &str) -> Groups {
        let mut groups = vec![];
        let mut first = true;
        let mut group = Group::default();
        for line in contents.lines() {
            if line.is_empty() {
                groups.push(group);
                group = Group::default();
                first = true;
            } else {
                if first {
                    for char in line.chars() {
                        group.insert(char);
                    }
                    first = false;
                }
                let mut comparison = Group::default();
                for char in line.chars() {
                    comparison.insert(char);
                }
                let intersection: HashSet<&char> = group.intersection(&comparison).collect();
                let mut intersection_group = Group::default();
                for char in intersection {
                    intersection_group.insert(*char);
                }
                group = intersection_group;
            }
        }
        if !group.is_empty() {
            groups.push(group);
        }

        groups
    }

    #[test]
    fn example_6a() {
        let contents = read_input("src/exercises/day6/example.txt");
        let groups = process_6a_input(&contents);

        let mut sum = 0;
        for group in groups {
            sum += group.len();
        }

        assert_eq!(11, sum);
    }

    #[test]
    fn example_6b() {
        let contents = read_input("src/exercises/day6/example.txt");
        let groups = process_6b_input(&contents);

        let mut sum = 0;
        for group in groups {
            sum += group.len();
        }

        assert_eq!(6, sum);
    }

    #[test]
    fn day6a() {
        let contents = read_input("src/exercises/day6/input.txt");
        let groups = process_6a_input(&contents);

        let mut sum = 0;
        for group in groups {
            sum += group.len();
        }

        assert_eq!(6443, sum);
    }

    #[test]
    fn day6b() {
        let contents = read_input("src/exercises/day6/input.txt");
        let groups = process_6b_input(&contents);

        let mut sum = 0;
        for group in groups {
            sum += group.len();
        }

        assert_eq!(3232, sum);
    }
}
