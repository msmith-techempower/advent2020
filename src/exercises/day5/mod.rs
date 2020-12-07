#[derive(Debug)]
pub struct BoardingPass {
    row: i32,
    column: i32,
    seat_id: i32,
}

/// The first 7 characters will either be F or B; these specify exactly one of
/// the 128 rows on the plane (numbered 0 through 127). Each letter tells you
/// which half of a region the given seat is in. Start with the whole list of
/// rows; the first letter indicates whether the seat is in the front (0
/// through 63) or the back (64 through 127). The next letter indicates which
/// half of that region the seat is in, and so on until you're left with
/// exactly one row.
///
/// For example, consider just the first seven characters of FBFBBFFRLR:
///
///     Start by considering the whole range, rows 0 through 127.
///     F means to take the lower half, keeping rows 0 through 63.
///     B means to take the upper half, keeping rows 32 through 63.
///     F means to take the lower half, keeping rows 32 through 47.
///     B means to take the upper half, keeping rows 40 through 47.
///     B keeps rows 44 through 47.
///     F keeps rows 44 through 45.
///     The final F keeps the lower of the two, row 44.
///
/// The last three characters will be either L or R; these specify exactly one
/// of the 8 columns of seats on the plane (numbered 0 through 7). The same
/// process as above proceeds again, this time with only three steps. L means
/// to keep the lower half, while R means to keep the upper half.
///
/// Every seat also has a unique seat ID: multiply the row by 8, then add the
/// column.
pub fn process_day5a_input(contents: &str) -> Vec<BoardingPass> {
    let mut to_ret = vec![];

    for line in contents.lines() {
        let mut row = 0;
        let mut f = 0;
        let mut b = 127;
        for (index, char) in line[..7].chars().enumerate() {
            match &char {
                'F' => {
                    if index == 6 {
                        row = f;
                    } else {
                        b = f + (b - f) / 2;
                    }
                }
                'B' => {
                    if index == 6 {
                        row = b;
                    } else {
                        f = f + (b - f) / 2 + 1;
                    }
                }
                _ => panic!("Got a non-F/B"),
            }
        }

        let mut column = 0;
        f = 0;
        b = 7;
        for (index, char) in line[7..].chars().enumerate() {
            match &char {
                'L' => {
                    if index == 2 {
                        column = f;
                    } else {
                        b = f + (b - f) / 2;
                    }
                }
                'R' => {
                    if index == 2 {
                        column = b;
                    } else {
                        f = f + (b - f) / 2 + 1;
                    }
                }
                _ => panic!("Got a non-L/R"),
            }
        }

        let seat_id = row * 8 + column;

        to_ret.push(BoardingPass {
            row,
            column,
            seat_id,
        })
    }

    to_ret
}

#[cfg(test)]
mod tests {
    use crate::exercises::day5::process_day5a_input;
    use crate::utils::read_input;

    #[test]
    fn example_input_is_correct() {
        let contents = read_input("src/exercises/day5/example.txt");
        let boarding_passes = process_day5a_input(&contents);

        let first = boarding_passes.get(0).expect("First");
        assert_eq!(70, first.row);
        assert_eq!(7, first.column);
        assert_eq!(567, first.seat_id);

        let second = boarding_passes.get(1).expect("Second");
        assert_eq!(14, second.row);
        assert_eq!(7, second.column);
        assert_eq!(119, second.seat_id);

        let third = boarding_passes.get(2).expect("Third");
        assert_eq!(102, third.row);
        assert_eq!(4, third.column);
        assert_eq!(820, third.seat_id);
    }

    #[test]
    fn day5a() {
        let contents = read_input("src/exercises/day5/input.txt");
        let boarding_passes = process_day5a_input(&contents);
        let mut highest = 0;
        for boarding_pass in boarding_passes {
            if boarding_pass.seat_id > highest {
                highest = boarding_pass.seat_id;
            }
        }

        assert_eq!(987, highest);
    }

    #[test]
    fn day5b() {
        let contents = read_input("src/exercises/day5/input.txt");
        let mut boarding_passes = process_day5a_input(&contents);
        boarding_passes.sort_by(|a, b| a.seat_id.cmp(&b.seat_id));

        let mut previous_boarding_pass = boarding_passes.get(0).expect("There is a first element!");
        let mut seat_id = 0;
        for boarding_pass in &boarding_passes[1..] {
            if boarding_pass.seat_id - previous_boarding_pass.seat_id != 1 {
                seat_id = previous_boarding_pass.seat_id + 1;
            }
            previous_boarding_pass = boarding_pass;
        }

        assert_eq!(603, seat_id);
    }
}
