use std::{collections::HashMap, error::Error, fs, path::Path};
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let c = fs::read_to_string(path)?;
    let input = Input::parse(&c);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug)]
struct Input(Vec<(Vec<char>, Vec<u64>)>);
impl Input {
    fn solve_one(&self) -> u64 {
        self.0
            .iter()
            .map(|(a, b)| {
                let mut map = HashMap::new();
                Self::less_naive(a.clone(), b, &mut map, 0)
            })
            .sum()
    }
    fn less_naive(
        a: Vec<char>,
        b: &[u64],
        map: &mut HashMap<(Vec<char>, Vec<u64>, u64), u64>,
        consecutive: u64,
    ) -> u64 {
        if let Some(n) = map.get(&(a.clone(), b.to_vec(), consecutive)) {
            *n
        } else if b.is_empty() {
            if a.iter().copied().any(|c| c == '#') {
                return 0;
            } else {
                return 1;
            }
        } else if a.is_empty() {
            if b.len() == 1 && b[0] == consecutive {
                return 1;
            } else {
                return 0;
            }
        } else if consecutive > b[0] {
            return 0;
        } else if a[0] == '.' {
            if b[0] == consecutive {
                return Self::less_naive(a[1..].to_vec(), &b[1..], map, 0);
            } else if consecutive == 0 {
                return Self::less_naive(a[1..].to_vec(), b, map, 0);
            } else {
                return 0;
            }
        } else if a[0] == '#' {
            if consecutive >= b[0] {
                return 0;
            } else {
                return Self::less_naive(a[1..].to_vec(), b, map, consecutive + 1);
            }
        } else {
            let mut o = a.clone();
            o[0] = '#';
            let mut p = a.clone();
            p[0] = '.';
            let result =
                Self::less_naive(p, b, map, consecutive) + Self::less_naive(o, b, map, consecutive);
            map.insert((a, b.to_vec(), consecutive), result);
            result
        }
    }
    fn solve_two(&self) -> u64 {
        self.0
            .iter()
            .map(|(a, b)| {
                let mut new_a = (0..5)
                    .flat_map(|_| {
                        let mut a = a.clone();
                        a.push('?');
                        a
                    })
                    .collect::<Vec<_>>();
                new_a.pop();
                let new_b = (0..5).flat_map(|_| b.clone()).collect::<Vec<_>>();
                let mut map = HashMap::new();
                Self::less_naive(new_a, &new_b, &mut map, 0)
            })
            .sum()
    }
    fn naive(a: &mut [char], b: &[u64], i: usize, map: &mut HashMap<Vec<char>, u64>) -> u64 {
        if i >= a.len() {
            if Self::works(a, b) {
                return 1;
            } else {
                return 0;
            }
        }
        if a[i] == '.' || a[i] == '#' {
            return Self::naive(a, b, i + 1, map);
        }
        a[i] = '#';
        let first = Self::naive(a, b, i + 1, map);
        a[i] = '.';
        let second = Self::naive(a, b, i + 1, map);
        a[i] = '?';
        second + first
    }
    fn works(c: &[char], n: &[u64]) -> bool {
        let mut i = 0;
        let mut consecutive = 0;
        for &char in c {
            if char == '#' {
                consecutive += 1;
                continue;
            } else if consecutive == 0 {
                continue;
            }
            if i >= n.len() {
                return false;
            }
            if consecutive == n[i] {
                consecutive = 0;
                i += 1;
            } else {
                return false;
            }
        }
        (i == n.len() && consecutive == 0) || (i + 1 == n.len() && n[i] == consecutive)
    }
    fn parse(a: &str) -> Self {
        Self(
            a.lines()
                .map(|a| {
                    let mut split = a.split(' ');
                    (
                        split.next().unwrap().chars().collect(),
                        split
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let input = Input::parse("???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1");
        // let input = Input::parse("??? 1,1");
        assert_eq!(input.solve_one(), 21);
    }
    #[test]
    fn two() {
        let input = Input::parse("???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1");
        // let input = Input::parse(".??..??...?##. 1,1,3");
        assert_eq!(input.solve_two(), 525152);
    }
}
