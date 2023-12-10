use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let input = Input::parse(&contents);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Start,
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Ground,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::Vertical => write!(f, "|"),
            Self::Horizontal => write!(f, "-"),
            Self::TopLeft => write!(f, "J"),
            Self::TopRight => write!(f, "L"),
            Self::BottomLeft => write!(f, "7"),
            Self::BottomRight => write!(f, "F"),
            Self::Ground => write!(f, "."),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ODirection {
    Top,
    Left,
    Bottom,
    Right,
}
#[derive(Debug)]
struct Input(Vec<Vec<Direction>>);
type Thing = ((usize, usize), ODirection);

impl Input {
    fn solve_one(&self) -> u64 {
        let start = self.get_start();
        let mut curr = self.get_pos_from_start(start);
        let mut size = 1;
        while let Some(a) = self.get_next(curr) {
            size += 1;
            curr = a;
        }
        size / 2
    }
    fn pretty(a: &Vec<Vec<Direction>>) {
        for x in a {
            println!("{x:?}");
        }
    }
    fn solve_two(&self) -> u64 {
        let mut new_thing = vec![vec![Direction::Ground; self.0[0].len()]; self.0.len()];
        let start = self.get_start();
        new_thing[start.0][start.1] = self.get_start_direction(start);
        // new_thing[start.0][start.1] = Direction::Start;
        let mut curr = self.get_pos_from_start(start);
        while let Some(a) = self.get_next(curr) {
            new_thing[curr.0 .0][curr.0 .1] = self.0[curr.0 .0][curr.0 .1];
            curr = a;
        }
        new_thing[curr.0 .0][curr.0 .1] = self.0[curr.0 .0][curr.0 .1];
        new_thing
            .iter()
            .map(|x| {
                x.iter()
                    .enumerate()
                    .filter(|(j, d)| {
                        let j = *j;
                        if **d != Direction::Ground {
                            return false;
                        }
                        (0..j)
                            .filter(|k| {
                                matches!(
                                    x[*k],
                                    // Direction::Vertical | Direction::TopRight | Direction::TopLeft
                                    Direction::Vertical
                                        | Direction::BottomLeft
                                        | Direction::BottomRight
                                )
                            })
                            .count()
                            % 2
                            == 1
                    })
                    .count()
            })
            .sum::<usize>() as u64
    }
    fn get_next(&self, o: Thing) -> Option<Thing> {
        let d = &self.0[o.0 .0][o.0 .1];
        let from = o.1;
        match d {
            Direction::Start => None,
            Direction::Vertical => {
                if from == ODirection::Bottom {
                    Some(((o.0 .0 - 1, o.0 .1), ODirection::Bottom))
                } else {
                    Some(((o.0 .0 + 1, o.0 .1), ODirection::Top))
                }
            }
            Direction::Horizontal => {
                if from == ODirection::Left {
                    Some(((o.0 .0, o.0 .1 + 1), ODirection::Left))
                } else {
                    Some(((o.0 .0, o.0 .1 - 1), ODirection::Right))
                }
            }
            Direction::TopLeft => {
                if from == ODirection::Left {
                    Some(((o.0 .0 - 1, o.0 .1), ODirection::Bottom))
                } else {
                    Some(((o.0 .0, o.0 .1 - 1), ODirection::Right))
                }
            }
            Direction::TopRight => {
                if from == ODirection::Right {
                    Some(((o.0 .0 - 1, o.0 .1), ODirection::Bottom))
                } else {
                    Some(((o.0 .0, o.0 .1 + 1), ODirection::Left))
                }
            }
            Direction::BottomLeft => {
                if from == ODirection::Left {
                    Some(((o.0 .0 + 1, o.0 .1), ODirection::Top))
                } else {
                    Some(((o.0 .0, o.0 .1 - 1), ODirection::Right))
                }
            }
            Direction::BottomRight => {
                if from == ODirection::Right {
                    Some(((o.0 .0 + 1, o.0 .1), ODirection::Top))
                } else {
                    Some(((o.0 .0, o.0 .1 + 1), ODirection::Left))
                }
            }
            Direction::Ground => unreachable!(),
        }
    }
    fn get_pos_from_start(&self, start: (usize, usize)) -> ((usize, usize), ODirection) {
        if start.0 != 0
            && matches!(
                self.0[start.0 - 1][start.1],
                Direction::Vertical | Direction::BottomRight | Direction::BottomLeft
            )
        {
            ((start.0 - 1, start.1), ODirection::Bottom)
        } else if start.1 != 0
            && matches!(
                self.0[start.0][start.1 - 1],
                Direction::Horizontal | Direction::BottomRight | Direction::TopRight
            )
        {
            return ((start.0, start.1 - 1), ODirection::Right);
        } else {
            ((start.0, start.1 + 1), ODirection::Left)
        }
    }
    fn get_start_direction(&self, start: (usize, usize)) -> Direction {
        let mut dirs = vec![];
        if start.0 != 0
            && matches!(
                self.0[start.0 - 1][start.1],
                Direction::Vertical | Direction::BottomRight | Direction::BottomLeft
            )
        {
            dirs.push(ODirection::Top);
        }
        if start.1 != 0
            && matches!(
                self.0[start.0][start.1 - 1],
                Direction::Horizontal | Direction::BottomRight | Direction::TopRight
            )
        {
            dirs.push(ODirection::Left);
        }
        if start.0 < self.0.len()
            && matches!(
                self.0[start.0 + 1][start.1],
                Direction::Vertical | Direction::TopLeft | Direction::TopRight
            )
        {
            dirs.push(ODirection::Bottom);
        }
        if start.1 < self.0[0].len()
            && matches!(
                self.0[start.0][start.1 + 1],
                Direction::Horizontal | Direction::TopLeft | Direction::BottomLeft
            )
        {
            dirs.push(ODirection::Right);
        }
        match (dirs[0], dirs[1]) {
            (ODirection::Right, ODirection::Left) | (ODirection::Left, ODirection::Right) => {
                Direction::Horizontal
            }
            (ODirection::Top, ODirection::Bottom) | (ODirection::Bottom, ODirection::Top) => {
                Direction::Vertical
            }
            (ODirection::Top, ODirection::Left) | (ODirection::Left, ODirection::Top) => {
                Direction::TopLeft
            }
            (ODirection::Top, ODirection::Right) | (ODirection::Right, ODirection::Top) => {
                Direction::TopRight
            }
            (ODirection::Bottom, ODirection::Left) | (ODirection::Left, ODirection::Bottom) => {
                Direction::BottomLeft
            }
            (ODirection::Bottom, ODirection::Right) | (ODirection::Right, ODirection::Bottom) => {
                Direction::BottomRight
            }
            _ => unreachable!(),
        }
    }
    fn get_start(&self) -> (usize, usize) {
        for (i, v) in self.0.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if *c == Direction::Start {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }
    fn parse(a: &str) -> Self {
        Self(
            a.lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '|' => Direction::Vertical,
                            '-' => Direction::Horizontal,
                            'L' => Direction::TopRight,
                            'J' => Direction::TopLeft,
                            '7' => Direction::BottomLeft,
                            'F' => Direction::BottomRight,
                            'S' => Direction::Start,
                            '.' => Direction::Ground,
                            _ => unreachable!(),
                        })
                        .collect()
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
        let input = Input::parse(".....\n.S-7.\n.|.|.\n.L-J.\n.....");
        assert_eq!(input.solve_one(), 4);
    }
    #[test]
    fn two() {
        let input = Input::parse("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........");
        assert_eq!(input.solve_two(), 4);
        let input = Input::parse(".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...");
        assert_eq!(input.solve_two(), 8);
        let input = Input::parse("FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L");
        assert_eq!(input.solve_two(), 10);
    }
}
