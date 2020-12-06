#[derive(Default, Debug)]
pub struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}
impl Passport {
    fn is_valid_4a(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        // `cid` not required
        // self.cid.is_some()
    }

    fn is_valid_4b(&self) -> bool {
        self.is_valid_4a()
            && self.is_valid_birth_year()
            && self.is_valid_issue_year()
            && self.is_valid_expiration_year()
            && self.is_valid_height()
            && self.is_valid_hair_color()
            && self.is_valid_eye_color()
            && self.is_valid_passport_id()
            && self.is_valid_country_id()
    }

    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn is_valid_birth_year(&self) -> bool {
        if let Some(birth_year) = &self.byr {
            if let Ok(birth_year) = str::parse::<i32>(birth_year) {
                return (1920..2003).contains(&birth_year);
            }
        }

        false
    }

    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn is_valid_issue_year(&self) -> bool {
        if let Some(issue_year) = &self.iyr {
            if issue_year.len() == 4 {
                if let Ok(issue_year) = str::parse::<i32>(issue_year) {
                    return (2010..2021).contains(&issue_year);
                }
            }
        }

        false
    }

    /// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn is_valid_expiration_year(&self) -> bool {
        if let Some(expiration_year) = &self.eyr {
            if expiration_year.len() == 4 {
                if let Ok(expiration_year) = str::parse::<i32>(expiration_year) {
                    return (2020..2031).contains(&expiration_year);
                }
            }
        }

        false
    }

    /// hgt (Height) - a number followed by either cm or in:
    /// If cm, the number must be at least 150 and at most 193.
    /// If in, the number must be at least 59 and at most 76.
    fn is_valid_height(&self) -> bool {
        if let Some(height) = &self.hgt {
            match &height[height.len() - 2..] {
                "cm" => {
                    (150..194).contains(&str::parse::<i32>(&height[..height.len() - 2]).unwrap())
                }
                "in" => (59..77).contains(&str::parse::<i32>(&height[..height.len() - 2]).unwrap()),
                _ => false,
            }
        } else {
            false
        }
    }

    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn is_valid_hair_color(&self) -> bool {
        if let Some(hair_color) = &self.hcl {
            if hair_color.starts_with('#') {
                if hair_color.len() == 7 {
                    let mut valid = true;
                    for char in hair_color[1..].chars() {
                        match char {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a'
                            | 'b' | 'c' | 'd' | 'e' | 'f' => {}
                            _ => valid = false,
                        }
                    }

                    return valid;
                }
            }
        }

        false
    }

    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn is_valid_eye_color(&self) -> bool {
        if let Some(eye_color) = &self.ecl {
            return match eye_color.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };
        }

        false
    }

    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn is_valid_passport_id(&self) -> bool {
        if let Some(passport_id) = &self.pid {
            if passport_id.len() == 9 {
                return str::parse::<u32>(&passport_id).is_ok();
            }
        }

        false
    }

    /// cid (Country ID) - ignored, missing or not.
    fn is_valid_country_id(&self) -> bool {
        true
    }
}

pub fn count_valid_passports(passports: Vec<Passport>) -> usize {
    let mut count = 0;
    for passport in passports {
        if passport.is_valid_4a() {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::exercises::day4::{count_valid_passports, Passport};
    use crate::utils::read_input;

    fn process_passport(line: String) -> Passport {
        let mut passport = Passport::default();

        for part in line.split_whitespace() {
            match &part[0..3] {
                "byr" => passport.byr = Some(part[4..].to_string()),
                "iyr" => passport.iyr = Some(part[4..].to_string()),
                "eyr" => passport.eyr = Some(part[4..].to_string()),
                "hgt" => passport.hgt = Some(part[4..].to_string()),
                "hcl" => passport.hcl = Some(part[4..].to_string()),
                "ecl" => passport.ecl = Some(part[4..].to_string()),
                "pid" => passport.pid = Some(part[4..].to_string()),
                "cid" => passport.cid = Some(part[4..].to_string()),
                _ => {}
            }
        }

        passport
    }

    fn process_day4_input(contents: &str) -> Vec<Passport> {
        let mut passports = vec![];
        let mut buffer = String::new();
        for line in contents.lines() {
            if line.is_empty() {
                passports.push(process_passport(buffer.clone()));
                buffer.clear();
            } else {
                if !buffer.is_empty() {
                    buffer.push(' ');
                }
                buffer.push_str(line);
            }
        }
        if !buffer.is_empty() {
            passports.push(process_passport(buffer.clone()));
            buffer.clear();
        }

        passports
    }

    #[test]
    fn default_passport_is_invalid() {
        assert_eq!(false, Passport::default().is_valid_4a());
    }

    #[test]
    fn test_example_input() {
        let contents = read_input("src/exercises/day4/example_a.txt");
        let passports = process_day4_input(&contents);

        assert_eq!(2, count_valid_passports(passports));
    }

    #[test]
    fn test_day4a() {
        let contents = read_input("src/exercises/day4/input.txt");
        let passports = process_day4_input(&contents);

        assert_eq!(
            235,
            passports.iter().fold(0, |count, passport| {
                if passport.is_valid_4a() {
                    count + 1
                } else {
                    count
                }
            })
        )
    }

    #[test]
    fn test_day4b() {
        let contents = read_input("src/exercises/day4/input.txt");
        let passports = process_day4_input(&contents);

        assert_eq!(
            194,
            passports.iter().fold(0, |count, passport| {
                if passport.is_valid_4b() {
                    count + 1
                } else {
                    count
                }
            })
        )
    }
}
