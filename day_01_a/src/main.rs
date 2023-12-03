use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut result = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        let mut first_digit = None;
        let mut last_digit = None;

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        }

        let first_digit = first_digit.unwrap_or('0').to_digit(10).unwrap();
        let last_digit = last_digit.unwrap_or('0').to_digit(10).unwrap();

        let number = first_digit * 10 + last_digit;
        result += number;
    }

    println!("{}", result);
    Ok(())
}
