crate::solution!(
    3,
    "Lobby",
    "If you look at any prefix of the input that excludes the last n(2/12) digits, the largest digit in that prefix is the first digit of the answer. This lets you greedily solve the problem, and array slicing makes the implementation efficient.",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

use nom::{
    IResult, Parser,
    character::complete::{multispace1, one_of},
    multi::{many1, separated_list1},
};

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let bank = many1(one_of("123456789").map(|d| d.to_digit(10).unwrap() as u8));
    separated_list1(multispace1, bank).parse(input.trim())
}

/// Largest number is always involved in the solution
pub fn solve_a(input: &str) -> u64 {
    let (rest, banks) = parse(input).expect("Failed to parse input");
    assert!(rest.is_empty(), "Unparsed input remaining");

    banks.iter().map(|bank| find_max_joltage(bank, 2)).sum()
}

pub fn solve_b(input: &str) -> u64 {
    let (rest, banks) = parse(input).expect("Failed to parse input");
    assert!(rest.is_empty(), "Unparsed input remaining");

    banks.iter().map(|bank| find_max_joltage(bank, 12)).sum()
}

fn find_max_joltage(bank: &[u8], limit: u32) -> u64 {
    assert!(
        bank.len() >= limit as usize,
        "Bank must be at least as long as limit"
    );

    let mut total: u64 = 0;
    let mut last_index = 0;

    for n in 0..limit {
        let slice = &bank[last_index..=(bank.len() - (limit - n) as usize)];
        let (max, max_index) = find_max(slice);
        last_index += max_index + 1;
        // println!("{slice:?} -> max {max} at index {last_index}");
        total += 10u64.pow(limit - 1 - n) * (u64::from(max));
    }

    total
}

fn find_max(input: &[u8]) -> (u8, usize) {
    let (mut max_index, mut max) = (0, 0);
    for (index, &value) in input.iter().enumerate() {
        if value > max {
            max = value;
            max_index = index;
        }
    }
    (max, max_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse directions");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 357);
    }

    #[test]
    fn test_a_all_same() {
        assert_eq!(solve_a("222222222222222"), 22);
    }

    #[test]
    fn test_a_increasing() {
        assert_eq!(solve_a("1234512345"), 55);
    }

    #[test]
    fn test_a_other_edges() {
        assert_eq!(solve_a("9123456789"), 99);
        assert_eq!(solve_a("9876543211"), 98);
        assert_eq!(solve_a("999999"), 99);
    }

    #[test]
    fn test_a_end() {
        assert_eq!(solve_a("1111111119"), 19);
        assert_eq!(solve_a("1111111199"), 99);
        assert_eq!(solve_a("1111611189"), 89);
        assert_eq!(solve_a("1111611181"), 81);
        assert_eq!(solve_a("1111611162"), 66);
        assert_eq!(solve_a("1111616162"), 66);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_b_lines() {
        assert_eq!(solve_b("987654321111111"), 987654321111);
        assert_eq!(solve_b("811111111111119"), 811111111119);
        assert_eq!(solve_b("234234234234278"), 434234234278);
        assert_eq!(solve_b("818181911112111"), 888911112111);
    }
}
