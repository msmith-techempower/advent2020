#[cfg(test)]
mod test {
    use crate::utils::read_input;
    use std::collections::HashMap;
    use std::ops::Range;

    type Rules = HashMap<String, Vec<Range<usize>>>;
    fn parse_rules(rules: &str) -> Rules {
        let mut to_ret = Rules::new();

        for line in rules.lines() {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let mut rule_set = vec![];
            if let Some(rule) = parts.get(1) {
                for range in rule.split(" or ") {
                    let range = range.split('-').collect::<Vec<&str>>();
                    let from = str::parse::<usize>(range.get(0).expect("Zero index"))
                        .expect("Valid integer");
                    let to = str::parse::<usize>(range.get(1).expect("One index"))
                        .expect("Valid integer")
                        + 1;
                    rule_set.push(from..to);
                }
            }
            to_ret.insert(parts.get(0).expect("Zero index").to_string(), rule_set);
        }

        to_ret
    }

    fn find_error_product(nearby_tickets: &str, rules: &Rules) -> usize {
        let mut to_ret = 0;

        for ticket in nearby_tickets.lines() {
            for value in ticket.split(',') {
                let value = str::parse::<usize>(value).expect("Valid integer");
                let mut valid_value = false;
                for valid_ranges in rules.values() {
                    if valid_value {
                        break;
                    }
                    for valid_range in valid_ranges {
                        if valid_range.contains(&value) {
                            valid_value = true;
                            break;
                        }
                    }
                }
                if !valid_value {
                    to_ret += value;
                }
            }
        }

        to_ret
    }

    #[test]
    fn example() {
        let rules = read_input("src/exercises/day16/example_rules.txt");
        let rules = parse_rules(&rules);
        let nearby_tickets = read_input("src/exercises/day16/example_nearby_tickets.txt");
        let error_product = find_error_product(&nearby_tickets, &rules);

        assert_eq!(71, error_product);
    }

    #[test]
    fn input() {
        let rules = read_input("src/exercises/day16/input_rules.txt");
        let rules = parse_rules(&rules);
        let nearby_tickets = read_input("src/exercises/day16/input_nearby_tickets.txt");
        let error_product = find_error_product(&nearby_tickets, &rules);

        assert_eq!(27_870, error_product);
    }

    #[test]
    fn part_2() {
        let rules = read_input("src/exercises/day16/input_rules.txt");
        let rules = parse_rules(&rules);
        let my_ticket_str = read_input("src/exercises/day16/input_ticket.txt");
        let mut my_ticket = vec![];
        for val in my_ticket_str.split(',') {
            my_ticket.push(str::parse::<usize>(val).expect("Valid integer"));
        }
        let mut nearby_tickets = vec![];
        for line in read_input("src/exercises/day16/valid_tickets.txt").lines() {
            let mut nearby_ticket = vec![];
            for val in line.split(',') {
                nearby_ticket.push(str::parse::<usize>(val).expect("Valid integer"));
            }
            nearby_tickets.push(nearby_ticket);
        }

        let mut better_rules = vec![];
        for rule in rules {
            better_rules.push(rule);
        }

        let mut valid_rules = vec![vec![]; 20];
        for (value_index, ticket_value) in my_ticket.iter().enumerate() {
            for rule in &better_rules {
                let rule1 = rule.1.get(0).expect("Zero index");
                let rule2 = rule.1.get(1).expect("One index");
                if rule1.contains(ticket_value) || rule2.contains(ticket_value) {
                    valid_rules
                        .get_mut(value_index)
                        .expect("Valid index")
                        .push(rule);
                }
            }
        }

        for ticket in nearby_tickets {
            for (value_index, ticket_value) in ticket.iter().enumerate() {
                let mut valid_rules_at_value_index = vec![];
                for rule in valid_rules.get(value_index).expect("Valid value index") {
                    let rule1 = rule.1.get(0).expect("Zero index");
                    let rule2 = rule.1.get(1).expect("One index");
                    if rule1.contains(ticket_value) || rule2.contains(ticket_value) {
                        valid_rules_at_value_index.push(*rule);
                    }
                }
                if let Some(thing) = valid_rules.get_mut(value_index) {
                    *thing = valid_rules_at_value_index;
                }
            }
        }

        // NOTE: I did the rest pretty much by hand... so, I'll just explain what happens.
        //
        // Here, `valid_rules` is a vec representing indexes of values on every ticket.
        // `valid_rules[0]` is the vec of rules that are valid for the `0`th value on every
        // ticket.

        // If you uncomment this debugging printout...
        // for rules in valid_rules {
        //     eprintln!("{:?}", rules);
        // }
        // it ends up printing the following:
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("seat", [29..791, 809..964]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("arrival location", [40..574, 582..965]), ("wagon", [38..112, 127..964]), ("train", [41..446, 461..953]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("seat", [29..791, 809..964]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("wagon", [38..112, 127..964]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("seat", [29..791, 809..964]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("arrival location", [40..574, 582..965]), ("wagon", [38..112, 127..964]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("departure station", [26..401, 413..966]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("zone", [28..227, 234..952]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("seat", [29..791, 809..964]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("arrival location", [40..574, 582..965]), ("arrival station", [42..840, 857..965]), ("wagon", [38..112, 127..964]), ("train", [41..446, 461..953]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("seat", [29..791, 809..964]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("arrival location", [40..574, 582..965]), ("arrival station", [42..840, 857..965]), ("wagon", [38..112, 127..964]), ("train", [41..446, 461..953]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("wagon", [38..112, 127..964]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("route", [32..747, 759..973]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("arrival platform", [26..90, 105..971]), ("duration", [35..353, 371..972])]
        // [("arrival platform", [26..90, 105..971]), ("row", [45..462, 467..956]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("arrival track", [30..503, 510..953]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("wagon", [38..112, 127..964]), ("price", [45..468, 486..971]), ("departure station", [26..401, 413..966]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("arrival platform", [26..90, 105..971]), ("row", [45..462, 467..956]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("arrival platform", [26..90, 105..971]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure date", [32..761, 783..956]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("departure station", [26..401, 413..966]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]
        // [("type", [40..684, 701..975]), ("departure location", [49..628, 650..971]), ("arrival platform", [26..90, 105..971]), ("departure platform", [28..376, 390..972]), ("row", [45..462, 467..956]), ("class", [39..883, 888..957]), ("departure time", [25..421, 439..962]), ("departure track", [45..394, 400..955]), ("duration", [35..353, 371..972])]

        // Note: the above has only 1 index with only 1 valid rule: index 2 = duration.
        // Therefore, I can safely remove `("duration", [35..353, 371..972])` from every *other* ruleset
        // since no other index can be duration.
        // Next, index 11 changes
        //   from `[("arrival platform", [26..90, 105..971]), ("duration", [35..353, 371..972])]`
        //   to   `[("arrival platform", [26..90, 105..971])]`

        // Rinse and repeat until every vec is length 1, which reveals the following:
        //     [("price", [45..468, 486..971])]
        //     [("train", [41..446, 461..953])]
        //     [("duration", [35..353, 371..972])]
        //     [("seat", [29..791, 809..964])]
        //     [("arrival location", [40..574, 582..965])]
        //     [("departure location", [49..628, 650..971])]
        //     [("arrival track", [30..503, 510..953])]
        //     [("zone", [28..227, 234..952])]
        //     [("arrival station", [42..840, 857..965])]
        //     [("route", [32..747, 759..973])]
        //     [("departure date", [32..761, 783..956])]
        //     [("arrival platform", [26..90, 105..971])]
        //     [("row", [45..462, 467..956])]
        //     [("departure track", [45..394, 400..955])]
        //     [("wagon", [38..112, 127..964])]
        //     [("type", [40..684, 701..975])]
        //     [("class", [39..883, 888..957])]
        //     [("departure platform", [28..376, 390..972])]
        //     [("departure station", [26..401, 413..966])]
        //     [("departure time", [25..421, 439..962])]

        // My ticket is
        //      107,109,163,127,167,157,139,67,131,59,151,53,73,83,61,89,71,149,79,137
        // and so mapping each value to its index, we find the six departure values
        // about which we care:
        //      157 * 151 * 83 * 149 * 79 * 137

        assert_eq!(
            3_173_135_507_987,
            157_u64 * 151_u64 * 83_u64 * 149_u64 * 79_u64 * 137_u64
        );
    }
}
