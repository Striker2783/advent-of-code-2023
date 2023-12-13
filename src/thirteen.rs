use std::{error::Error, fs, path::Path};
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(path)?;
    let input = Input::parse(&a);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug)]
struct Input(Vec<Matrix>);
impl Input {
    fn parse(a: &str) -> Self {
        Self(a.split("\n\n").map(Matrix::parse).collect())
    }
    fn solve_one(&self) -> u64 {
        self.0.iter().map(Matrix::get_score_one).sum()
    }
    fn solve_two(&self) -> u64 {
        self.0.iter().map(Matrix::get_score_two).sum()
    }
}
#[derive(Debug)]
struct Matrix(Vec<Vec<char>>);
impl Matrix {
    fn get_score_one(&self) -> u64 {
        let row = (0..(self.0.len() - 1)).find(|i| self.is_horizontal_reflection(*i));
        if let Some(r) = row {
            return (r as u64 + 1) * 100;
        }
        let column = (0..(self.0[0].len())).find(|i| self.is_vertical_reflection(*i));
        if let Some(c) = column {
            return c as u64 + 1;
        }
        0
    }
    fn get_score_two(&self) -> u64 {
        let row = (0..(self.0.len() - 1)).find(|i| self.is_horizontal_reflection_two(*i));
        if let Some(r) = row {
            return (r as u64 + 1) * 100;
        }
        let column = (0..(self.0[0].len())).find(|i| self.is_vertical_reflection_two(*i));
        if let Some(c) = column {
            return c as u64 + 1;
        }
        0
    }
    fn is_horizontal_reflection_two(&self, i: usize) -> bool {
        if i >= self.0.len() - 1 {
            return false;
        }
        let len = self.0.len();
        let mut b = false;
        for j in 0.. {
            if j > i || i + j + 1 >= len {
                break;
            }
            let prev = i - j;
            let next = i + j + 1;
            if self.0[prev] != self.0[next] {
                if !b {
                    let mut diff = 0;
                    for (p, n) in self.0[prev]
                        .iter()
                        .copied()
                        .zip(self.0[next].iter().copied())
                    {
                        if p != n {
                            diff += 1;
                            if diff > 1 {
                                break;
                            }
                        }
                    }
                    if diff == 1 {
                        b = true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        b
    }
    fn is_vertical_reflection_two(&self, j: usize) -> bool {
        if j >= self.0[0].len() - 1 {
            return false;
        }
        let len = self.0[0].len();
        let mut b = false;
        for i in 0.. {
            if i > j || j + i + 1 >= len {
                break;
            }
            let prev = j - i;
            let next = j + i + 1;
            if self.0.iter().any(|v| v[prev] != v[next]) {
                if !b {
                    let mut diff = 0;
                    for v in self.0.iter() {
                        if v[prev] != v[next] {
                            diff += 1;
                            if diff > 1 {
                                break;
                            }
                        }
                    }
                    if diff == 1 {
                        b = true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        b
    }
    fn is_horizontal_reflection(&self, i: usize) -> bool {
        if i >= self.0.len() - 1 {
            return false;
        }
        let len = self.0.len();
        for j in 0.. {
            if j > i || i + j + 1 >= len {
                break;
            }
            let prev = i - j;
            let next = i + j + 1;
            if self.0[prev] != self.0[next] {
                return false;
            }
        }
        true
    }
    fn is_vertical_reflection(&self, j: usize) -> bool {
        if j >= self.0[0].len() - 1 {
            return false;
        }
        let len = self.0[0].len();
        for i in 0.. {
            if i > j || j + i + 1 >= len {
                break;
            }
            let prev = j - i;
            let next = j + i + 1;
            if self.0.iter().any(|v| v[prev] != v[next]) {
                return false;
            }
        }
        true
    }
    fn parse(a: &str) -> Self {
        Self(a.lines().map(|l| l.chars().collect()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_one() {
        let input = Input::parse(
        "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#",
        );
        assert_eq!(input.solve_one(), 405);
        assert_eq!(input.solve_two(), 400);
    }
}
