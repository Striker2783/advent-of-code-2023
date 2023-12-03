use std::fs;

pub fn run() {
    let content = match fs::read_to_string("files/2.txt") {
        Ok(a) => a,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let games = Games::parse(&content);
    println!("Star 1: {}", games.solve_one());
    println!("Star 2: {}", games.solve_two());
}
#[derive(Debug, Default)]
struct Games {
    games: Vec<Game>,
}
impl Games {
    fn parse(content: &str) -> Self {
        let mut new = Self::default();
        for line in content.lines() {
            let value = Game::parse_line(line);
            // println!("{line}");
            // println!("{value:?}");
            new.games.push(value);
        }
        new
    }
    fn solve_one(&self) -> u32 {
        self.games
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v.is_valid() {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
            .sum()
    }
    fn solve_two(&self) -> u32 {
        self.games.iter().map(Game::get_power_sum).sum()
    }
}
#[derive(Default, PartialEq, Eq, Debug)]
struct Game {
    matches: Vec<Match>,
}
impl Game {
    fn parse_line(line: &str) -> Self {
        let mut new = Self::default();
        let mut seperator = line.split(": ");
        seperator.next();
        let line = seperator.next().unwrap();
        for text in line.split("; ") {
            new.matches.push(Match::parse(text));
        }
        new
    }
    fn is_valid(&self) -> bool {
        for m in &self.matches {
            if !m.is_valid() {
                return false;
            }
        }
        true
    }
    fn get_power_sum(&self) -> u32 {
        let mut maxs = Match::new(0, 0, 0);
        for m in &self.matches {
            maxs.blue = maxs.blue.max(m.blue);
            maxs.red = maxs.red.max(m.red);
            maxs.green = maxs.green.max(m.green);
        }
        maxs.blue * maxs.green * maxs.red
    }
}
#[derive(Debug, PartialEq, Eq, Default)]
struct Match {
    red: u32,
    green: u32,
    blue: u32,
}
impl Match {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
    fn parse(text: &str) -> Self {
        let mut new = Self::default();
        for color in text.split(", ") {
            let mut space = color.split(' ');
            let number: u32 = space.next().unwrap().parse().unwrap();
            let color = space.next().unwrap();
            match color {
                "green" => new.green = number,
                "blue" => new.blue = number,
                "red" => new.red = number,
                _ => unreachable!(),
            }
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use crate::two::Games;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Games::parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
            .solve_one(),
            8
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Games::parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
            .solve_two(),
            2286
        );
    }
}
