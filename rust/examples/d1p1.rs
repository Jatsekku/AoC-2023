use std::fs;

fn line_to_number(line: &str) -> Result<u32, ()> {
    // Split line into chars and filter out everything that is not 0-9 digit
    let mut digits = line.chars().filter(|c| c.is_digit(10));

    // Get first char
    let first_digit = digits.next().ok_or(())?.to_digit(10).ok_or(())?;

    // Get last char (treat first digit also as last digit if there are no more chars)
    let last_digit = match digits.last() {
        Some(x) => x.to_digit(10).ok_or(())?,
        None => first_digit,
    };

    // Calculate and return result
    Ok(first_digit * 10 + last_digit)
}

fn recover_calibration_value(input: &str) -> Result<u32, ()> {
    //split input to lines and apply line_to_number() fo each line, sum all at the end
    input.lines().map(|l| line_to_number(l)).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/d1p1_my_input").expect("Input file error");
    let calibration_value = recover_calibration_value(&input).unwrap();
    println!("Calibration value: {calibration_value}");
}

#[cfg(test)]
mod tests {
    use crate::line_to_number;

    #[test]
    fn line_to_number_2digit_input() {
        assert_eq!(Ok(73), line_to_number("fourp783fiveseventhree"))
    }

    #[test]
    fn line_to_number_1digit_input() {
        assert_eq!(Ok(77), line_to_number("treb7uchet"))
    }
}
