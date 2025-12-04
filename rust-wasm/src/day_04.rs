crate::solution!(
    4,
    "Printing Department",
    r"Simple Cellular Automaton Simulation using matrix convolution with a $3 \times 3$ kernel. This could be optimized further by keeping track of recently changed cell neighbours in a queue.",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
use nom::{
    IResult, Parser,
    character::complete::{multispace1, one_of},
    multi::{many1, separated_list1},
};

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    let row = many1(one_of(".@").map(|d| match d {
        '@' => true,
        '.' => false,
        _ => unreachable!(),
    }));
    let (rest, grid) = separated_list1(multispace1, row).parse(input.trim())?;

    debug_assert!(rest.is_empty(), "Unparsed input remaining");

    // Test that all rows are same length
    debug_assert!(
        grid.iter().all(|row| row.len() == grid[0].len()),
        "Non-rectangular grid"
    );
    Ok((rest, grid))
}

pub fn solve_a(input: &str) -> u64 {
    let (_, grid) = parse(input).expect("Failed to parse input");
    let rows = grid.len();
    let cols = grid[0].len();

    // Convert vec to ndarray
    let matrix = Array2::from_shape_fn((rows, cols), |(r, c)| if grid[r][c] { 1u8 } else { 0u8 });

    // Convolution kernel
    let kernel = arr2(&[[1u8, 1u8, 1u8], [1u8, 0u8, 1u8], [1u8, 1u8, 1u8]]);
    let mut neighbours = matrix
        .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
        .expect("Should never fail, unless the data is messed up");

    // Filter
    neighbours.mapv_inplace(|x| if x < 4 { 1u8 } else { 0u8 });
    let accessible = matrix * neighbours;

    accessible.sum() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (_, grid) = parse(input).expect("Failed to parse input");
    let rows = grid.len();
    let cols = grid[0].len();

    // Convert vec to ndarray
    let mut matrix =
        Array2::from_shape_fn((rows, cols), |(r, c)| if grid[r][c] { 1u8 } else { 0u8 });
    // Convolution kernel
    let kernel = arr2(&[[1u8, 1u8, 1u8], [1u8, 0u8, 1u8], [1u8, 1u8, 1u8]]);

    let mut changed = true;
    let inital_count = matrix.sum() as u64;
    let mut count = u64::MAX;

    while changed {
        // Apply Kernel
        let mut neighbours = matrix
            .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
            .expect("Should never fail, unless the data is messed up");

        // Filter
        neighbours.mapv_inplace(|x| if x < 4 { 0u8 } else { 1u8 });
        matrix = matrix * neighbours;

        // Check if anything changed
        let current_count = matrix.sum() as u64;
        changed = current_count != count;
        count = current_count;
    }

    inital_count - count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse directions");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert!(parsed.len() == 10);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 13);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 43);
    }
}
