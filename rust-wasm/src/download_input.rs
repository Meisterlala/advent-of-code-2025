use std::{fs::OpenOptions, path::Path};

#[allow(dead_code)]
pub fn check_if_present(day: u32) -> bool {
    Path::new(&format!("./inputs/day{:02}", day)).exists()
}

#[allow(dead_code)]
pub fn read_input(day: u32) -> Result<String, String> {
    std::fs::read_to_string(format!("./inputs/day{:02}", day)).map_err(|e| e.to_string())
}
