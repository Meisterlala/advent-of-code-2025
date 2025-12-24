crate::solution!(
    9,
    "Movie Theater",
    r#"For Part 2 I'm checking if a line intersects the area and consider it invalid."#,
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

use std::fmt;

use nom::{
    IResult, Parser,
    character::complete::{self, char},
    multi::separated_list1,
};

use rayon::prelude::*;

pub fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let tile = ((complete::u64, char(','), complete::u64)).map(|(x, _, y): (u64, _, u64)| (x, y));
    let (rest, tiles) = separated_list1(complete::line_ending, tile).parse(input.trim())?;
    debug_assert!(rest.is_empty(), "Unparsed input remaining");
    Ok((rest, tiles))
}

pub fn solve_a(input: &str) -> u64 {
    let (_, tiles) = parse(input).expect("Failed to parse tiles");
    // print_tiles(&tiles);

    // index x, index y, area
    let mut max: Option<(usize, usize, u64)> = None;
    for x_i in 0..tiles.len() {
        for y_i in x_i + 1..tiles.len() {
            let (x, y) = (tiles[x_i], tiles[y_i]);
            let area = (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1);
            if let Some((_, _, max_area)) = max {
                if area > max_area {
                    max = Some((x_i, y_i, area));
                }
            } else {
                max = Some((x_i, y_i, area));
            }
        }
    }
    assert!(max.is_some(), "No maximum area found");
    let max = max.unwrap();
    // let (max_x, max_y) = (tiles[max.0], tiles[max.1]);
    max.2
}

#[derive(Debug, Clone, Copy)]
struct AreaPair {
    x_index: usize,
    y_index: usize,
    area: u64,
}

impl AreaPair {
    fn print<T>(&self, tiles: &[(T, T)])
    where
        T: fmt::Display,
    {
        let (x, y) = (&tiles[self.x_index], &tiles[self.y_index]);
        println!(
            "({:3},{:3}) | ({:3},{:3}) | {:5}",
            x.0, x.1, y.0, y.1, self.area
        );
    }
}

#[allow(dead_code)]
fn print_areas<T>(areas: &Vec<AreaPair>, tiles: &[(T, T)])
where
    T: fmt::Display,
{
    if areas.len() > 0 {
        println!("--  X  -- | --  Y  -- |  Area");
    }
    for area in areas {
        area.print(tiles);
    }
}

#[allow(dead_code)]
fn print_tiles(tiles: &Vec<(u64, u64)>) {
    let max_x = tiles.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = tiles.iter().map(|&(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_tiles_green<T>(tiles: &Vec<(T, T)>)
where
    T: Into<i128> + Copy,
{
    let tiles: Vec<(i128, i128)> = tiles.iter().map(|&(x, y)| (x.into(), y.into())).collect();
    let max_x = tiles.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = tiles.iter().map(|&(_, y)| y).max().unwrap();

    // Create lines from tiles
    let mut lines: Vec<((i128, i128), (i128, i128))> = Vec::with_capacity(tiles.len());
    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        let start = (tiles[i].0 as i128, tiles[i].1 as i128);
        let end = (tiles[next].0 as i128, tiles[next].1 as i128);
        lines.push((start, end));
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles.contains(&(x, y)) {
                print!("#");
            } else if line_intersects_rect(&((x, y), (x, y)), &lines) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solve_b(input: &str) -> u64 {
    let (_, tiles) = parse(input).expect("Failed to parse tiles");
    // print_tiles_green(&tiles);

    // Calculate all areas
    let mut areas: Vec<AreaPair> = Vec::with_capacity(tiles.len() * (tiles.len() - 1) / 2);
    for x_i in 0..tiles.len() {
        for y_i in x_i + 1..tiles.len() {
            let (x, y) = (tiles[x_i], tiles[y_i]);
            let area = (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1);
            areas.push(AreaPair {
                x_index: x_i,
                y_index: y_i,
                area,
            });
        }
    }
    // Sort areas descending
    areas.sort_unstable_by(|a, b| b.area.cmp(&a.area));

    // Create lines from tiles
    let mut lines: Vec<((i128, i128), (i128, i128))> = Vec::with_capacity(tiles.len());
    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        let start = (tiles[i].0 as i128, tiles[i].1 as i128);
        let end = (tiles[next].0 as i128, tiles[next].1 as i128);
        lines.push((start, end));
    }

    // Convert to i128
    let tiles: Vec<(i128, i128)> = tiles.iter().map(|&(x, y)| (x as i128, y as i128)).collect();

    // For each area, check if all sided are green
    let max = areas
        .into_par_iter()
        .find_first(|area| {
            let (x, y) = (tiles[area.x_index], tiles[area.y_index]);

            let (min_x, max_x) = if x.0 < y.0 { (x.0, y.0) } else { (y.0, x.0) };
            let (min_y, max_y) = if x.1 < y.1 { (x.1, y.1) } else { (y.1, x.1) };

            !line_intersects_rect(&((min_x + 1, min_y + 1), (max_x - 1, max_y - 1)), &lines)
        })
        .unwrap_or_else(|| {
            panic!("No valid area found");
        });
    return max.area;
}

fn line_intersects_rect(
    rect: &((i128, i128), (i128, i128)),
    lines: &[((i128, i128), (i128, i128))],
) -> bool {
    let ((rx1, ry1), (rx2, ry2)) = rect;

    let rect_lines = vec![
        ((*rx1, *ry1), (*rx2, *ry1)), // top
        ((*rx2, *ry1), (*rx2, *ry2)), // right
        ((*rx2, *ry2), (*rx1, *ry2)), // bottom
        ((*rx1, *ry2), (*rx1, *ry1)), // left
    ];

    for line in lines {
        for rect_line in &rect_lines {
            if lines_intersect(line, rect_line) {
                return true;
            }
        }
    }

    false
}

/// Check if two lines intersect, only for straight lines
fn lines_intersect(
    line1: &((i128, i128), (i128, i128)),
    line2: &((i128, i128), (i128, i128)),
) -> bool {
    let ((x1, y1), (x2, y2)) = line1;
    let ((x3, y3), (x4, y4)) = line2;

    // Check if lines are vertical or horizontal
    let line1_vertical = x1 == x2;
    let line1_horizontal = y1 == y2;
    let line2_vertical = x3 == x4;
    let line2_horizontal = y3 == y4;

    if line1_vertical && line2_horizontal {
        // line1 is vertical, line2 is horizontal
        return x1 >= x3.min(x4) && x1 <= x3.max(x4) && y3 >= y1.min(y2) && y3 <= y1.max(y2);
    } else if line1_horizontal && line2_vertical {
        // line1 is horizontal, line2 is vertical
        return x3 >= x1.min(x2) && x3 <= x1.max(x2) && y1 >= y3.min(y4) && y1 <= y3.max(y4);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(
            parsed,
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 50);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 24);
    }
}
