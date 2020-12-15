type Seats = Vec<Vec<Option<bool>>>;

pub fn count_occupied_seats(seats: &Seats) -> usize {
    let mut to_ret = 0;
    for row in seats {
        let mut occupied_seats = 0;
        for seat in row {
            if let Some(seat) = seat {
                if *seat {
                    occupied_seats += 1;
                }
            }
        }
        to_ret += occupied_seats;
    }

    to_ret
}

pub fn parse_input(contents: &str) -> Seats {
    let mut to_ret = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for char in line.chars() {
            match char {
                'L' => row.push(Some(false)),
                '#' => row.push(Some(true)),
                '.' => row.push(None),
                _ => panic!("Invalid char: {}", char),
            }
        }
        to_ret.push(row);
    }

    to_ret
}

fn _print_seats(seats: &Seats) {
    for row in seats {
        for seat in row {
            match *seat {
                Some(false) => eprint!("L"),
                Some(true) => eprint!("#"),
                None => eprint!("."),
            }
        }
        eprintln!();
    }
    eprintln!();
}

pub mod a {
    #[cfg(test)]
    mod test {
        use crate::exercises::day11::{count_occupied_seats, parse_input, Seats};
        use crate::utils::read_input;

        fn run_round(seats: &Seats) -> (Seats, bool) {
            let mut to_ret = seats.clone();

            // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
            // occupied.
            // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the
            // seat becomes empty.
            // Otherwise, the seat's state does not change.
            for (row_index, row) in seats.iter().enumerate() {
                for (seat_index, seat) in row.iter().enumerate() {
                    match seat {
                        Some(false) => {
                            let mut can_sit = true;
                            if row_index > 0 {
                                if let Some(previous_row) = seats.get(row_index - 1) {
                                    let from = if seat_index == 0 { 0 } else { seat_index - 1 };
                                    let to = if seat_index == row.len() - 1 {
                                        row.len() - 1
                                    } else {
                                        seat_index + 1
                                    };
                                    for index in from..to + 1 {
                                        if let Some(seat) = previous_row.get(index) {
                                            if let Some(seat) = seat {
                                                can_sit &= !seat;
                                            }
                                        }
                                    }
                                }
                            }
                            if seat_index > 0 {
                                if let Some(neighbor) = row.get(seat_index - 1) {
                                    if let Some(seat) = neighbor {
                                        can_sit &= !seat;
                                    }
                                }
                            }
                            if seat_index < row.len() - 1 {
                                if let Some(neighbor) = row.get(seat_index + 1) {
                                    if let Some(seat) = neighbor {
                                        can_sit &= !seat;
                                    }
                                }
                            }
                            if row_index < seats.len() - 1 {
                                if let Some(next_row) = seats.get(row_index + 1) {
                                    let from = if seat_index == 0 { 0 } else { seat_index - 1 };
                                    let to = if seat_index == row.len() - 1 {
                                        row.len() - 1
                                    } else {
                                        seat_index + 1
                                    };
                                    for index in from..to + 1 {
                                        if let Some(seat) = next_row.get(index) {
                                            if let Some(seat) = seat {
                                                can_sit &= !seat;
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(row) = to_ret.get_mut(row_index) {
                                if let Some(seat) = row.get_mut(seat_index) {
                                    *seat = Some(can_sit);
                                }
                            }
                        }
                        Some(true) => {
                            let mut adjacent_sitters_count = 0;
                            if row_index > 0 {
                                if let Some(previous_row) = seats.get(row_index - 1) {
                                    let from = if seat_index == 0 { 0 } else { seat_index - 1 };
                                    let to = if seat_index == row.len() - 1 {
                                        row.len()
                                    } else {
                                        seat_index + 1
                                    };
                                    for index in from..to + 1 {
                                        if let Some(seat) = previous_row.get(index) {
                                            if let Some(seat) = seat {
                                                if *seat {
                                                    adjacent_sitters_count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if seat_index > 0 {
                                if let Some(neighbor) = row.get(seat_index - 1) {
                                    if let Some(seat) = neighbor {
                                        if *seat {
                                            adjacent_sitters_count += 1;
                                        }
                                    }
                                }
                            }
                            if seat_index < row.len() - 1 {
                                if let Some(neighbor) = row.get(seat_index + 1) {
                                    if let Some(seat) = neighbor {
                                        if *seat {
                                            adjacent_sitters_count += 1;
                                        }
                                    }
                                }
                            }
                            if row_index < seats.len() - 1 {
                                if let Some(next_row) = seats.get(row_index + 1) {
                                    let from = if seat_index == 0 { 0 } else { seat_index - 1 };
                                    let to = if seat_index == row.len() - 1 {
                                        row.len()
                                    } else {
                                        seat_index + 1
                                    };
                                    for index in from..to + 1 {
                                        if let Some(seat) = next_row.get(index) {
                                            if let Some(seat) = seat {
                                                if *seat {
                                                    adjacent_sitters_count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(row) = to_ret.get_mut(row_index) {
                                if let Some(seat) = row.get_mut(seat_index) {
                                    if adjacent_sitters_count >= 4 {
                                        *seat = Some(false);
                                    }
                                }
                            }
                        }
                        None => {} // Floor
                    }
                }
            }

            let mut equal = true;
            for (row_index, row) in to_ret.iter().enumerate() {
                for (seat_index, seat) in row.iter().enumerate() {
                    let original = seats
                        .get(row_index)
                        .expect("Same row count")
                        .get(seat_index)
                        .expect("Same row length");
                    equal &= seat == original;
                }
            }

            (to_ret, equal)
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day11/example.txt");
            let mut seats = parse_input(&contents);

            let mut settled = false;
            while !settled {
                let results = run_round(&seats);
                seats = results.0;
                settled = results.1;
            }

            assert_eq!(37, count_occupied_seats(&seats));
        }

        #[test]
        fn input() {
            let contents = read_input("src/exercises/day11/input.txt");
            let mut seats = parse_input(&contents);

            let mut settled = false;
            while !settled {
                let results = run_round(&seats);
                seats = results.0;
                settled = results.1;
            }

            assert_eq!(2277, count_occupied_seats(&seats));
        }
    }
}

pub mod b {
    #[cfg(test)]
    mod tests {
        use crate::exercises::day11::{count_occupied_seats, parse_input, Seats};
        use crate::utils::read_input;

        fn run_round(seats: &Seats) -> (Seats, bool) {
            let mut to_ret = seats.clone();

            for (row_index, row) in seats.iter().enumerate() {
                for (seat_index, seat) in row.iter().enumerate() {
                    if seat.is_none() {
                        continue;
                    }

                    let mut can_sit = true;
                    let mut seated_in_view = 0;

                    for index in (0..row_index).rev() {
                        if let Some(prev_row) = seats.get(index) {
                            if let Some(seat) = prev_row.get(seat_index) {
                                if let Some(true) = seat {
                                    // 12:00
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                    }
                    let mut index = 1;
                    while index <= row.len() && row_index >= index {
                        if let Some(prev_row) = seats.get(row_index - index) {
                            if let Some(seat) = prev_row.get(seat_index + index) {
                                if let Some(true) = seat {
                                    // 1:30
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                        index += 1;
                    }
                    for index in seat_index + 1..row.len() {
                        if let Some(seat) = row.get(index) {
                            if let Some(true) = seat {
                                // 3:00
                                can_sit &= false;
                                seated_in_view += 1;
                                break;
                            } else if let Some(false) = seat {
                                break;
                            }
                        }
                    }
                    let mut index = 1;
                    while seat_index + index < row.len() && row_index + index < seats.len() {
                        if let Some(next_row) = seats.get(row_index + index) {
                            if let Some(seat) = next_row.get(seat_index + index) {
                                if let Some(true) = seat {
                                    // 4:30
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                        index += 1;
                    }
                    for index in row_index + 1..seats.len() {
                        if let Some(next_row) = seats.get(index) {
                            if let Some(seat) = next_row.get(seat_index) {
                                if let Some(true) = seat {
                                    // 6:00
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                    }
                    let mut index = 1;
                    while row_index + index <= seats.len() && index <= seat_index {
                        if let Some(next_row) = seats.get(row_index + index) {
                            if let Some(seat) = next_row.get(seat_index - index) {
                                if let Some(true) = seat {
                                    // 7:30
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                        index += 1;
                    }
                    for index in (0..seat_index).rev() {
                        if let Some(seat) = row.get(index) {
                            if let Some(true) = seat {
                                // 9:00
                                can_sit &= false;
                                seated_in_view += 1;
                                break;
                            } else if let Some(false) = seat {
                                break;
                            }
                        }
                    }
                    let min = usize::min(seat_index, row_index);
                    for index in 1..min + 1 {
                        if let Some(prev_row) = seats.get(row_index - index) {
                            if let Some(seat) = prev_row.get(seat_index - index) {
                                if let Some(true) = seat {
                                    // 10:30
                                    can_sit &= false;
                                    seated_in_view += 1;
                                    break;
                                } else if let Some(false) = seat {
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(row) = to_ret.get_mut(row_index) {
                        if let Some(seat) = row.get_mut(seat_index) {
                            match seat {
                                Some(true) => {
                                    if seated_in_view >= 5 {
                                        *seat = Some(false);
                                    }
                                }
                                Some(false) => {
                                    *seat = Some(can_sit);
                                }
                                None => {}
                            }
                        }
                    }
                }
            }

            let mut equal = true;
            for (row_index, row) in to_ret.iter().enumerate() {
                for (seat_index, seat) in row.iter().enumerate() {
                    let original = seats
                        .get(row_index)
                        .expect("Same row count")
                        .get(seat_index)
                        .expect("Same row length");
                    equal &= seat == original;
                }
            }

            (to_ret, equal)
        }

        #[test]
        fn example() {
            let contents = read_input("src/exercises/day11/example.txt");
            let mut seats = parse_input(&contents);

            let mut settled = false;
            while !settled {
                let results = run_round(&seats);
                seats = results.0;
                settled = results.1;
            }

            assert_eq!(26, count_occupied_seats(&seats));
        }

        #[test]
        fn b() {
            let contents = read_input("src/exercises/day11/input.txt");
            let mut seats = parse_input(&contents);

            let mut settled = false;
            while !settled {
                let results = run_round(&seats);
                seats = results.0;
                settled = results.1;
            }

            assert_eq!(2066, count_occupied_seats(&seats));
        }
    }
}
