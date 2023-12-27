use std::{borrow::BorrowMut, collections::HashMap, error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    println!("Star 1: {}", solve_one(&contents));
    println!("Star 2: {}", solve_two(&contents));
    Ok(())
}

fn is_adjacent(i: usize, j: usize, chars: &[&[u8]]) -> bool {
    let elements = get_adjs(i, j);
    for (i, j) in elements {
        let c = match chars.get(i) {
            Some(c) => match c.get(j) {
                Some(c) => *c as char,
                None => continue,
            },
            None => continue,
        };
        if c.is_numeric() || c == '.' {
            continue;
        }
        return true;
    }
    false
}

fn get_adjs(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut elements = vec![(i, j + 1), (i + 1, j), (i + 1, j + 1)];
    if i > 0 {
        elements.push((i - 1, j));
        elements.push((i - 1, j + 1));
    }
    if j > 0 && i > 0 {
        elements.push((i - 1, j - 1));
    }
    if j > 0 {
        elements.push((i, j - 1));
        elements.push((i + 1, j - 1));
    }
    elements
}

fn solve_one(input: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    for (i, line) in input.lines().enumerate() {
        let mut chars = line.chars().enumerate();
        while let Some((mut j, mut c)) = chars.next() {
            let mut is_adj = false;
            let mut num = 0;
            while let Some(n) = c.to_digit(10) {
                num = num * 10 + n;
                is_adj = is_adj || is_adjacent(i, j, &lines);
                (j, c) = match chars.next() {
                    None => break,
                    Some(c) => c,
                }
            }
            if is_adj {
                sum += num;
            }
        }
    }
    sum
}
fn add_to_hash(
    i: usize,
    j: usize,
    num: u32,
    map: &mut HashMap<(usize, usize), (u32, u32)>,
) -> bool {
    let elements = get_adjs(i, j);
    for coord in elements {
        if let Some(v) = map.get_mut(&coord) {
            *v = (v.0.saturating_mul(num), v.1 + 1);
            return true;
        }
    }
    false
}
fn solve_two(input: &str) -> u32 {
    let mut sum = 0;
    let mut nums = vec![];
    let mut gears = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut chars = line.chars().enumerate();
        while let Some((mut j, mut c)) = chars.next() {
            let mut num = 0;
            let mut len = 0;
            while let Some(n) = c.to_digit(10) {
                num = num * 10 + n;
                len += 1;
                (j, c) = match chars.next() {
                    None => break,
                    Some(c) => c,
                }
            }
            if c == '*' {
                gears.insert((i, j), (1, 0));
            }
            if num > 0 {
                nums.push((i, (j, len), num));
            }
        }
    }
    'o: for (i, (j, len), num) in nums {
        for j in (j - len)..j {
            if add_to_hash(i, j, num, gears.borrow_mut()) {
                continue 'o;
            };
        }
    }
    for (_, (ratio, neighbors)) in gears {
        if neighbors != 2 {
            continue;
        }
        sum += ratio;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::three::{solve_one, solve_two};

    #[test]
    fn test_solve_one() {
        assert_eq!(
            solve_one(
                "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."
            ),
            4361
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            solve_two(
                "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."
            ),
            467835
        );
    }
}
