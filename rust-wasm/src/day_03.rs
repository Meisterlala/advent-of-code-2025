crate::solution!(3, "Lobby", "", &EXAMPLE, solve_a, solve_b);

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

    // Go through and find the max values
    let mut total: u64 = 0;
    for bank in &banks {
        let max = bank
            .iter()
            .take(bank.len() - 1)
            .max()
            .expect("There has to be at least one number");
        let max_index = bank
            .iter()
            .position(|d| d == max)
            .expect("Max has to be in the bank, we found it earlier");
        let second_max = bank
            .iter()
            .skip(max_index + 1)
            .max()
            .expect("There has to be a second number, we skipped the last");
        let bank_sum: u64 = (max * 10 + second_max) as u64;

        assert!(bank_sum < 100, "Bank sum must be two digits");
        total += bank_sum;
    }
    total
}

pub fn solve_b(input: &str) -> u64 {
    let (rest, banks) = parse(input).expect("Failed to parse input");
    assert!(rest.is_empty(), "Unparsed input remaining");

    // Go through and find the max values
    let mut total: u64 = 0;
    for bank in &banks {
        assert!(
            bank.len() >= 12,
            "Bank must have at least 12 digits for part B"
        );

        let mut result = [0u8; 12];
        let mut last_index = 0;

        for n in 0..12 {
            let max = bank
                .iter()
                .skip(last_index)
                .take(bank.len() - (12 - n) - last_index + 1)
                .map(|d| *d)
                .max()
                .expect("There has to be at least one number");
            let max_index = bank
                .iter()
                .skip(last_index)
                .position(|&d| d == max)
                .expect("Max has to be in the bank, we found it earlier");
            last_index = max_index + 1 + last_index;
            result[n] = max;
        }

        let bank_sum: u64 = result
            .iter()
            .enumerate()
            .map(|(i, d)| (*d as u64) * 10u64.pow(11 - i as u32))
            .sum();

        total += bank_sum;
    }
    total
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
