use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn new(color: &str) -> Color {
        match color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Unknown color: {}", color),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

fn game_id(input: &str) -> IResult<&str, u32> {
    let (input, id) = delimited(
        tuple((tag("Game"), multispace1)),
        digit1,
        tuple((tag(":"), multispace0)),
    )(input)?;

    Ok((input, id.parse().unwrap()))
}

fn number_and_color(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, (number, _, color)) = tuple((
        digit1,
        multispace1,
        alt((tag("red"), tag("green"), tag("blue"))),
    ))(input)?;

    Ok((input, (number.parse().unwrap(), Color::new(color))))
}

fn cube_set(input: &str) -> IResult<&str, CubeSet> {
    separated_list1(tuple((tag(","), multispace0)), number_and_color)(input).map(
        |(input, numbers)| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for (number, color) in numbers {
                match color {
                    Color::Red => red = number,
                    Color::Green => green = number,
                    Color::Blue => blue = number,
                }
            }

            (input, CubeSet { red, green, blue })
        },
    )
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = game_id(input)?;
    let (input, cube_sets) = separated_list1(tuple((tag(";"), multispace0)), cube_set)(input)?;

    Ok((input, Game { id, cube_sets }))
}

fn main() {
    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;

    let stdin = io::stdin();
    let mut result = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let (_, game) = game(&line).unwrap();

        if !game
            .cube_sets
            .iter()
            .any(|cubes| cubes.red > max_reds || cubes.green > max_greens || cubes.blue > max_blues)
        {
            result += game.id;
        }
    }

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_id() {
        assert_eq!(game_id("Game 1: "), Ok(("", 1)));
        assert_eq!(game_id("Game 23: "), Ok(("", 23)));
        assert_eq!(game_id("Game 456: "), Ok(("", 456)));
        assert_eq!(game_id("Game   7:   "), Ok(("", 7)));
    }

    #[test]
    fn test_number_and_color() {
        assert_eq!(number_and_color("1 red"), Ok(("", (1, Color::Red))));
        assert_eq!(number_and_color("23 green"), Ok(("", (23, Color::Green))));
        assert_eq!(number_and_color("456 blue"), Ok(("", (456, Color::Blue))));
        assert_eq!(number_and_color("7    blue"), Ok(("", (7, Color::Blue))));
    }

    #[test]
    fn test_cube_set() {
        assert_eq!(
            cube_set("1 red, 23 green, 456 blue;"),
            Ok((
                ";",
                CubeSet {
                    red: 1,
                    green: 23,
                    blue: 456
                }
            ))
        );

        assert_eq!(
            cube_set("1 blue, 2 red;"),
            Ok((
                ";",
                CubeSet {
                    red: 2,
                    green: 0,
                    blue: 1
                }
            ))
        );

        assert_eq!(
            cube_set("3 green;"),
            Ok((
                ";",
                CubeSet {
                    red: 0,
                    green: 3,
                    blue: 0
                }
            ))
        );

        assert_eq!(
            cube_set("3 green"),
            Ok((
                "",
                CubeSet {
                    red: 0,
                    green: 3,
                    blue: 0
                }
            ))
        );
    }

    #[test]
    fn test_game() {
        assert_eq!(
            game("Game 1: 1 red, 23 green, 456 blue"),
            Ok((
                "",
                Game {
                    id: 1,
                    cube_sets: vec![CubeSet {
                        red: 1,
                        green: 23,
                        blue: 456
                    }]
                }
            ))
        );

        assert_eq!(
            game("Game 2: 1 blue, 2 red; 3 green"),
            Ok((
                "",
                Game {
                    id: 2,
                    cube_sets: vec![
                        CubeSet {
                            red: 2,
                            green: 0,
                            blue: 1
                        },
                        CubeSet {
                            red: 0,
                            green: 3,
                            blue: 0
                        }
                    ]
                }
            ))
        );
    }
}
