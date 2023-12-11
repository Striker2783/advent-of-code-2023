use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let input = Input::parse(&content);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two(1_000_000));
    Ok(())
}
#[derive(Debug)]
struct Input(Vec<Vec<char>>);
impl Input {
    fn solve_one(&self) -> u64 {
        let changed = self.one_change('.');
        Self::refactored(changed, |i, j| (i, j))
    }
    fn solve_two(&self, times: u64) -> u64 {
        let changed = self.one_change('!');
        let (rows, columns) = Self::get_stores(&changed, times);
        Self::refactored(changed, |i, j| (rows[i] as usize, columns[j] as usize))
    }
    fn get_stores(changed: &[Vec<char>], times: u64) -> (Vec<u64>, Vec<u64>) {
        let times = times - 1;
        let mut curr = 0;
        let rows: Vec<_> = changed
            .iter()
            .map(|c| {
                if c[0] == '!' {
                    curr += times;
                } else {
                    curr += 1;
                }
                curr
            })
            .collect();
        let mut curr = 0;
        let columns: Vec<_> = changed[0]
            .iter()
            .copied()
            .map(|c| {
                if c == '!' {
                    curr += times;
                } else {
                    curr += 1;
                }
                curr
            })
            .collect();
        (rows, columns)
    }
    fn pretty(a: &Vec<Vec<char>>) {
        for l in a {
            println!("{l:?}");
        }
    }
    fn one_change(&self, c: char) -> Vec<Vec<char>> {
        let mut vec = self.0.clone();
        // Add rows
        let mut i = 0;
        let curr_len = vec[0].len();
        while i < vec.len() {
            if vec[i].iter().copied().all(|c| c == '.') {
                vec.insert(i, vec![c; curr_len]);
                i += 1;
            }
            i += 1;
        }
        // Add columns
        let mut j = 0;
        while j < vec[0].len() {
            if vec.iter().all(|v| v[j] == '.' || v[j] == c) {
                vec.iter_mut().for_each(|v| v.insert(j, c));
                j += 1;
            }
            j += 1;
        }
        vec
    }
    fn parse(a: &str) -> Self {
        Self(a.lines().map(|l| l.chars().collect()).collect())
    }

    fn refactored(
        changed: Vec<Vec<char>>,
        func: impl Copy + Fn(usize, usize) -> (usize, usize),
    ) -> u64 {
        let pairs: Vec<_> = changed
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .filter_map(move |(j, &c)| if c == '#' { Some(func(i, j)) } else { None })
            })
            .collect();
        pairs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, one)| {
                pairs.iter().skip(i + 1).copied().fold(0, |mut acc, two| {
                    acc += two.0.max(one.0) - two.0.min(one.0);
                    acc += two.1.max(one.1) - two.1.min(one.1);
                    acc
                })
            })
            .sum::<usize>() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let i = Input::parse("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....");
        assert_eq!(i.solve_one(), 374);
    }

    #[test]
    fn two() {
        let i = Input::parse("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....");
        assert_eq!(i.solve_two(10), 1030);
        assert_eq!(i.solve_two(100), 8410);
    }
}
