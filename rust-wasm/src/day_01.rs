crate::solution!(1, "Secret Entrance", "test", solve_a, solve_b);

use nom::{
    IResult, Parser,
    character::complete::{self, line_ending, one_of},
    multi::separated_list1,
};

pub fn solve_a(input: &str) -> u32 {
    let (rest, directions) = parse_a(input).expect("Failed to parse input");
    assert!(rest.is_empty(), "Unparsed input remaining: {}", rest);

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

pub fn solve_b(input: &str) -> u32 {
    let (rest, directions) = parse_a(input).expect("Failed to parse input");
    assert!(rest.is_empty(), "Unparsed input remaining: {}", rest);

    let mut pos: i32 = 50;
    let mut times_at_zero = 0;
    for direction in directions {
        match direction {
            Direction::Left(dist) => {
                if dist as i32 >= pos {
                    times_at_zero += (((dist + 100) as i32 - pos) as f64 / 100.0).floor() as u32;
                    if pos == 0 {
                        times_at_zero -= 1;
                    }
                }
                pos -= dist as i32;
            }
            Direction::Right(dist) => {
                if (pos + dist as i32) >= 100 {
                    times_at_zero += ((pos + dist as i32) as f64 / 100.0).floor() as u32;
                }
                pos += dist as i32
            }
        }
        pos = (pos + 10000000) % 100;
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

    static INPUT: &str = "L68
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

    #[test]
    fn test_parse_a() {
        let (remaining, directions) = parse_a(INPUT).expect("Failed to parse directions");
        assert_eq!(remaining, "");
        assert_eq!(directions.len(), 10);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(INPUT), 3);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(INPUT), 6);
    }

    #[test]
    fn test_wrap_around() {
        let input = "R1000";
        assert_eq!(solve_b(input), 10);
    }

    #[test]
    fn test_wrap_around2() {
        let input = "L1000";
        assert_eq!(solve_b(input), 10);
    }

    #[test]
    fn test_wrap_around3() {
        let input = "R150";
        assert_eq!(solve_b(input), 2);
        let input = "R150\nL150";
        assert_eq!(solve_b(input), 3);
        let input = "R150\nL200";
        assert_eq!(solve_b(input), 4);
        let input = "R150\nL210";
        assert_eq!(solve_b(input), 4);
        let input = "R151";
        assert_eq!(solve_b(input), 2);
        let input = "R151\nL200";
        assert_eq!(solve_b(input), 4);
        let input = "R151\nL220";
        assert_eq!(solve_b(input), 5);
    }

    #[test]
    fn edge_case_zero_start() {
        let input = "L50";
        assert_eq!(solve_b(input), 1);
        let input = "R50";
        assert_eq!(solve_b(input), 1);

        assert_eq!(solve_b("L50\nL100"), 2);
        assert_eq!(solve_b("R50\nL100"), 2);
        assert_eq!(solve_b("L50\nR100"), 2);
        assert_eq!(solve_b("R50\nR100"), 2);
    }
}
