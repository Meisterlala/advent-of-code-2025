crate::solution!(10, "Factory", r#""#, &EXAMPLE, solve_a, solve_b);

static EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

use std::fmt;

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, char, multispace1, space1},
    multi::{many1, separated_list1},
    sequence::delimited,
};

use rayon::prelude::*;

pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage: Vec<usize>,
}

pub fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    let lights = delimited(
        char('['),
        many1(alt((char('#').map(|_| true), char('.').map(|_| false)))),
        char(']'),
    );
    let button = delimited(
        char('('),
        separated_list1(char(','), complete::u64.map(|n| n as usize)),
        char(')'),
    );
    let buttons = separated_list1(space1, button);
    let joltage = delimited(
        char('{'),
        separated_list1(char(','), complete::u64.map(|n| n as usize)),
        char('}'),
    );
    let machine =
        (lights, space1, buttons, space1, joltage).map(|(lights, _, buttons, _, joltage)| {
            Machine {
                lights,
                buttons,
                joltage,
            }
        });
    separated_list1(multispace1, machine).parse(input.trim())
}

fn bfs(machines: Machine, start_state: &[bool], end_state: &[bool]) -> Option<usize> {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start_state.to_vec(), 0));

    while let Some((current_state, depth)) = queue.pop_front() {
        if current_state == end_state {
            return Some(depth);
        }

        // Generate next states
        for button in &machines.buttons {
            let mut next_state = current_state.to_vec();
            for &index in button {
                next_state[index] = !next_state[index];
            }

            // If not visited, add to queue
            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }
    None
}

pub fn solve_a(input: &str) -> u64 {
    let (_, machines) = parse(input).expect("Failed to parse");
    let total_steps: usize = machines
        .into_par_iter()
        .map(|machine| {
            let start_state = vec![false; machine.lights.len()];
            let end_state = machine.lights.clone();
            bfs(machine, &start_state, &end_state).expect("No solution found")
        })
        .sum();
    total_steps as u64
}
pub fn solve_b(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(parsed.len(), 3, "Expected 3 machines");
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 7);
    }

    #[test]
    fn test_solve_a_parts() {
        let lines = EXAMPLE.trim().lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 3, "Expected 3 lines in example");
        let solutions = vec![2, 3, 2];

        for (line, sol) in lines.iter().zip(solutions.iter()) {
            let (remaining, parsed) = parse(line).expect("Failed to parse line");
            assert!(remaining.is_empty(), "Unparsed input remaining in line");
            assert_eq!(parsed.len(), 1, "Expected 1 machine per line");

            assert_eq!(solve_a(line), *sol, "Unexpected solution for line");
        }
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 24);
    }
}
