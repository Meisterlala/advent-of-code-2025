crate::solution!(1, "The first day XXXXX", solve_a);

use nom::{
    IResult, Parser,
    character::complete::{self, line_ending, one_of},
    multi::{many1, separated_list1},
};

pub fn solve_a(input: &str) -> u32 {
    let (_, directions) = parse_a(input).expect("Failed to parse input");

    let mut pos: i32 = 50;
    let mut times_at_zero = 0;
    for direction in directions {
        match direction {
            Direction::Left(dist) => pos -= dist as i32,
            Direction::Right(dist) => pos += dist as i32,
        }

        // Wrap around at 100
        pos = (pos + 100) % 100;

        if pos == 0 {
            times_at_zero += 1;
        }
    }
    times_at_zero
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left(u32),
    Right(u32),
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir_char) = one_of("LR")(input)?;
    let (input, distance) = complete::u32(input)?;
    let direction = match dir_char {
        'L' => Direction::Left(distance),
        'R' => Direction::Right(distance),
        _ => unreachable!(),
    };
    Ok((input, direction))
}

fn parse_a(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, parse_direction).parse(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let (remaining, directions) = parse_a(input).expect("Failed to parse directions");
        assert_eq!(remaining, "");
        assert_eq!(directions.len(), 10);
    }

    #[test]
    fn test_solve_a() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(solve_a(input), 3);
    }
}
