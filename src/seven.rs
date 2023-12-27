use std::{error::Error, fs, path::Path};
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let match_list = MatchList::parse(&content);

    println!("Star 1: {}", match_list.solve_one());
    println!("Star 2: {}", match_list.solve_two());
    Ok(())
}
#[derive(Default, Debug)]
struct MatchList {
    hands: Vec<Hand>,
}
impl MatchList {
    fn new(hands: Vec<Hand>) -> Self {
        Self { hands }
    }
    fn solve_one(&self) -> u32 {
        let mut vec = self.hands.clone();
        vec.sort();
        vec.iter()
            .rev()
            .enumerate()
            .map(|(i, v)| v.bid * ((i + 1) as u32))
            .sum()
    }
    fn solve_two(&self) -> u32 {
        let mut vec = self.hands.clone();
        vec.iter_mut().for_each(|h| {
            for c in h.cards.iter_mut() {
                if *c == 11 {
                    *c = 0;
                }
            }
        });
        vec.sort_by(|a, b| a.cmp_two(b));
        vec.iter()
            .rev()
            .enumerate()
            .map(|(i, v)| v.bid * ((i + 1) as u32))
            .sum()
    }
    fn parse(str: &str) -> Self {
        Self::new(str.lines().map(Hand::parse).collect())
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    One,
    High,
}
#[derive(Default, Debug, Eq, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<u32>, bid: u32) -> Self {
        Self { cards, bid }
    }
    fn cmp_two(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type_two().cmp(&other.get_type_two()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => other.cards.cmp(&self.cards),
        }
    }
    fn get_type(&self) -> HandType {
        let mut sorted = self.cards.clone();
        sorted.sort();
        let iter = sorted.iter();
        let mut vec = vec![];
        let mut curr_matches = 0;
        let mut prev = u32::MAX;
        for &n in iter {
            if prev == n {
                curr_matches += 1;
                continue;
            }
            vec.push(curr_matches);
            curr_matches = 0;
            prev = n;
        }
        vec.push(curr_matches);
        let (max, o_max) = vec.iter().copied().fold((0, 0), |(a, b), n| {
            if n > a {
                if a > b {
                    (n, a)
                } else {
                    (n, b)
                }
            } else if n > b {
                (a, n)
            } else {
                (a, b)
            }
        });
        if max == 2 && o_max == 1 {
            return HandType::Full;
        }
        if max == 1 && o_max == 1 {
            return HandType::TwoPair;
        }
        match max {
            4 => HandType::Five,
            3 => HandType::Four,
            2 => HandType::Three,
            1 => HandType::One,
            0 => HandType::High,
            _ => unreachable!(),
        }
    }
    fn get_type_two(&self) -> HandType {
        let mut sorted = self.cards.clone();
        sorted.sort();
        let iter = sorted.iter();
        let mut vec = vec![];
        let mut jokers = 0;
        let mut curr_matches = 0;
        let mut prev = u32::MAX;
        for &n in iter {
            if n == 0 {
                jokers += 1;
                continue;
            }
            if prev == n {
                curr_matches += 1;
                continue;
            }
            vec.push(curr_matches);
            curr_matches = 0;
            prev = n;
        }
        vec.push(curr_matches);
        let (max, o_max) = vec.iter().copied().fold((0, 0), |(a, b), n| {
            if n > a {
                if a > b {
                    (n, a)
                } else {
                    (n, b)
                }
            } else if n > b {
                (a, n)
            } else {
                (a, b)
            }
        });
        let max = max + jokers;
        if max == 2 && o_max == 1 {
            return HandType::Full;
        }
        if max == 1 && o_max == 1 {
            return HandType::TwoPair;
        }
        match max {
            5 => HandType::Five,
            4 => HandType::Five,
            3 => HandType::Four,
            2 => HandType::Three,
            1 => HandType::One,
            0 => HandType::High,
            _ => unreachable!(),
        }
    }
    fn parse(line: &str) -> Self {
        let mut split = line.split(' ');
        let cards = split
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if let Some(n) = c.to_digit(10) {
                    return n;
                }
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        let bid: u32 = split.next().unwrap().parse().unwrap();
        Self::new(cards, bid)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => other.cards.cmp(&self.cards),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MatchList;

    #[test]
    fn test_one() {
        let a = MatchList::parse("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        assert_eq!(a.solve_one(), 6440);
    }
    #[test]
    fn test_two() {
        let a = MatchList::parse("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        assert_eq!(a.solve_two(), 5905);
    }
}
