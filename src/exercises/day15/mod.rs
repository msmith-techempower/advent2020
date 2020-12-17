#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn play_game(mut starting_values: Vec<usize>, last_turn: usize) -> usize {
        let mut spoken = HashMap::new();

        // sentinel
        spoken.insert(0, (0, 0));

        let mut turn = 1;
        let mut last_turn_value = 0;

        starting_values.reverse();
        while !starting_values.is_empty() {
            if let Some(next) = starting_values.pop() {
                spoken.insert(next, (turn, 0));
                last_turn_value = next;
                turn += 1;
            }
        }

        while turn <= last_turn {
            let last = *spoken.get(&last_turn_value).unwrap();
            if last.1 == 0 {
                // First time seeing it other than the last time.
                let zero = *spoken.get(&0).unwrap();
                spoken.insert(0, (turn, zero.0));
                last_turn_value = 0;
            } else {
                // We have seen it before at `turn = last.0`
                let next_spoken_number = last.0 - last.1;
                if !spoken.contains_key(&next_spoken_number) {
                    spoken.insert(next_spoken_number, (turn, 0));
                    last_turn_value = next_spoken_number;
                } else {
                    let next = *spoken.get(&next_spoken_number).unwrap();
                    spoken.insert(next_spoken_number, (turn, next.0));
                    last_turn_value = next_spoken_number;
                }
            }
            turn += 1;
        }

        last_turn_value
    }

    #[test]
    fn example() {
        let answer = play_game(vec![0, 3, 6], 2020);
        assert_eq!(436, answer);
        let answer = play_game(vec![1, 3, 2], 2020);
        assert_eq!(1, answer);
        let answer = play_game(vec![2, 1, 3], 2020);
        assert_eq!(10, answer);
        let answer = play_game(vec![1, 2, 3], 2020);
        assert_eq!(27, answer);
        let answer = play_game(vec![2, 3, 1], 2020);
        assert_eq!(78, answer);
        let answer = play_game(vec![3, 2, 1], 2020);
        assert_eq!(438, answer);
        let answer = play_game(vec![3, 1, 2], 2020);
        assert_eq!(1836, answer);
    }

    #[test]
    fn input() {
        let answer = play_game(vec![0, 13, 1, 8, 6, 15], 2020);

        assert_eq!(1618, answer);
    }

    #[test]
    fn input2() {
        // Takes a long time - don't run by default.
        // let answer = play_game(vec![0, 13, 1, 8, 6, 15], 30_000_000);
        //
        // assert_eq!(548_531, answer);
    }
}
