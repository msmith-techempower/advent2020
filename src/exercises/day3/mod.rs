type Grid = Vec<Vec<bool>>;

trait GridMethods {
    fn x_len(&self) -> usize;
}

impl GridMethods for Grid {
    fn x_len(&self) -> usize {
        self.get(0).expect("Grid has a height").len()
    }
}

pub fn is_tree_at_location(grid: &Grid, x: usize, y: usize) -> bool {
    if let Some(row) = grid.get(y) {
        if let Some(tree) = row.get(x) {
            return *tree;
        }
    }

    false
}

/// Counts the number of trees we would encounter traversing the map by going
/// right `x_delta` and down `y_delta` every move.
pub fn count_number_of_trees_encountered(grid: &Grid, x_delta: usize, y_delta: usize) -> usize {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < grid.len() {
        x = (x + x_delta) % grid.x_len();
        y += y_delta;
        if is_tree_at_location(&grid, x, y) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::exercises::day3::{
        count_number_of_trees_encountered, is_tree_at_location, Grid, GridMethods,
    };
    use crate::utils::read_input;

    // Example input:
    //  key:
    //    . empty
    //    # tree
    //  Pattern repeats to the right infinitely (e.g. modulo line-length)
    //
    // ..##.......
    // #...#...#..
    // .#....#..#.
    // ..#.#...#.#
    // .#...##..#.
    // ..#.##.....
    // .#.#.#....#
    // .#........#
    // #.##...#...
    // #...##....#
    // .#..#...#.#

    fn process_day3_input(contents: &str) -> Grid {
        let mut grid = vec![];
        for line in contents.lines() {
            let mut row = vec![];
            for chr in line.chars() {
                match chr {
                    '#' => row.push(true),
                    _ => row.push(false),
                }
            }
            grid.push(row);
        }

        grid
    }

    #[test]
    fn test_example_grid() {
        let grid = process_day3_input("..##.......");
        assert_eq!(false, is_tree_at_location(&grid, 0, 0));
        assert_eq!(false, is_tree_at_location(&grid, 1, 0));
        assert_eq!(true, is_tree_at_location(&grid, 2, 0));
        assert_eq!(true, is_tree_at_location(&grid, 3, 0));
        assert_eq!(false, is_tree_at_location(&grid, 4, 0));
        assert_eq!(false, is_tree_at_location(&grid, 5, 0));
        assert_eq!(false, is_tree_at_location(&grid, 6, 0));
        assert_eq!(false, is_tree_at_location(&grid, 7, 0));
        assert_eq!(false, is_tree_at_location(&grid, 8, 0));
        assert_eq!(false, is_tree_at_location(&grid, 9, 0));
        assert_eq!(false, is_tree_at_location(&grid, 10, 0));

        assert_eq!(11, grid.x_len());
    }

    #[test]
    fn test_first_line_of_input() {
        let contents = read_input("src/exercises/day3/input.txt");
        let grid = process_day3_input(&contents);
        // first line of our input:
        //   .........#..##.##..............
        assert_eq!(false, is_tree_at_location(&grid, 0, 0));
        assert_eq!(false, is_tree_at_location(&grid, 1, 0));
        assert_eq!(false, is_tree_at_location(&grid, 2, 0));
        assert_eq!(false, is_tree_at_location(&grid, 3, 0));
        assert_eq!(false, is_tree_at_location(&grid, 4, 0));
        assert_eq!(false, is_tree_at_location(&grid, 5, 0));
        assert_eq!(false, is_tree_at_location(&grid, 6, 0));
        assert_eq!(false, is_tree_at_location(&grid, 7, 0));
        assert_eq!(false, is_tree_at_location(&grid, 8, 0));
        assert_eq!(true, is_tree_at_location(&grid, 9, 0));
        assert_eq!(false, is_tree_at_location(&grid, 10, 0));
        assert_eq!(false, is_tree_at_location(&grid, 11, 0));
        assert_eq!(true, is_tree_at_location(&grid, 12, 0));
        assert_eq!(true, is_tree_at_location(&grid, 13, 0));
        assert_eq!(false, is_tree_at_location(&grid, 14, 0));
        assert_eq!(true, is_tree_at_location(&grid, 15, 0));
        assert_eq!(true, is_tree_at_location(&grid, 16, 0));
        assert_eq!(false, is_tree_at_location(&grid, 17, 0));
        assert_eq!(false, is_tree_at_location(&grid, 18, 0));
        assert_eq!(false, is_tree_at_location(&grid, 19, 0));
        assert_eq!(false, is_tree_at_location(&grid, 20, 0));
        assert_eq!(false, is_tree_at_location(&grid, 21, 0));
        assert_eq!(false, is_tree_at_location(&grid, 22, 0));
        assert_eq!(false, is_tree_at_location(&grid, 23, 0));
        assert_eq!(false, is_tree_at_location(&grid, 24, 0));
        assert_eq!(false, is_tree_at_location(&grid, 25, 0));
        assert_eq!(false, is_tree_at_location(&grid, 26, 0));
        assert_eq!(false, is_tree_at_location(&grid, 27, 0));
        assert_eq!(false, is_tree_at_location(&grid, 28, 0));
        assert_eq!(false, is_tree_at_location(&grid, 29, 0));
        assert_eq!(false, is_tree_at_location(&grid, 30, 0));
    }

    #[test]
    fn test_example_grid_first_line() {
        let contents = read_input("src/exercises/day3/example_input.txt");
        let grid = process_day3_input(&contents);

        assert_eq!(7, count_number_of_trees_encountered(&grid, 3, 1));
    }

    #[test]
    fn test_day3a() {
        let contents = read_input("src/exercises/day3/input.txt");
        let grid = process_day3_input(&contents);

        // Starting at the top-left corner of your map and following a slope of
        // right 3 and down 1, how many trees would you encounter?
        assert_eq!(268, count_number_of_trees_encountered(&grid, 3, 1));
    }

    #[test]
    fn test_example_b_grid() {
        let contents = read_input("src/exercises/day3/example_input.txt");
        let grid = process_day3_input(&contents);

        assert_eq!(2, count_number_of_trees_encountered(&grid, 1, 1));
        assert_eq!(7, count_number_of_trees_encountered(&grid, 3, 1));
        assert_eq!(3, count_number_of_trees_encountered(&grid, 5, 1));
        assert_eq!(4, count_number_of_trees_encountered(&grid, 7, 1));
        assert_eq!(2, count_number_of_trees_encountered(&grid, 1, 2));
    }

    #[test]
    fn test_day3b() {
        let contents = read_input("src/exercises/day3/input.txt");
        let grid = process_day3_input(&contents);

        // What do you get if you multiply together the number of trees
        // encountered on each of the listed slopes?
        let product = count_number_of_trees_encountered(&grid, 1, 1)
            * count_number_of_trees_encountered(&grid, 3, 1)
            * count_number_of_trees_encountered(&grid, 5, 1)
            * count_number_of_trees_encountered(&grid, 7, 1)
            * count_number_of_trees_encountered(&grid, 1, 2);

        assert_eq!(3093068400, product);
    }
}
