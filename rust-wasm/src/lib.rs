pub mod day_01;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! from Rust+WASM ???sdfsdf?", name)
}

pub static DAYS: &[&Day] = &[&day_01::SOLUTION];

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Day {
    pub day: u32,
    #[wasm_bindgen(skip)]
    pub description: fn() -> String,
    #[wasm_bindgen(skip)]
    pub part1: Option<fn(&str) -> String>,
    #[wasm_bindgen(skip)]
    pub part2: Option<fn(&str) -> String>,
}

#[wasm_bindgen]
impl Day {
    pub fn part1(&self, input: &str) -> String {
        match self.part1 {
            Some(f) => f(input),
            None => "Part 1 not implemented".to_string(),
        }
    }

    pub fn part2(&self, input: &str) -> String {
        match self.part2 {
            Some(f) => f(input),
            None => "Part 2 not implemented".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn desc(&self) -> String {
        (self.description)()
    }

    #[wasm_bindgen(getter)]
    pub fn number(&self) -> u32 {
        self.day
    }
}

#[wasm_bindgen]
pub fn get_day(day: u32) -> Option<Day> {
    DAYS.iter().find(|d| d.day == day).map(|&&d| d)
}

#[wasm_bindgen]
pub fn get_days() -> Vec<Day> {
    DAYS.iter().map(|&&d| d).collect()
}

#[wasm_bindgen]
pub fn get_day_description(day: u32) -> Option<String> {
    get_day(day).map(|d| d.desc())
}

#[doc(hidden)]
pub fn __solution_to_string<T: std::fmt::Display>(value: T) -> String {
    // Converts any Display output into the string the WASM interface expects.
    value.to_string()
}

#[macro_export]
macro_rules! solution {
    ($day:expr,$description:expr, $part1:expr $(,)?) => {
        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            description: || -> String { $crate::__solution_to_string($description) },
            part1: Some(|input| -> String { $crate::__solution_to_string($part1(input)) }),
            part2: None,
        };
    };

    ($day:expr,$description:expr, $part1:expr, $part2:expr $(,)?) => {
        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            description: || -> String { $crate::__solution_to_string($description) },
            part1: Some(|input| -> String { $crate::__solution_to_string($part1(input)) }),
            part2: Some(|input| -> String { $crate::__solution_to_string($part2(input)) }),
        };
    };
}
