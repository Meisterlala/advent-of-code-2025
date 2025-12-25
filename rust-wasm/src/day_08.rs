crate::solution!(
    8,
    "Playground",
    r"Right now its a pretty unoptimized solution, taking about 20ms for each part. This needs a rework with graph theory at some point.",
    &EXAMPLE,
    solve_a,
    solve_b
);
// TODO: Optimize with graph theory, this needs a disjoint set union and kuruskal's algorithm

static EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:5}X {:5}Y {:5}Z]", self.x, self.y, self.z)
    }
}

use nom::{
    IResult, Parser,
    character::complete::{self, char, digit1},
    multi::separated_list1,
};

pub fn parse(input: &str) -> IResult<&str, Vec<Position>> {
    let position = (digit1, char(','), digit1, char(','), digit1).map(
        |(x_str, _, y_str, _, z_str): (&str, _, &str, _, &str)| Position {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
            z: z_str.parse().unwrap(),
        },
    );
    let (rest, positions) = separated_list1(complete::line_ending, position).parse(input.trim())?;
    debug_assert!(rest.is_empty(), "Unparsed input remaining");

    Ok((rest, positions))
}

pub fn solve_a(input: &str) -> u64 {
    solve_a_with_iterations(input, 1_000)
}

pub fn solve_a_with_iterations(input: &str, mut itertations: usize) -> u64 {
    let (_, positions) = parse(input).expect("Failed to parse input");

    // "Hack" for example input, so it doesnt break on the website. Only needed because of the arbitrary iteration number for the example
    if positions.len() == 20 {
        itertations = 10;
    }

    // Initialize each position as its own circet
    let mut circets: Vec<Vec<Position>> = Vec::with_capacity(positions.len());
    for pos in &positions {
        circets.push(vec![*pos]);
    }

    // Sort them by distance
    let mut distances: Vec<(Position, Position, f64)> =
        Vec::with_capacity(positions.len() * (positions.len() - 1) / 2);
    for pos_a in 0..positions.len() {
        for pos_b in pos_a + 1..positions.len() {
            let dist = distance(&positions[pos_a], &positions[pos_b]);
            distances.push((positions[pos_a], positions[pos_b], dist));
        }
    }

    // Sort distances descending
    distances.sort_unstable_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for _ in 0..itertations {
        // Minumum distance is the first in the sorted list
        let min_pos = distances.pop().expect("No more distances to process");

        // Merge circets based on distance criteria
        let mut did_merge = false;
        'circet: for i in 0..circets.len() {
            if circets[i].contains(&min_pos.0) {
                for j in 0..circets.len() {
                    if circets[j].contains(&min_pos.1) {
                        // Merge circets[i] and circets[j]
                        if i == j {
                            did_merge = true;
                            break 'circet;
                        }
                        let mut to_merge = circets.swap_remove(j);
                        // Append to circets[i], check if its the swapped one
                        if i == circets.len() {
                            circets[j].append(&mut to_merge);
                        } else {
                            circets[i].append(&mut to_merge);
                        }
                        did_merge = true;
                        break 'circet;
                    }
                }
            }
        }
        debug_assert!(did_merge, "Failed to merge circets");
    }

    // Find 3 with max length
    circets.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    debug_assert!(circets.len() >= 3, "Less than 3 circets found");
    circets[0].len() as u64 * circets[1].len() as u64 * circets[2].len() as u64
}

fn distance(a: &Position, b: &Position) -> f64 {
    let x = (a.x as f64 - b.x as f64).powi(2);
    let y = (a.y as f64 - b.y as f64).powi(2);
    let z = (a.z as f64 - b.z as f64).powi(2);

    (x + y + z).sqrt()
}

pub fn solve_b(input: &str) -> u64 {
    let (_, positions) = parse(input).expect("Failed to parse input");

    // Initialize each position as its own circet
    let mut circets: Vec<Vec<Position>> = Vec::with_capacity(positions.len());
    for pos in &positions {
        circets.push(vec![*pos]);
    }

    // Sort them by distance
    let mut distances: Vec<(Position, Position, f64)> =
        Vec::with_capacity(positions.len() * (positions.len() - 1) / 2);
    for pos_a in 0..positions.len() {
        for pos_b in pos_a + 1..positions.len() {
            let dist = distance(&positions[pos_a], &positions[pos_b]);
            distances.push((positions[pos_a], positions[pos_b], dist));
        }
    }

    // Sort distances descending
    distances.sort_unstable_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let last_boxes;

    'outer: loop {
        // Minumum distance is the first in the sorted list
        let min_pos = distances.pop().unwrap();

        // Merge circets based on distance criteria
        let mut did_merge = false;
        'circet: for i in 0..circets.len() {
            if circets[i].contains(&min_pos.0) {
                for j in 0..circets.len() {
                    if circets[j].contains(&min_pos.1) {
                        // Merge circets[i] and circets[j]
                        if i == j {
                            did_merge = true;
                            break 'circet;
                        }
                        let mut to_merge = circets.swap_remove(j);
                        // Append to circets[i], check if its the swapped one
                        if i == circets.len() {
                            circets[j].append(&mut to_merge);
                        } else {
                            circets[i].append(&mut to_merge);
                        }

                        // End condition
                        if circets.len() <= 1 {
                            last_boxes = Some((min_pos.0, min_pos.1));
                            break 'outer;
                        }
                        did_merge = true;
                        break 'circet;
                    }
                }
            }
        }
        debug_assert!(did_merge, "Failed to merge circets");
    }
    assert!(last_boxes.is_some(), "No last boxes found");
    let (box_a, box_b) = last_boxes.unwrap();
    box_a.x as u64 * box_b.x as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, positions) = parse(EXAMPLE).expect("Failed to parse");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(positions.len(), 20);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a_with_iterations(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 25272);
    }
}
