crate::solution!(
    9,
    "Movie Theater",
    r#"For Part 2 I'm using the <a href="https://en.wikipedia.org/wiki/Point_in_polygon">point in polygon</a> algorithm to check if the rectangle is valid."#,
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

    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles.contains(&(x, y)) {
                print!("#");
            } else if point_in_polygon((x.into(), y.into()), &tiles) {
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
    // print_areas(&areas, &tiles);

    // Convert to i128
    let tiles: Vec<(i128, i128)> = tiles.iter().map(|&(x, y)| (x as i128, y as i128)).collect();

    // For each area, check if all sided are green
    for area in areas {
        let (x, y) = (tiles[area.x_index], tiles[area.y_index]);

        // Check that all are inside the polygon
        let top = if x.0 < y.0 { x.0..=y.0 } else { y.0..=x.0 }.map(|xx| (xx, x.1));
        let bottom = if x.0 < y.0 { x.0..=y.0 } else { y.0..=x.0 }.map(|xx| (xx, y.1));
        let left = if x.1 < y.1 { x.1..=y.1 } else { y.1..=x.1 }.map(|yy| (x.0, yy));
        let right = if x.1 < y.1 { x.1..=y.1 } else { y.1..=x.1 }.map(|yy| (y.0, yy));
        let sides = top
            .chain(bottom)
            .chain(left)
            .chain(right)
            .collect::<Vec<(i128, i128)>>();

        if sides
            .into_par_iter()
            .all(|point| point_in_polygon(point, &tiles))
        {
            return area.area;
        }
    }

    panic!("No valid area found");
}

/// Only works for axis aligned polygons
fn point_in_polygon(point: (i128, i128), poly: &[(i128, i128)]) -> bool {
    let (px, py) = point;
    let mut inside = false;

    let n = poly.len();
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];

        // Check if point is on edge
        if xi == xj {
            // vertical edge
            if px == xi && py >= yi.min(yj) && py <= yi.max(yj) {
                return true;
            }
        } else {
            // horizontal edge
            if py == yi && px >= xi.min(xj) && px <= xi.max(xj) {
                return true;
            }
        }

        // ray casting
        if xi == xj {
            if (yi > py) != (yj > py) && px < xi {
                inside = !inside;
            }
        }

        j = i;
    }

    inside
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
