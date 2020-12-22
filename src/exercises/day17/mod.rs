pub mod a {
    #[cfg(test)]
    mod test {
        use crate::utils::read_input;
        use std::collections::HashMap;

        type Cubes = HashMap<(i32, i32, i32), bool>;

        fn parse_points(contents: &str) -> Cubes {
            let mut cubes = HashMap::new();
            let z = 0;
            let mut y = 0;
            for line in contents.lines() {
                for (x, char) in line.chars().enumerate() {
                    if char == '#' {
                        cubes.insert((x as i32, y, z), true);
                    } else {
                        cubes.insert((x as i32, y, z), false);
                    }
                }
                y += 1;
            }
            cubes
        }

        fn cycle(cubes: &mut Cubes) -> Cubes {
            // Add the missing neighbors
            let mut missing_neighbors = vec![];
            for key in cubes.keys() {
                for x0 in key.0 - 1..key.0 + 2 {
                    for y0 in key.1 - 1..key.1 + 2 {
                        for z0 in key.2 - 1..key.2 + 2 {
                            if &(x0, y0, z0) != key {
                                if !cubes.contains_key(&(x0, y0, z0)) {
                                    missing_neighbors.push((x0, y0, z0));
                                }
                            }
                        }
                    }
                }
            }
            for missing_neighbor in missing_neighbors {
                cubes.insert(missing_neighbor, false);
            }
            let mut new_cubes = cubes.clone();
            for (key, _) in cubes.clone() {
                let mut active_neighbor_count = 0;
                for x0 in key.0 - 1..key.0 + 2 {
                    for y0 in key.1 - 1..key.1 + 2 {
                        for z0 in key.2 - 1..key.2 + 2 {
                            if (x0, y0, z0) != key {
                                if let Some(true) = cubes.get(&(x0, y0, z0)) {
                                    active_neighbor_count += 1;
                                }
                            }
                        }
                    }
                }
                if let Some(active) = new_cubes.get_mut(&key) {
                    if *active {
                        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
                        // the cube remains active.
                        // Otherwise, the cube becomes inactive.
                        if active_neighbor_count != 2 && active_neighbor_count != 3 {
                            *active = false;
                        }
                    } else {
                        // If a cube is inactive but exactly 3 of its neighbors are active, the cube
                        // becomes active.
                        // Otherwise, the cube remains inactive.
                        if active_neighbor_count == 3 {
                            *active = true;
                        }
                    }
                }
            }
            new_cubes
        }

        fn _draw_points(points: &Cubes) {
            let mut z_axis = HashMap::new();
            for (key, value) in points {
                if !z_axis.contains_key(&key.2) {
                    z_axis.insert(key.2, vec![]);
                }

                if let Some(points) = z_axis.get_mut(&key.2) {
                    points.push((key.0, key.1, *value));
                }
            }
            let mut z_keys = z_axis.keys().collect::<Vec<&i32>>();
            z_keys.sort();
            for z in z_keys {
                eprintln!("Z-index: {}", z);
                let mut map = HashMap::new();
                let mut min_x = 0;
                let mut max_x = 0;
                let mut min_y = 0;
                let mut max_y = 0;
                if let Some(x_y) = z_axis.get(z) {
                    for xya in x_y {
                        if xya.0 < min_x {
                            min_x = xya.0;
                        }
                        if xya.1 < min_y {
                            min_y = xya.1
                        }
                        if xya.0 > max_x {
                            max_x = xya.0;
                        }
                        if xya.1 > max_y {
                            max_y = xya.1;
                        }
                        map.insert((xya.0, xya.1), xya.2);
                    }
                }
                for y in min_y..max_y + 1 {
                    for x in min_x..max_x + 1 {
                        if let Some(active) = map.get(&(x, y)) {
                            if *active {
                                eprint!("#");
                            } else {
                                eprint!(".");
                            }
                        }
                    }
                    eprintln!();
                }
                eprintln!();
            }
            eprintln!();
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day17/example.txt");
            let mut points = parse_points(&contents);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);

            let mut active_count = 0;
            for key in points {
                if key.1 {
                    active_count += 1;
                }
            }
            assert_eq!(112, active_count);
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day17/input.txt");
            let mut points = parse_points(&contents);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);

            let mut active_count = 0;
            for key in points {
                if key.1 {
                    active_count += 1;
                }
            }

            assert_eq!(391, active_count);
        }
    }
}

pub mod b {
    #[cfg(test)]
    mod test {
        use crate::utils::read_input;
        use std::collections::HashMap;

        type HyperCubes = HashMap<(i32, i32, i32, i32), bool>;

        fn parse_points(contents: &str) -> HyperCubes {
            let mut cubes = HashMap::new();
            let z = 0;
            let w = 0;
            let mut y = 0;
            for line in contents.lines() {
                for (x, char) in line.chars().enumerate() {
                    if char == '#' {
                        cubes.insert((x as i32, y, z, w), true);
                    } else {
                        cubes.insert((x as i32, y, z, w), false);
                    }
                }
                y += 1;
            }
            cubes
        }

        fn cycle(cubes: &mut HyperCubes) -> HyperCubes {
            // Add the missing neighbors
            let mut missing_neighbors = vec![];
            for key in cubes.keys() {
                for x0 in key.0 - 1..key.0 + 2 {
                    for y0 in key.1 - 1..key.1 + 2 {
                        for z0 in key.2 - 1..key.2 + 2 {
                            for w0 in key.3 - 1..key.3 + 2 {
                                if &(x0, y0, z0, w0) != key {
                                    if !cubes.contains_key(&(x0, y0, z0, w0)) {
                                        missing_neighbors.push((x0, y0, z0, w0));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for missing_neighbor in missing_neighbors {
                cubes.insert(missing_neighbor, false);
            }
            let mut new_cubes = cubes.clone();
            for (key, _) in cubes.clone() {
                let mut active_neighbor_count = 0;
                for x0 in key.0 - 1..key.0 + 2 {
                    for y0 in key.1 - 1..key.1 + 2 {
                        for z0 in key.2 - 1..key.2 + 2 {
                            for w0 in key.3 - 1..key.3 + 2 {
                                if (x0, y0, z0, w0) != key {
                                    if let Some(true) = cubes.get(&(x0, y0, z0, w0)) {
                                        active_neighbor_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(active) = new_cubes.get_mut(&key) {
                    if *active {
                        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
                        // the cube remains active.
                        // Otherwise, the cube becomes inactive.
                        if active_neighbor_count != 2 && active_neighbor_count != 3 {
                            *active = false;
                        }
                    } else {
                        // If a cube is inactive but exactly 3 of its neighbors are active, the cube
                        // becomes active.
                        // Otherwise, the cube remains inactive.
                        if active_neighbor_count == 3 {
                            *active = true;
                        }
                    }
                }
            }
            new_cubes
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day17/example.txt");
            let mut points = parse_points(&contents);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            // draw_points(&points);

            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);

            let mut active_count = 0;
            for key in points {
                if key.1 {
                    active_count += 1;
                }
            }
            assert_eq!(848, active_count);
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day17/input.txt");
            let mut points = parse_points(&contents);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);
            points = cycle(&mut points);

            let mut active_count = 0;
            for key in points {
                if key.1 {
                    active_count += 1;
                }
            }

            assert_eq!(2264, active_count);
        }
    }
}
