crate::solution!(
    6,
    "Trash Compactor",
    r"Parsing made part2 difficult",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

use ndarray::Array2;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::separated_pair,
};

pub enum Operation {
    Add,
    Multiply,
}

pub fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    let line = separated_list1(space1, nom::character::complete::u64);
    let lines = separated_list1(multispace1, line);
    let operations = separated_list1(
        space1,
        alt((tag("*"), tag("+"))).map(|s: &str| match s {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unreachable!("Invalid operation"),
        }),
    );

    let (rest, (numbers, operations)) =
        separated_pair(lines, multispace0, operations).parse(input.trim())?;
    debug_assert!(rest.is_empty(), "Unparsed input remaining");

    Ok((rest, (numbers, operations)))
}

pub fn solve_a(input: &str) -> u64 {
    let (_, (numbers, operations)) = parse(input).expect("Failed to parse input");

    // Transpose the numbers array, so we can access it by columns
    let array = Array2::from_shape_vec(
        (numbers.len(), numbers[0].len()),
        numbers.iter().flatten().cloned().collect(),
    )
    .expect("Failed to create array");

    // Iterate over each column and corresponding operation
    let colums = array.columns();
    colums
        .into_iter()
        .zip(operations.into_iter())
        .map(|(colum, operation)| match operation {
            Operation::Add => colum.sum(),
            Operation::Multiply => colum.product(),
        })
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let data = input.trim().split(|b| b == '\n').collect::<Vec<_>>();
    let (mut nums, mut current, mut total, mut op) = (Vec::with_capacity(4), 0, 0, 0);

    for i in 0..data[0].len() {
        nums.clear();
        nums.extend(
            data[..data.len() - 1]
                .iter()
                .map(|line| line.as_bytes()[i])
                .filter(|&b| b != b' '),
        );
        if nums.is_empty() {
            total += current;
            continue;
        }

        let value = nums
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| (b - b'0') as usize * 10_usize.pow(i as u32))
            .sum::<usize>();

        let new = data.last().unwrap().as_bytes().get(i).unwrap_or(&b' ');
        if *new != b' ' {
            op = *new;
            current = (op == b'*') as _;
        }

        if op == b'+' {
            current += value;
        } else {
            current *= value;
        }
    }
    total += current;

    return total as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, (ranges, ids)) = parse(EXAMPLE).expect("Failed to parse directions");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], vec![123, 328, 51, 64]);
        assert_eq!(ids.len(), 4);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 4277556);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 3263827);
    }
}
