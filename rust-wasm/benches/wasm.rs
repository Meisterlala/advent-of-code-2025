use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::{Criterion, wasm_bindgen_bench};

use advent_of_code_2025::DAYS;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_bench]
pub fn combined(c: &mut Criterion) {
    for d in DAYS {
        let input = get_input(d.day);
        c.bench_function(&format!("day{:02}_combined", d.day), |b| {
            b.iter(|| {
                if let Some(p1) = d.part1 {
                    p1(&input);
                }
                if let Some(p2) = d.part2 {
                    p2(&input);
                }
            })
        });
    }
}

// Here we need to include the inputs directly since filesystem access is not available in wasm
fn get_input(day: u32) -> String {
    match day {
        1 => include_str!("../inputs/day01"),
        2 => include_str!("../inputs/day02"),
        3 => include_str!("../inputs/day03"),
        4 => include_str!("../inputs/day04"),
        5 => include_str!("../inputs/day05"),
        6 => include_str!("../inputs/day06"),
        7 => include_str!("../inputs/day07"),
        8 => include_str!("../inputs/day08"),
        9 => include_str!("../inputs/day09"),
        10 => include_str!("../inputs/day10"),
        11 => include_str!("../inputs/day11"),
        // 12 => include_str!("../inputs/day12"),
        _ => {
            panic!("Input for day {} not found", day);
        }
    }
    .to_string()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    combined(&mut Criterion::default());
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
