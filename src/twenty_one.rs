use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs,
    path::Path,
};

pub fn run(p: &Path) -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string(p)?;
    println!("{}", Solution::solve_one(&s, 64));
    Ok(())
}
struct Solution {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
}
impl Solution {
    fn solve_one(s: &str, n: u32) -> u32 {
        let solution = Solution::from(s);
        let mut set = HashSet::new();
        set.insert(solution.start);
        let mut next = HashSet::new();
        for _ in 0..n {
            for (i, j) in set {
                if i > 0 && solution.grid[i - 1][j] == b'.' {
                    next.insert((i - 1, j));
                }
                if j > 0 && solution.grid[i][j - 1] == b'.' {
                    next.insert((i, j - 1));
                }
                if i + 1 < solution.grid.len() && solution.grid[i + 1][j] == b'.' {
                    next.insert((i + 1, j));
                }
                if j + 1 < solution.grid[0].len() - 1 && solution.grid[i][j + 1] == b'.' {
                    next.insert((i, j + 1));
                }
            }
            set = next;
            next = HashSet::new();
        }
        set.len() as u32
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut grid = vec![];
        let mut start = (0, 0);
        for (i, line) in value.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            grid.push(vec![]);
            for (j, b) in line.bytes().enumerate() {
                if b == b'S' {
                    grid[i].push(b'.');
                    start = (i, j);
                } else {
                    grid[i].push(b);
                }
            }
        }
        Self { grid, start }
    }
}
#[cfg(test)]
mod tests {
    use crate::twenty_one::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
",
                6
            ),
            16
        );
    }
}
