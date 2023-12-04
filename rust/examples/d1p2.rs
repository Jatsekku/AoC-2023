use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Digit {
    value: u8,
    position: usize,
}

impl Digit {
    fn from_words(line: &str) -> Option<(Digit, Digit)> {
        const WORDS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let first_digit = WORDS
            .into_iter()
            .enumerate()
            .filter_map(|(idx, word)| {
                if let Some(pos) = line.find(word) {
                    Some((idx + 1, pos))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.cmp(&b.1))?;

        let first_digit = Digit {
            value: first_digit.0 as u8,
            position: first_digit.1,
        };

        let last_digit = WORDS
            .into_iter()
            .enumerate()
            .filter_map(|(idx, word)| {
                if let Some(position) = line.rfind(word) {
                    Some((idx + 1, position))
                } else {
                    None
                }
            })
            .max_by(|a, b| a.1.cmp(&b.1));

        let last_digit = match last_digit {
            Some((val, pos)) => Digit {
                value: val as u8,
                position: pos,
            },
            None => first_digit,
        };

        Some((first_digit, last_digit))
    }

    fn from_digit_chars(line: &str) -> Option<(Digit, Digit)> {
        // Split line into chars and filter out everything that is not 0-9 digit
        let mut digits = line.chars().enumerate().filter(|(_, c)| c.is_digit(10));

        let first_digit = digits.next()?;
        let first_digit = Digit {
            value: first_digit.1.to_digit(10)? as u8,
            position: first_digit.0,
        };

        // Get last char (treat first digit also as last digit if there are no more chars)
        let last_digit = digits.last();
        let last_digit = match last_digit {
            Some((pos, val)) => Digit {
                value: val.to_digit(10)? as u8,
                position: pos,
            },
            None => first_digit,
        };

        Some((first_digit, last_digit))
    }

    fn is_earlier(&self, other: &Digit) -> bool {
        self.position < other.position
    }

    fn is_later(&self, other: &Digit) -> bool {
        self.position > other.position
    }
}

fn line_to_number(line: &str) -> Result<u32, ()> {
    // Split line into chars and filter out everything that is not 0-9 digit

    let da = Digit::from_words(line);
    let db = Digit::from_digit_chars(line);

    let final_digits = match (da, db) {
        // No digits
        (None, None) => None,
        // Spelled digits and arabic digits detected
        (Some((da_first, da_last)), Some((db_first, db_last))) => Some((
            if da_first.is_earlier(&db_first) {
                da_first
            } else {
                db_first
            },
            if da_last.is_later(&db_last) {
                da_last
            } else {
                db_last
            },
        )),
        (Some(d), None) => Some(d),
        (None, Some(d)) => Some(d),
    };

    let (d_start, d_last) = final_digits.ok_or(())?;
    Ok((d_start.value * 10 + d_last.value) as u32)
}

fn recover_calibration_value(input: &str) -> Result<u32, ()> {
    //split input to lines and apply line_to_number() fo each line, sum all at the end
    input.lines().map(|l| line_to_number(l)).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/d1p2_my_input").expect("Input file error");
    let calibration_value = recover_calibration_value(&input).unwrap();
    println!("Calibration value: {calibration_value}");
}

#[cfg(test)]
mod tests {
    use crate::Digit;

    #[test]
    fn digit_from_words() {
        assert_eq!(
            Some((
                Digit {
                    value: 1,
                    position: 0
                },
                Digit {
                    value: 1,
                    position: 4,
                }
            )),
            Digit::from_words("one7one")
        );
    }

    #[test]
    fn digit_from_digit_chars() {
        assert_eq!(
            Some((
                Digit {
                    value: 4,
                    position: 0
                },
                Digit {
                    value: 2,
                    position: 15
                }
            )),
            Digit::from_digit_chars("4nineeightseven2")
        );
    }
}
