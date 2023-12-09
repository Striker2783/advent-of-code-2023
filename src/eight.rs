use std::{collections::HashMap, error::Error, fs, path::Path};

pub fn run(file: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let input = Input::parse(&content);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
#[derive(Debug)]
enum Direction {
    Right,
    Left,
}
#[derive(Debug, Default)]
struct Input<'a> {
    instructions: Vec<Direction>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}
impl<'a> Input<'a> {
    fn new(instructions: Vec<Direction>, map: HashMap<&'a str, (&'a str, &'a str)>) -> Self {
        Self { instructions, map }
    }
    fn solve_one(&self) -> u32 {
        let mut curr = "AAA";
        self.instructions
            .iter()
            .cycle()
            .take_while(|&instruction| {
                if curr == "ZZZ" {
                    return false;
                }
                let (left, right) = self.map.get(curr).unwrap();
                curr = match instruction {
                    Direction::Right => right,
                    Direction::Left => left,
                };
                true
            })
            .count() as u32
    }
    fn solve_two(&self) -> u64 {
        let a = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .copied()
            .collect::<Vec<_>>();
        let multipliers: Vec<_> = a
            .iter()
            .map(|k| {
                let mut curr = *k;
                let mut stuff = 0;
                let mut hit = false;
                for d in self.instructions.iter().cycle() {
                    if curr.ends_with('Z') {
                        if hit {
                            break;
                        }
                        hit = true;
                    }
                    let (left, right) = self.map.get(curr).unwrap();
                    curr = match d {
                        Direction::Right => right,
                        Direction::Left => left,
                    };
                    if hit {
                        stuff += 1;
                    }
                }
                stuff
            })
            .collect();
        multipliers
            .iter()
            .copied()
            .fold(*multipliers.first().unwrap(), |acc, curr| {
                num::integer::lcm(acc, curr)
            })
    }
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect();
        lines.next();
        let map = lines
            .filter(|x| !x.is_empty())
            .map(|line| {
                let mut split = line.split(" = ");
                let source = split.next().unwrap();
                let mut dest = split.next().unwrap().split(", ");
                let left = &dest.next().unwrap()[1..];
                let right = &dest.next().unwrap()[..3];
                (source, (left, right))
            })
            .collect();
        Self::new(instructions, map)
    }
}
#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn solve_one() {
        let input = Input::parse("RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)");
        assert_eq!(input.solve_one(), 2);
        let input = Input::parse("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)");
        assert_eq!(input.solve_one(), 6);
    }
    #[test]
    fn solve_two() {
        let input = Input::parse("LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)");
        assert_eq!(input.solve_two(), 6);
    }
}
