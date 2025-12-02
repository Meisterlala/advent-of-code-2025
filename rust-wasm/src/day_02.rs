crate::solution!(
    2,
    "Gift Shop",
    "I iterate through the list of IDs and filter them. This is pretty straightforward but not very efficient, because it needs a lot of string comparisons.",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

use std::ops::Range;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{self, multispace1},
    multi::{many1, separated_list1},
};

pub fn solve_a(input: &str) -> u64 {
    let (remaining, ranges) = parse(input).expect("Failed to parse ranges");
    assert!(
        remaining.is_empty(),
        "Unparsed input remaining: {}",
        remaining
    );

    ranges
        .iter()
        .flat_map(|range| {
            range
                .clone()
                .filter(|&id| has_repeats_a(id))
                .collect::<Vec<u64>>()
        })
        .sum::<u64>()
}

pub fn solve_b(input: &str) -> u64 {
    let (remaining, ranges) = parse(input).expect("Failed to parse ranges");
    assert!(
        remaining.is_empty(),
        "Unparsed input remaining: {}",
        remaining
    );

    ranges
        .iter()
        .flat_map(|range| {
            range
                .clone()
                .filter(|&id| has_repeats_b(id))
                .collect::<Vec<u64>>()
        })
        .sum::<u64>()
}

fn has_repeats_a(id: u64) -> bool {
    let id_str = id.to_string();
    // Not in sample input
    // let id_str = id_str.strip_prefix("0").unwrap_or(&id_str);
    let len = id_str.len();

    // Cant have repeats if length is odd
    if !len.is_multiple_of(2) {
        return false;
    }

    let sub_str = &id_str[0..len / 2];
    if sub_str == &id_str[len / 2..] {
        return true;
    }
    false
}

fn has_repeats_b(id: u64) -> bool {
    let id_str = id.to_string();
    // Not in sample input
    // let id_str = id_str.strip_prefix("0").unwrap_or(&id_str);
    let len = id_str.len();

    for sub_len in 1..=(len / 2) {
        if len.is_multiple_of(sub_len) {
            let sub_str = &id_str[0..sub_len];
            if sub_str.repeat(len / sub_len) == id_str {
                return true;
            }
        }
    }
    false
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
