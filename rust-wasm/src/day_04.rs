crate::solution!(
    4,
    "Printing Department",
    r"This is just a 2D array iteration where i apply a kernel. The current solution is brute force. But it could be optimized by precomputing the number of neighbours. And then using a queue to only recheck rolls that are adjacent to changed rolls. Or with a Matrix convolution approach.",
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

    let mut accessible_rolls = 0u64;

    // Just 2d grid iteration
    for row_index in 0..grid.len() {
        for col_index in 0..grid[0].len() {
            // Not a roll
            if !grid[row_index][col_index] {
                continue;
            }

            let neighbour_row_start = row_index.saturating_sub(1);
            let neighbour_col_start = col_index.saturating_sub(1);
            let neighbour_row_end = (row_index + 1).min(grid.len() - 1);
            let neighbour_col_end = (col_index + 1).min(grid[col_index].len() - 1);

            let mut neightbour_count = 0;
            'neighbours: for n_row in neighbour_row_start..=neighbour_row_end {
                for n_col in neighbour_col_start..=neighbour_col_end {
                    if n_row == row_index && n_col == col_index {
                        continue;
                    }
                    if grid[n_row][n_col] {
                        neightbour_count += 1;
                    }
                    // Early exit if already too many neighbours
                    if neightbour_count > 3 {
                        break 'neighbours;
                    }
                }
            }
            if neightbour_count < 4 {
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

pub fn solve_b(input: &str) -> u64 {
    let (_, mut grid) = parse(input).expect("Failed to parse input");

    let mut accessible_rolls = 0u64;
    let mut changed = true;

    while changed {
        changed = false;
        for row_index in 0..grid.len() {
            for col_index in 0..grid[0].len() {
                // Not a roll
                if !grid[row_index][col_index] {
                    continue;
                }

                let neighbour_row_start = row_index.saturating_sub(1);
                let neighbour_col_start = col_index.saturating_sub(1);
                let neighbour_row_end = (row_index + 1).min(grid.len() - 1);
                let neighbour_col_end = (col_index + 1).min(grid[col_index].len() - 1);

                let mut neightbour_count = 0;
                'neighbours: for n_row in neighbour_row_start..=neighbour_row_end {
                    for n_col in neighbour_col_start..=neighbour_col_end {
                        if n_row == row_index && n_col == col_index {
                            continue;
                        }
                        if grid[n_row][n_col] {
                            neightbour_count += 1;
                        }
                        // Early exit if already too many neighbours
                        if neightbour_count > 3 {
                            break 'neighbours;
                        }
                    }
                }
                if neightbour_count < 4 {
                    changed = true;
                    grid[row_index][col_index] = false;
                    accessible_rolls += 1;
                }
            }
        }
    }

    accessible_rolls
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
