#[cfg(not(target_arch = "wasm32"))]
pub mod download_input;

use wasm_bindgen::prelude::*;

// Specify all days here
days!(
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
);

// WASM Interface. For some reason i cant use strings. So its all wrapped functions.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Day {
    pub day: u32,
    #[wasm_bindgen(skip)]
    pub title_fn: fn() -> String,
    #[wasm_bindgen(skip)]
    pub description: fn() -> String,
    #[wasm_bindgen(skip)]
    pub example_fn: fn() -> String,
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
    pub fn title(&self) -> String {
        (self.title_fn)()
    }

    #[wasm_bindgen(getter)]
    pub fn example(&self) -> String {
        (self.example_fn)()
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

#[doc(hidden)]
pub fn __to_string<T: std::fmt::Display>(value: T) -> String {
    value.to_string()
}

#[macro_export]
macro_rules! solution {
    ($day:expr, $title:expr, $description:expr, $example:expr, $part1:expr) => {
        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            title_fn: || -> String { $crate::__to_string($title) },
            description: || -> String { $crate::__to_string($description) },
            example_fn: || -> String { $crate::__to_string($example) },
            part1: Some(|input| -> String { $crate::__to_string($part1(input)) }),
            part2: None,
        };
    };

    ($day:expr, $title:expr, $description:expr, $example:expr, $part1:expr, $part2:expr) => {
        pub static SOLUTION: $crate::Day = $crate::Day {
            day: $day,
            title_fn: || -> String { $crate::__to_string($title) },
            description: || -> String { $crate::__to_string($description) },
            example_fn: || -> String { $crate::__to_string($example) },
            part1: Some(|input| -> String { $crate::__to_string($part1(input)) }),
            part2: Some(|input| -> String { $crate::__to_string($part2(input)) }),
        };
    };
}

#[macro_export]
macro_rules! days {
    ( $( $mod:ident ),* $(,)? ) => {
        $(
            pub mod $mod;
        )*
        pub static DAYS: &[&Day] = &[
            $(
                &$mod::SOLUTION,
            )*
        ];
    };
}
