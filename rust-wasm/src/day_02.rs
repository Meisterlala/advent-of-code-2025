crate::solution!(
    2,
    "Gift Shop",
    r"We can generate the invalid IDs instead of brute force checking each ID in the range, because most IDs are valid. 
    Assume we have a repeating pattern $p$ with $d$ digits that is repeated $k$ times, then the invalid IDs are of form:<br> 
    $$\text{Part1}= p * (10^d+1)$$
    $$\text{Part2} = p* \sum_{i=0}^{k-1}{10^{i*d}}=p* \frac{10^{k*d}-1}{10^k -1}$$
    We can transform the part 2 formular into a closed form to reduce iterations.
    ",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

use std::{collections::HashSet, ops::Range};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{self, multispace1},
    multi::{many1, separated_list1},
};

// Genereate all the valid ids and then check the ranges
pub fn solve_a(input: &str) -> u64 {
    let (remaining, ranges) = parse(input).expect("Failed to parse ranges");
    debug_assert!(
        remaining.is_empty(),
        "Unparsed input remaining: {}",
        remaining
    );

    ranges
        .iter()
        .flat_map(move |range| invalid_a(range))
        .sum::<u64>()
}

pub fn solve_b(input: &str) -> u64 {
    let (remaining, ranges) = parse(input).expect("Failed to parse ranges");
    debug_assert!(
        remaining.is_empty(),
        "Unparsed input remaining: {}",
        remaining
    );

    let max_digits = ranges
        .iter()
        .map(|range| range.end)
        .max()
        .unwrap_or(0)
        .to_string()
        .len();

    // Require a set to dont double count numbers, that might have the same pattern with different repitions
    // Like [11][11][11] and [1][1][1][1][1][1][1] or [111][111]
    let set = (2..=max_digits)
        .flat_map(|repitions| {
            ranges
                .iter()
                .flat_map(move |range| invalid_b(range, repitions))
        })
        .collect::<HashSet<u64>>();

    set.into_iter().sum::<u64>()
}

/// N = base * (10^d + 1)
fn invalid_a(range: &Range<u64>) -> impl Iterator<Item = u64> {
    // A 64Bit number can only have 20 digits, so we only need to check up to 10 digits repeated twice
    (1..=10).flat_map(move |num_digits| {
        // We calculate the multiplier for the current number of digits
        let multiplier = 10u64.pow(num_digits) + 1;

        // Calculate which bases produce numbers in [start, end]
        let min_base = range.start.div_ceil(multiplier);
        let max_base = range.end / multiplier;

        // Base must have exactly num_digits
        let base_start = 10u64.pow(num_digits - 1);
        let base_end = 10u64.pow(num_digits);

        (min_base.max(base_start)..=max_base.min(base_end - 1))
            .map(move |base| base * multiplier)
            .filter(move |&n| n >= range.start && n <= range.end)
    })
}

/// N = base * (10^d - 1) / (10^d - 1)
/// Referenced: https://github.com/G36maid/advent-of-code-2025/blob/main/src/bin/02.rs
fn invalid_b(range: &Range<u64>, repitions: usize) -> impl Iterator<Item = u64> {
    let start_digits = range.start.to_string().len();
    let end_digits = range.end.to_string().len();

    // Determine min and max pattern lengths needed for this range and k
    let min_pattern_len = start_digits.div_ceil(repitions);
    let max_pattern_len = end_digits / repitions;

    (min_pattern_len..=max_pattern_len).flat_map(move |pattern_len| {
        let base_start = 10u64.pow(pattern_len.saturating_sub(1) as u32);
        let base_end = 10u64.pow(pattern_len as u32);

        let d = pattern_len as u32;
        let power_d = 10u64.pow(d);

        // Calculate the geometric series sum multiplier
        let multiplier = if let Some(power_kd) = 10u64.checked_pow(repitions as u32 * d) {
            (power_kd - 1) / (power_d - 1)
        } else {
            return (0..0)
                .filter_map(|_| None::<u64>)
                .collect::<Vec<_>>()
                .into_iter();
        };

        (base_start..base_end)
            .filter_map(move |base| {
                base.checked_mul(multiplier)
                    .filter(|&n| n >= range.start && n <= range.end)
            })
            .collect::<Vec<_>>()
            .into_iter()
    })
}

pub fn parse(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    fn id_pair(input: &str) -> IResult<&str, Range<u64>> {
        let (input, start) = complete::u64(input)?;
        let (input, _) = complete::char('-')(input)?;
        let (input, end) = complete::u64(input)?;
        Ok((
            input,
            Range {
                start,
                end: end + 1,
            },
        ))
    }
    separated_list1(many1(alt((tag(","), multispace1))), id_pair).parse_complete(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse directions");
        assert_eq!(remaining, "");
        assert_eq!(
            parsed,
            vec![
                11..23,
                95..116,
                998..1013,
                1188511880..1188511891,
                222220..222225,
                1698522..1698529,
                446443..446450,
                38593856..38593863,
                565653..565660,
                824824821..824824828,
                2121212118..2121212125,
            ]
        );
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 4174379265);
    }
}
