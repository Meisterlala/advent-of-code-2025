use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

use advent_of_code_2025::DAYS;
use advent_of_code_2025::download_input;

pub fn seperate(c: &mut Criterion) {
    for d in DAYS {
        let input = get_input(d.day);
        let mut group = c.benchmark_group(format!("day_{:02}", d.day));
        if let Some(p1) = d.part1 {
            group.bench_function("Part 1", |b| b.iter(|| p1(&input)));
        }
        if let Some(p2) = d.part2 {
            group.bench_function("Part 2", |b| b.iter(|| p2(&input)));
        }
        group.finish();
    }
}

pub fn combined(c: &mut Criterion) {
    for d in DAYS {
        let input = get_input(d.day);
        c.bench_function(&format!("day_{:02}", d.day), |b| {
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

fn get_input(day: u32) -> String {
    if !download_input::check_if_present(day) {
        println!("Day {:2} | Downloading Input ...", day);
        if let Err(e) = download_input::download_input(day) {
            let err = format!("Day {:2} | Failed to download input", day);
            println!("{err}: {e}");
            return String::new();
        }
    }

    match download_input::read_input(day) {
        Ok(input) => input,
        Err(e) => {
            println!("Day {:2} | Failed to read input: {}", day, e);
            String::new()
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = combined
}

criterion_main!(benches);
