crate::solution!(
    5,
    "Printing Department",
    r"For Part 2 we insert the ranges into a new list while merging overlaps, so we don't double count anything",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

use std::ops::Range;

use nom::{
    IResult, Parser, character::complete::multispace1, multi::separated_list1,
    sequence::separated_pair,
};

pub fn parse(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<u64>)> {
    let ranges = separated_list1(
        multispace1,
        nom::sequence::separated_pair(
            nom::character::complete::u64,
            nom::bytes::complete::tag("-"),
            nom::character::complete::u64,
        )
        .map(|(start, end)| start..end + 1),
    );
    let ids = separated_list1(multispace1, nom::character::complete::u64);

    let (rest, (ranges, ids)) = separated_pair(ranges, multispace1, ids).parse(input.trim())?;

    debug_assert!(rest.is_empty(), "Unparsed input remaining");

    Ok((rest, (ranges, ids)))
}

pub fn solve_a(input: &str) -> u64 {
    let (_, (ranges, ids)) = parse(input).expect("Failed to parse input");

    let count = ids
        .iter()
        .filter(|id| ranges.iter().any(move |r| r.contains(id)))
        .count();

    count as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (_, (mut ranges, _)) = parse(input).expect("Failed to parse input");

    let mut merged: Vec<Range<u64>> = Vec::new();
    ranges.sort_by_key(|r| r.start);

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.start <= last.end {
                last.end = last.end.max(r.end);
            } else {
                merged.push(r);
            }
        } else {
            merged.push(r);
        }
    }

    let count = merged.iter().map(|r| r.end - r.start).sum();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, (ranges, ids)) = parse(EXAMPLE).expect("Failed to parse directions");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(ranges.len(), 4);
        assert_eq!(ids.len(), 6);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 3);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 14);
    }

    #[test]
    fn test_range_bad_overlap() {
        let inp = "3-5
4-6
3-8
3-10

1";
        assert_eq!(solve_b(inp), 8);
    }
}
