use std::io::{self, BufRead};

fn get_digit(s: &str) -> Option<u32> {
    if let Some(first) = s.chars().next() {
        if first.is_digit(10) {
            return first.to_digit(10);
        }
    }

    match s {
        _ if s.starts_with("one") => Some(1),
        _ if s.starts_with("two") => Some(2),
        _ if s.starts_with("three") => Some(3),
        _ if s.starts_with("four") => Some(4),
        _ if s.starts_with("five") => Some(5),
        _ if s.starts_with("six") => Some(6),
        _ if s.starts_with("seven") => Some(7),
        _ if s.starts_with("eight") => Some(8),
        _ if s.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut result = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        let mut first_digit = None;
        let mut last_digit = None;

        for (i, _) in line.char_indices() {
            let value = get_digit(&line[i..]);
            println!("{}: {:?}", i, value);
            if value.is_some() {
                if first_digit.is_none() {
                    first_digit = value;
                }
                last_digit = value;
            }
        }

        let number = first_digit.unwrap() * 10 + last_digit.unwrap();
        result += number;
    }

    println!("{}", result);
    Ok(())
}
