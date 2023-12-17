use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs,
    path::Path,
};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(path)?;
    let input = Input::parse(&a);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    None,
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
}
struct Input(Vec<Vec<u32>>);
impl Input {
    fn dikstra(
        input: &Vec<Vec<u32>>,
        neighbors: impl Fn(
            (usize, usize),
            Direction,
            (usize, usize),
        ) -> Vec<((usize, usize), Direction)>,
    ) -> u32 {
        let max = (input.len(), input[0].len());
        let start = (0, 0);
        let mut pq = BinaryHeap::new();
        let mut dist = HashMap::new();
        dist.insert((start, Direction::None), (0, (0, 0)));
        pq.push((Reverse(0), start, Direction::None));
        while let Some((_, pos, dir)) = pq.pop() {
            if pos == (max.0 - 1, max.1 - 1) {
                return dist.get(&(pos, dir)).unwrap().0;
            }
            for (pos2, dir2) in neighbors(pos, dir, max) {
                let new = dist.get(&(pos, dir)).unwrap().0 + input[pos2.0][pos2.1];
                if new < dist.get(&(pos2, dir2)).get_or_insert(&(u32::MAX, (0, 0))).0 {
                    dist.insert((pos2, dir2), (new, pos));
                    pq.push((Reverse(new), pos2, dir2));
                }
            }
        }
        unreachable!()
    }
    fn neighbors2(
        pos: (usize, usize),
        run: Direction,
        max: (usize, usize),
    ) -> Vec<((usize, usize), Direction)> {
        match run {
            Direction::None => Self::neighbors(pos, run, max, 10),
            Direction::Up(n) => {
                if n < 4 {
                    if pos.0 > 0 {
                        vec![((pos.0 - 1, pos.1), Direction::Up(n + 1))]
                    } else {
                        vec![]
                    }
                } else {
                    Self::neighbors(pos, run, max, 10)
                }
            }
            Direction::Down(n) => {
                if n < 4 {
                    if pos.0 + 1 < max.0 {
                        vec![((pos.0 + 1, pos.1), Direction::Down(n + 1))]
                    } else {
                        vec![]
                    }
                } else {
                    Self::neighbors(pos, run, max, 10)
                }
            }
            Direction::Right(n) => {
                if n < 4 {
                    if pos.1 + 1 < max.1 {
                        vec![((pos.0, pos.1 + 1), Direction::Right(n + 1))]
                    } else {
                        vec![]
                    }
                } else {
                    Self::neighbors(pos, run, max, 10)
                }
            }
            Direction::Left(n) => {
                if n < 4 {
                    if pos.1 > 0 {
                        vec![((pos.0, pos.1 - 1), Direction::Left(n + 1))]
                    } else {
                        vec![]
                    }
                } else {
                    Self::neighbors(pos, run, max, 10)
                }
            }
        }
    }
    fn neighbors(
        pos: (usize, usize),
        run: Direction,
        max: (usize, usize),
        max_runs: u32,
    ) -> Vec<((usize, usize), Direction)> {
        let mut vec = vec![];
        match run {
            Direction::Down(_) | Direction::Up(_) => {
                if pos.1 > 0 {
                    vec.push(((pos.0, pos.1 - 1), Direction::Left(1)))
                }
                if pos.1 + 1 < max.1 {
                    vec.push(((pos.0, pos.1 + 1), Direction::Right(1)))
                }
            }
            Direction::Left(_) | Direction::Right(_) => {
                if pos.0 > 0 {
                    vec.push(((pos.0 - 1, pos.1), Direction::Up(1)))
                }
                if pos.0 + 1 < max.0 {
                    vec.push(((pos.0 + 1, pos.1), Direction::Down(1)))
                }
            }
            Direction::None => {
                if pos.0 > 0 {
                    vec.push(((pos.0 - 1, pos.1), Direction::Up(1)))
                }
                if pos.0 + 1 < max.0 {
                    vec.push(((pos.0 + 1, pos.1), Direction::Down(1)))
                }
                if pos.1 > 0 {
                    vec.push(((pos.0, pos.1 - 1), Direction::Left(1)))
                }
                if pos.1 + 1 < max.1 {
                    vec.push(((pos.0, pos.1 + 1), Direction::Right(1)))
                }
            }
        }
        match run {
            Direction::Up(n) => {
                if n < max_runs && pos.0 > 0 {
                    vec.push(((pos.0 - 1, pos.1), Direction::Up(n + 1)))
                }
            }
            Direction::Down(n) => {
                if n < max_runs && pos.0 + 1 < max.0 {
                    vec.push(((pos.0 + 1, pos.1), Direction::Down(n + 1)))
                }
            }
            Direction::Right(n) => {
                if n < max_runs && pos.1 + 1 < max.1 {
                    vec.push(((pos.0, pos.1 + 1), Direction::Right(n + 1)))
                }
            }
            Direction::Left(n) => {
                if n < max_runs && pos.1 > 0 {
                    vec.push(((pos.0, pos.1 - 1), Direction::Left(n + 1)))
                }
            }
            Direction::None => (),
        }
        vec
    }
    fn solve_one(&self) -> u32 {
        Self::dikstra(&self.0, |p, d, m| Self::neighbors(p, d, m, 3))
    }
    fn solve_two(&self) -> u32 {
        Self::dikstra(&self.0, Self::neighbors2)
    }
    fn parse(a: &str) -> Self {
        Self(
            a.lines()
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let input = Input::parse("2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533");
        assert_eq!(input.solve_one(), 102);
    }
    #[test]
    fn two() {
        let input = Input::parse("2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533");
        assert_eq!(input.solve_two(), 94);
    }
}
