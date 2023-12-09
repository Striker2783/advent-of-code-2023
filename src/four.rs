use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    println!("Star 1: {}", solve_one(&contents));
    println!("Star 2: {}", solve_two(&contents));
    Ok(())
}

fn solve_one(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line.split(": ").nth(1).unwrap();
        let mut split = numbers.split(" | ");
        let mut winning_nums = vec![];
        for num in split.next().unwrap().split(' ') {
            if let Ok(n) = num.parse::<u32>() {
                winning_nums.push(n);
            }
        }
        let mut score = 0;
        for num in split.next().unwrap().split(' ') {
            if let Ok(n) = num.parse::<u32>() {
                if !winning_nums.contains(&n) {
                    continue;
                };
                if score == 0 {
                    score = 1;
                } else {
                    score <<= 1;
                }
            }
        }
        sum += score;
    }
    sum
}
fn solve_two(input: &str) -> u32 {
    let mut vec = vec![1];
    for (i, line) in input.lines().enumerate() {
        let copies = {
            if let Some(n) = vec.get(i) {
                *n
            } else {
                vec.push(1);
                1
            }
        };
        let numbers = line.split(": ").nth(1).unwrap();
        let mut split = numbers.split(" | ");
        let mut winning_nums = vec![];
        for num in split.next().unwrap().split(' ') {
            if let Ok(n) = num.parse::<u32>() {
                winning_nums.push(n);
            }
        }
        let mut curr = i;
        for num in split.next().unwrap().split(' ') {
            if let Ok(n) = num.parse::<u32>() {
                if !winning_nums.contains(&n) {
                    continue;
                };
                curr += 1;
                if let Some(n) = vec.get(curr) {
                    vec[curr] = *n + copies;
                } else {
                    vec.push(copies + 1);
                }
            }
        }
    }
    vec.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::four::{solve_one, solve_two};

    #[test]
    fn test_solve_one() {
        assert_eq!(
            solve_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            solve_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
