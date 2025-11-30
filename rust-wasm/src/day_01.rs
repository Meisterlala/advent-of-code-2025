crate::solution!(1, "The first day XXXXX", solve_a, solve_b);

pub fn solve_a(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            sum += first * 10 + last;
        }
    }
    sum
}

pub fn solve_b(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut numbers: Vec<u32> = Vec::new();

        // Iterate over all substrings of line
        for sub_start in 0..line.len() {
            // with any length
            for sub_end in sub_start..line.len() {
                let substring = &line[sub_start..(sub_end + 1)];

                // If substring is a number, add it to the list of numbers
                if let Some(number) = parse_substring(substring) {
                    numbers.push(number);
                    break;
                }
            }
        }

        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            sum += first * 10 + last;
        }
    }
    sum
}

fn spelled_digit(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_substring(input: &str) -> Option<u32> {
    if input.len() == 1 {
        input.parse::<u32>().ok()
    } else {
        spelled_digit(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn individual() {
        assert_eq!(solve_a("1abc2"), 12);
        assert_eq!(solve_a("pqr3stu8vwx"), 38);
        assert_eq!(solve_a("a1b2c3d4e5f"), 15);
        assert_eq!(solve_a("treb7uchet"), 77);
    }

    #[test]
    fn multiple() {
        assert_eq!(solve_a("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    }

    #[test]
    fn parse_29() {
        assert_eq!(solve_b("two1nine"), 29);
    }

    #[test]
    fn parse_83() {
        assert_eq!(solve_b("eightwothree"), 83);
    }

    #[test]
    fn parse_13() {
        assert_eq!(solve_b("abcone2threexyz"), 13);
    }

    #[test]
    fn parse_24() {
        assert_eq!(solve_b("xtwone3four"), 24);
    }

    #[test]
    fn parse_42() {
        assert_eq!(solve_b("4nineeightseven2"), 42);
    }

    #[test]
    fn parse_14() {
        assert_eq!(solve_b("zoneight234"), 14);
    }

    #[test]
    fn parse_76() {
        assert_eq!(solve_b("7pqrstsixteen"), 76);
    }

    #[test]
    fn parse_sum() {
        assert_eq!(
            solve_b(
                "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
            ),
            281
        );
    }

    #[test]
    fn parse_18() {
        assert_eq!(solve_b("oneight"), 18);
    }
}
