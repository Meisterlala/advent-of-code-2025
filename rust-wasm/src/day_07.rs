crate::solution!(
    7,
    "Laboratories",
    r"The key insight for Part 2 is that you need a seperate array to track how many new rays get added per splitter",
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

pub fn solve_a(input: &str) -> u64 {
    let mut lines = input
        .trim()
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    for x in 0..lines[0].len() {
        if lines[0][x] == b'S' {
            lines[1][x] = b'|';
        }
    }

    let mut splits = 0;
    for y in (2..lines.len() - 1).step_by(2) {
        for x in 0..lines[0].len() {
            // If not a spliiter
            if lines[y][x] != b'^' {
                // But there might be a continued ray
                if lines[y - 1][x] == b'|' {
                    // Continue pipe down
                    lines[y][x] = b'|';
                    lines[y + 1][x] = b'|';
                }
                continue;
            };
            // Check above
            if lines[y - 1][x] != b'|' {
                continue;
            }
            // Add new pipe left and right
            splits += 1;
            lines[y][x - 1] = b'|';
            lines[y + 1][x - 1] = b'|';
            lines[y][x + 1] = b'|';
            lines[y + 1][x + 1] = b'|';
        }
    }
    splits
}

pub fn solve_b(input: &str) -> u64 {
    let lines = input
        .trim()
        .split('\n')
        .map(|inp| inp.as_bytes())
        .filter(|&line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    let mut paths = vec![0u64; lines[0].len()];
    paths[lines[0].iter().position(|&b| b == b'S').unwrap()] = 1;

    for row in lines[1..].iter() {
        for i in 0..row.len() {
            if row[i] == b'^' {
                let count = paths[i];
                paths[i] = 0;
                paths[i - 1] += count;
                paths[i + 1] += count;
            }
        }
    }
    paths.into_iter().sum()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        print_row(row);
    }
}

#[allow(dead_code)]
fn print_row(row: &[u8]) {
    println!("{}", String::from_utf8_lossy(row));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 21);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 40);
    }
}
