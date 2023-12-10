use nom::{bytes::complete::take_till, character::complete::digit1, IResult};
use nom_locate::{position, LocatedSpan};
use std::cmp::{max, min};
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Number {
    value: u32,
    length: usize,
    line: usize,
    column: usize,
}

fn number(input: Span) -> IResult<Span, Number> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, position) = position(input)?;
    let (input, digits) = digit1(input)?;
    Ok((
        input,
        Number {
            value: digits.fragment().parse().unwrap(),
            length: digits.fragment().len(),
            line: position.location_line() as usize - 1,
            column: position.get_column() - 1,
        },
    ))
}

fn get_numbers(input: Span) -> IResult<Span, Vec<Number>> {
    let (input, numbers) = nom::multi::many1(number)(input)?;
    Ok((input, numbers))
}

fn number_has_symbol_neighbors(number: &Number, rows: &[&str]) -> bool {
    let x_min = max(0, number.column as i32 - 1) as usize;
    let x_max = min(number.column + number.length, rows[0].len() - 1);

    let y_min = max(0, number.line as i32 - 1) as usize;
    let y_max = min(number.line as i32 + 1, rows.len() as i32 - 1) as usize;

    println!("x: {:?}, y: {:?}", x_min..x_max, y_min..y_max);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let character = rows[y].chars().nth(x).unwrap();
            print!("{}", character);
            if !character.is_numeric() && character != '.' {
                return true;
            }
        }
        println!();
    }

    false
}

fn main() {
    let input = include_str!("../input.txt");

    let rows = input.lines().collect::<Vec<_>>();

    let input = Span::new(input);
    let (_, numbers) = get_numbers(input).unwrap();

    let mut result = 0;
    for number in numbers {
        println!("{:?}", number);
        if number_has_symbol_neighbors(&number, &rows) {
            result += number.value;
        }
        println!();
    }

    println!("{}", result);
}
