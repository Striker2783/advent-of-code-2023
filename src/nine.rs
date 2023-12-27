use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let input = Input::parse(&contents);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug, Default)]
struct Input(Vec<Vec<i64>>);
impl Input {
    fn new(a: Vec<Vec<i64>>) -> Self {
        Self(a)
    }
    fn parse(l: &str) -> Self {
        Self::new(
            l.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.split(' ').filter_map(|n| n.parse().ok()).collect())
                .collect(),
        )
    }
    fn refactor(&self, mut func: impl FnMut(&mut Vec<Vec<i64>>) -> i64) -> i64 {
        self.0
            .iter()
            .map(|a| {
                let mut matrix = vec![a.clone()];
                loop {
                    let curr = matrix.last().unwrap().clone();
                    if curr.iter().copied().all(|x| x == 0) {
                        break;
                    }
                    let value = curr
                        .iter()
                        .skip(1)
                        .copied()
                        .zip(curr.iter().copied())
                        .map(|(curr, prev)| curr - prev)
                        .collect();
                    matrix.push(value);
                }
                func(&mut matrix)
            })
            .sum()
    }
    fn solve_one(&self) -> i64 {
        self.refactor(|matrix| {
            for i in (1..matrix.len()).rev() {
                let curr_last = *matrix[i].last().unwrap();
                let prev_last = *matrix[i - 1].last().unwrap();
                matrix[i - 1].push(curr_last + prev_last);
            }
            *matrix.first().unwrap().last().unwrap()
        })
    }
    fn solve_two(&self) -> i64 {
        self.refactor(|matrix| {
            for i in (1..matrix.len()).rev() {
                let curr_first = *matrix[i].first().unwrap();
                let prev_first = *matrix[i - 1].first().unwrap();
                matrix[i - 1].insert(0, prev_first - curr_first);
            }
            *matrix.first().unwrap().first().unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_one() {
        let input = Input::parse("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45");
        assert_eq!(input.solve_one(), 114);
    }
    #[test]
    fn test_two() {
        let input = Input::parse("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45");
        assert_eq!(input.solve_two(), 2);
    }
}
