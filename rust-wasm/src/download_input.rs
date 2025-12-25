use std::path::Path;

#[allow(dead_code)]
pub fn check_if_present(day: u32) -> bool {
    Path::new(&format!("./inputs/day{:02}", day)).exists()
}

#[allow(dead_code)]
pub fn read_input(day: u32) -> Result<String, String> {
    std::fs::read_to_string(format!("./inputs/day{:02}", day)).map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn download_input(day: u32) -> Result<(), String> {
    use reqwest::blocking::Client;
    use std::fs::OpenOptions;

    // Load session cookie from .env file
    dotenv::dotenv().ok();
    let cookie = std::env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable not set and not in '.env' file. You get this by signing in on https://adventofcode.com")?;

    // Download input
    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    let mut response = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .map_err(|e| format!("Failed to download input: {}", e))?;

    // Check response status
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download input: {}",
            response.status().as_str()
        ));
    }

    // Create inputs directory if it doesn't exist
    std::fs::create_dir_all("./inputs")
        .map_err(|e| format!("Failed to create inputs directory: {}", e))?;

    // Write input to file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("./inputs/day{:02}", day))
        .map_err(|e| {
            format!(
                "Failed to create input file './inputs/day{:02}': {}",
                day, e
            )
        })?;
    std::io::copy(&mut response, &mut file)
        .map_err(|e| format!("Failed to write input file: {}", e))?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn download_input(_: u32) -> Result<(), String> {
    panic!("Downloading input not supported in wasm32 target");
}
