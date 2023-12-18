use std::{error::Error, fs, path::Path};

pub fn run(p: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(p)?;
    let input = Input::from(a.as_str());
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug)]
struct Input(Vec<Line>);
impl Input {
    fn solve_one(&self) -> u64 {
        let (boundary, vertices) = Self::get_vertices(&self.0);
        Self::get_area(&vertices, boundary)
    }

    fn get_vertices(v: &[Line]) -> (u64, Vec<(u64, u64)>) {
        let mut vertices = vec![(0, 0)];
        let mut curr = (0, 0);
        let mut boundary = 0;
        for l in v {
            match l.dir {
                'R' => curr.1 += l.num as i64,
                'L' => curr.1 -= l.num as i64,
                'U' => curr.0 -= l.num as i64,
                'D' => curr.0 += l.num as i64,
                _ => unreachable!(),
            }
            boundary += l.num as u64;
            vertices.push(curr);
        }
        let mut min = (i64::MAX, i64::MAX);
        for v in &vertices {
            if v.0 < min.0 {
                min.0 = v.0;
            }
            if v.1 < min.1 {
                min.1 = v.1;
            }
        }
        let vertices = vertices
            .iter()
            .map(|v| ((v.0 - min.0) as u64, (v.1 - min.1) as u64))
            .collect::<Vec<_>>();
        (boundary, vertices)
    }
    fn get_area(vertices: &Vec<(u64, u64)>, boundary: u64) -> u64 {
        let mut right = 0;
        let mut left = 0;
        for i in 0..(vertices.len() - 1) {
            let p1 = vertices[i];
            let p2 = vertices[i + 1];
            left += p1.0 * p2.1;
            right += p2.0 * p1.1;
        }
        if left > right {
            (left - right) / 2 + boundary / 2 + 1
        } else {
            (right - left) / 2 + boundary / 2 + 1
        }
    }
    fn solve_two(&self) -> u64 {
        let lines = self.0.iter().map(Line::part_two).collect::<Vec<_>>();
        let (boundary, vertices) = Self::get_vertices(&lines);
        Self::get_area(&vertices, boundary)
    }
    fn solve_one_naive(&self) -> u32 {
        let mut hit = vec![vec![true]];
        let mut curr = (0, 0);
        for l in &self.0 {
            for _ in 0..l.num {
                match l.dir {
                    'R' => {
                        curr.1 += 1;
                        if curr.1 >= hit[0].len() {
                            Self::add_column(&mut hit, false);
                        }
                        hit[curr.0][curr.1] = true;
                    }
                    'L' => {
                        if curr.1 == 0 {
                            Self::add_column(&mut hit, true);
                        } else {
                            curr.1 -= 1;
                        }
                        hit[curr.0][curr.1] = true;
                    }
                    'D' => {
                        curr.0 += 1;
                        if curr.0 >= hit.len() {
                            Self::add_row(&mut hit, false);
                        }
                        hit[curr.0][curr.1] = true;
                    }
                    'U' => {
                        if curr.0 == 0 {
                            Self::add_row(&mut hit, true);
                        } else {
                            curr.0 -= 1;
                        }
                        hit[curr.0][curr.1] = true;
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
        let mut stack = vec![];
        'all: for i in 0..hit.len() {
            'out: for j in 0..hit[i].len() {
                if hit[i][j] {
                    continue;
                }
                let mut consecutive = false;
                let mut count = 0;
                for k in (j + 1)..hit[i].len() {
                    if hit[i][k] {
                        if consecutive {
                            continue 'out;
                        } else {
                            consecutive = true;
                            count += 1;
                        }
                    }
                }
                if count % 2 == 1 {
                    stack.push((i, j));
                    break 'all;
                }
            }
        }
        while let Some((y, x)) = stack.pop() {
            if hit[y][x] {
                continue;
            }
            hit[y][x] = true;
            if y > 0 {
                stack.push((y - 1, x))
            }
            if x > 0 {
                stack.push((y, x - 1))
            }
            if y + 1 < hit.len() {
                stack.push((y + 1, x))
            }
            if x + 1 < hit[0].len() {
                stack.push((y, x + 1))
            }
        }
        hit.iter()
            .map(|v| v.iter().filter(|b| **b).count())
            .sum::<usize>() as u32
    }
    fn add_row(v: &mut Vec<Vec<bool>>, up: bool) {
        if v.is_empty() {
            v.push(vec![]);
            return;
        }
        if up {
            let len = v[0].len();
            v.insert(0, vec![false; len]);
        } else {
            let len = v[0].len();
            v.push(vec![false; len]);
        }
    }
    fn add_column(v: &mut [Vec<bool>], left: bool) {
        for v in v {
            if left {
                v.insert(0, false);
            } else {
                v.push(false);
            }
        }
    }
    fn graphic(v: &[Vec<bool>]) {
        for l in v {
            for b in l {
                let c = if *b { "#" } else { "." };
                print!("{c}")
            }
            println!()
        }
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self(value.lines().map(Line::from).collect())
    }
}
#[derive(Debug)]
struct Line {
    dir: char,
    num: u32,
    color: String,
}
impl Line {
    fn part_two(&self) -> Self {
        let mut num = 0;
        for c in self.color[..5].chars() {
            let n = c.to_digit(16).unwrap();
            num = num * 16 + n;
        }
        let dir = match self.color.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!(),
        };
        Self {
            dir,
            num,
            color: self.color.clone(),
        }
    }
}
impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        Self {
            dir: split.next().unwrap().chars().nth(0).unwrap(),
            num: split.next().unwrap().parse().unwrap(),
            color: split
                .next()
                .unwrap()
                .chars()
                .skip(2)
                .take_while(char::is_ascii_hexdigit)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let input = Input::from("R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)");
        assert_eq!(input.solve_one(), 62)
    }
    #[test]
    fn two() {
        let input = Input::from("R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)");
        assert_eq!(input.solve_two(), 952408144115)
    }
}
