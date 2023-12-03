use std::{fs, path::Path};

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(path: &Path) {
    let string = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    println!("First star: {}", first(&string));
    println!("Second star: {}", second(&string));
}

fn calibration(line: &str) -> u32 {
    let first = line.chars().find_map(|c| c.to_digit(10)).unwrap_or(0);
    let chars = line.chars();
    let last = chars.rev().find_map(|c| c.to_digit(10)).unwrap_or(0);
    first * 10 + last
}

fn helper(line: impl Iterator<Item = char>, nums: &[&str]) -> u32 {
    let mut str = String::new();
    for c in line {
        if let Some(d) = c.to_digit(10) {
            return d;
        }
        str.push(c);
        for (i, num) in nums.iter().enumerate() {
            if str.contains(num) {
                return i as u32;
            }
        }
    }
    0
}

fn calibration_two(line: &str) -> u32 {
    let first = helper(line.chars(), &NUMBERS);
    let rev_numbers: Vec<String> = NUMBERS
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let rev_numbers: Vec<_> = rev_numbers.iter().map(|s| s.as_str()).collect();
    let last = helper(line.chars().rev(), &rev_numbers);

    first * 10 + last
}

fn first(input: &str) -> u32 {
    input.lines().map(calibration).sum()
}
fn second(input: &str) -> u32 {
    input.lines().map(calibration_two).sum()
}
#[cfg(test)]
mod tests {
    use crate::one::{calibration, calibration_two, first, second};

    #[test]
    fn test_calibration() {
        assert_eq!(calibration("1abc2"), 12);
        assert_eq!(calibration("pqr3stu8vmx"), 38);
    }
    #[test]
    fn test_calibration_two() {
        assert_eq!(calibration_two("two1nine"), 29);
        assert_eq!(calibration_two("4nineeightseven2"), 42);
        assert_eq!(calibration_two("abcone2threexyz"), 13);
    }
    #[test]
    fn test_first() {
        assert_eq!(first("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    }
    #[test]
    fn test_second() {
        assert_eq!(second("two1nine\neightwothree"), 29 + 83);
    }
}
