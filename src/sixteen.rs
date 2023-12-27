use std::{collections::HashSet, error::Error, fs, path::Path};
pub fn run(p: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(p)?;
    let input = Input::from(a.as_str());
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    fn next(&self, pos: (usize, usize), max: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Direction::Down => {
                if pos.0 + 1 >= max.0 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            Direction::Right => {
                if pos.1 + 1 >= max.1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            Direction::Left => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
        }
    }
}
#[derive(Debug)]
struct Input(Vec<Vec<char>>);
impl Input {
    fn add_to_vec(
        d: Direction,
        pos: (usize, usize),
        max: (usize, usize),
        curr: &mut Vec<((usize, usize), Direction)>,
    ) {
        if let Some(next) = d.next(pos, max) {
            curr.push((next, d));
        }
    }
    fn solve_one(&self) -> u64 {
        let start = ((0, 0), Direction::Right);
        self.refactor(start)
    }
    fn solve_two(&self) -> u64 {
        let mut max = 0;
        let num_rows = self.0.len();
        let num_columns = self.0[0].len();
        for i in 0..num_rows {
            max = max.max(self.refactor(((i, 0), Direction::Right)));
            max = max.max(self.refactor(((i, num_columns - 1), Direction::Left)));
        }
        for i in 0..num_columns {
            max = max.max(self.refactor(((0, i), Direction::Down)));
            max = max.max(self.refactor(((num_columns - 1, i), Direction::Up)));
        }

        max
    }

    fn refactor(&self, start: ((usize, usize), Direction)) -> u64 {
        let mut set = HashSet::new();
        let num_rows = self.0.len();
        let num_columns = self.0[0].len();
        let max = (num_rows, num_columns);
        let mut visited = vec![vec![false; num_columns]; num_rows];
        let mut curr = vec![start];
        while let Some((pos, d)) = curr.pop() {
            if set.contains(&(pos, d)) {
                continue;
            }
            set.insert((pos, d));
            visited[pos.0][pos.1] = true;
            let char = self.0[pos.0][pos.1];
            match char {
                '.' => {
                    Self::add_to_vec(d, pos, max, &mut curr);
                }
                '-' => match d {
                    Direction::Down | Direction::Up => {
                        Self::add_to_vec(Direction::Right, pos, max, &mut curr);
                        Self::add_to_vec(Direction::Left, pos, max, &mut curr);
                    }
                    Direction::Left | Direction::Right => {
                        Self::add_to_vec(d, pos, max, &mut curr);
                    }
                },
                '|' => match d {
                    Direction::Left | Direction::Right => {
                        Self::add_to_vec(Direction::Up, pos, max, &mut curr);
                        Self::add_to_vec(Direction::Down, pos, max, &mut curr);
                    }
                    Direction::Down | Direction::Up => {
                        Self::add_to_vec(d, pos, max, &mut curr);
                    }
                },
                '/' => match d {
                    Direction::Up => Self::add_to_vec(Direction::Right, pos, max, &mut curr),
                    Direction::Down => Self::add_to_vec(Direction::Left, pos, max, &mut curr),
                    Direction::Right => Self::add_to_vec(Direction::Up, pos, max, &mut curr),
                    Direction::Left => Self::add_to_vec(Direction::Down, pos, max, &mut curr),
                },
                '\\' => match d {
                    Direction::Up => Self::add_to_vec(Direction::Left, pos, max, &mut curr),
                    Direction::Down => Self::add_to_vec(Direction::Right, pos, max, &mut curr),
                    Direction::Right => Self::add_to_vec(Direction::Down, pos, max, &mut curr),
                    Direction::Left => Self::add_to_vec(Direction::Up, pos, max, &mut curr),
                },
                _ => unreachable!(),
            }
        }
        visited
            .iter()
            .map(|v| v.iter().filter(|b| **b).count() as u64)
            .sum()
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|l| l.chars().collect()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let input = Input::from(
            ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....",
        );
        assert_eq!(input.solve_one(), 46)
    }
    #[test]
    fn two() {
        let input = Input::from(
            ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....",
        );
        assert_eq!(input.solve_two(), 51)
    }
}
