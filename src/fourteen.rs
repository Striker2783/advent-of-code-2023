use std::{collections::HashMap, error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(path)?;
    let i = Input::parse(&a);
    println!("Star 1: {}", i.solve_one());
    println!("Star 2: {}", i.solve_two());
    Ok(())
}
type Vecs = Vec<Vec<char>>;
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Input(Vecs);
impl Input {
    fn solve_one(&self) -> u64 {
        Self::get_score(&self.go_north())
    }

    fn get_score(v: &Vecs) -> u64 {
        v.iter()
            .rev()
            .enumerate()
            .map(|(i, v)| {
                let i = i + 1;
                i * v.iter().filter(|c| **c == 'O').count()
            })
            .sum::<usize>() as u64
    }
    fn solve_two(&self) -> u64 {
        let mut curr = self.clone();
        let mut seen = vec![curr.clone()];
        loop {
            curr = Self(curr.go_north());
            curr = Self(curr.go_west());
            curr = Self(curr.go_south());
            curr = Self(curr.go_east());
            if let Some(i) = seen.iter().position(|x| x == &curr) {
                let cycle_len = seen.len() - i;
                let start = i;
                let final_cycle = seen[start + (1000000000 - start) % cycle_len].clone();
                return Self::get_score(&final_cycle.0);
            }
            seen.push(curr.clone());
        }
    }
    fn go_west(&self) -> Vecs {
        let v = &self.0;
        let f = |v: &mut Vecs, i: usize, j: usize| {
            for j in (0..j).rev() {
                if v[i][j] == '.' {
                    continue;
                }
                v[i][j + 1] = 'O';
                return;
            }
            v[i][0] = 'O';
        };
        let mut new: Vec<Vec<char>> = v
            .iter()
            .map(|v| {
                v.iter()
                    .copied()
                    .map(|c| if c == 'O' { '.' } else { c })
                    .collect()
            })
            .collect();
        for (i, v) in v.iter().enumerate() {
            for (j, c) in v.iter().copied().enumerate() {
                if c == '.' || c == '#' {
                    continue;
                }
                f(&mut new, i, j);
            }
        }
        new
    }
    fn go_east(&self) -> Vecs {
        let v = &self.0;
        let f = |v: &mut Vecs, i: usize, j: usize| {
            let len = v[0].len();
            for j in (j + 1)..len {
                if v[i][j] == '.' {
                    continue;
                }
                v[i][j - 1] = 'O';
                return;
            }
            v[i][len - 1] = 'O';
        };
        let mut new: Vec<Vec<char>> = v
            .iter()
            .map(|v| {
                v.iter()
                    .copied()
                    .map(|c| if c == 'O' { '.' } else { c })
                    .collect()
            })
            .collect();
        for (i, v) in v.iter().enumerate() {
            for (j, c) in v.iter().copied().enumerate().rev() {
                if c == '.' || c == '#' {
                    continue;
                }
                f(&mut new, i, j);
            }
        }
        new
    }
    fn go_south(&self) -> Vecs {
        let v = &self.0;
        let f = |v: &mut Vecs, i, j| {
            let len = v.len();
            for i in (i + 1)..len {
                if v[i][j] == '.' {
                    continue;
                }
                v[i - 1][j] = 'O';
                return;
            }
            v[len - 1][j] = 'O';
        };
        let mut new: Vec<Vec<char>> = v
            .iter()
            .map(|v| {
                v.iter()
                    .copied()
                    .map(|c| if c == 'O' { '.' } else { c })
                    .collect()
            })
            .collect();
        for (i, v) in v.iter().enumerate().rev() {
            for (j, c) in v.iter().copied().enumerate() {
                if c == '.' || c == '#' {
                    continue;
                }
                f(&mut new, i, j);
            }
        }
        new
    }
    fn go_north(&self) -> Vecs {
        let v = &self.0;
        let f = |v: &mut Vec<Vec<char>>, i: usize, j: usize| {
            for i in (0..i).rev() {
                if v[i][j] == '.' {
                    continue;
                }
                v[i + 1][j] = 'O';
                return;
            }
            v[0][j] = 'O';
        };
        let mut new: Vec<Vec<char>> = v
            .iter()
            .map(|v| {
                v.iter()
                    .copied()
                    .map(|c| if c == 'O' { '.' } else { c })
                    .collect()
            })
            .collect();
        for (i, v) in v.iter().enumerate() {
            for (j, c) in v.iter().copied().enumerate() {
                if c == '.' || c == '#' {
                    continue;
                }
                f(&mut new, i, j);
            }
        }
        new
    }
    fn parse(a: &str) -> Self {
        Self(a.lines().map(|l| l.chars().collect()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let i = Input::parse("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....");
        assert_eq!(i.solve_one(), 136);
    }
    #[test]
    fn two() {
        let i = Input::parse("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....");
        assert_eq!(i.solve_two(), 64);
    }
}
