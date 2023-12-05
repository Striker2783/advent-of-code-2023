use std::{fs, path::Path, str::Lines};

pub fn run(file: &Path) {
    let contents = match fs::read_to_string(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let maps = Maps::parse(&contents);
    println!("Star 1: {}", maps.solve_one());
}
#[derive(Default, Debug)]
struct Maps {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}
impl Maps {
    pub fn solve_one(&self) -> u64 {
        let mut destinations = self.seeds.clone();
        for v in &self.maps {
            let mut new_destinations = vec![];
            for dest in destinations {
                new_destinations.push(Self::get_destination(dest, v));
            }
            destinations = new_destinations;
        }
        *destinations.iter().min().unwrap()
    }
    fn get_destination(source: u64, vec: &[(u64, u64, u64)]) -> u64 {
        for stuff in vec {
            if !Self::in_range((stuff.1, stuff.2), source) {
                continue;
            }
            return source - stuff.1 + stuff.0;
        }
        source
    }
    pub fn solve_two(&self) -> u64 {
        let mut destinations = self.seeds.clone();
        for seed in self.seeds.chunks_exact(2) {
            let (seed1, seed2) = (seed[0], seed[1]);
        }
        for v in &self.maps {
            let mut new_destinations = vec![];
            for dest in destinations {
                new_destinations.push(Self::get_destination(dest, v));
            }
            destinations = new_destinations;
        }
        *destinations.iter().min().unwrap()
    }
    fn in_range(range: (u64, u64), num: u64) -> bool {
        range.0 <= num && num < range.0 + range.1
    }
    fn parse_line(&mut self, lines: &mut Lines) {
        self.maps.push(vec![]);
        let thing = self.maps.last_mut().unwrap();
        for line in lines {
            if line.is_empty() {
                return;
            }
            if line.contains(':') {
                continue;
            }
            let mut split = line.split(' ');
            thing.push((
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            ));
        }
    }
    pub fn parse(input: &str) -> Self {
        let mut this = Self::default();
        let mut lines = input.lines();
        let seeds = lines.next().unwrap().split(": ").nth(1).unwrap();
        for seed in seeds.split(' ') {
            if let Ok(n) = seed.parse::<u64>() {
                this.seeds.push(n);
            }
        }
        lines.next();
        for _ in 0..7 {
            this.parse_line(&mut lines);
        }
        this
    }
}

#[cfg(test)]
mod tests {
    use super::Maps;
    #[test]
    fn test_solve_one() {
        let map = Maps::parse(
            "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4",
        );
        assert_eq!(map.solve_one(), 35);
    }
}
