use std::{fs, path::Path, vec};
pub fn run(file: &Path) {
    let contents = match fs::read_to_string(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let a = Records::parse(&contents);
    println!("Star 1: {}", a.solve_one());
    println!("Star 2: {}", a.solve_two());
}
#[derive(Debug, Default)]
struct Records {
    time: Vec<u64>,
    record: Vec<u64>,
}
impl Records {
    pub fn parse(input: &str) -> Self {
        let mut new = Self::default();
        let mut thing = input.lines().map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter_map(|a| a.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        });
        new.time = thing.next().unwrap();
        new.record = thing.next().unwrap();
        new
    }
    fn thing(t: u64, r: u64) -> u64 {
        (1..t)
            .filter(|a| {
                let new_time = t - a;
                new_time * a > r
            })
            .count() as u64
    }
    fn solve_one(&self) -> u64 {
        self.time
            .iter()
            .zip(self.record.iter())
            .map(|(t, r)| Self::thing(*t, *r))
            .product()
    }
    fn solve_two(&self) -> u64 {
        let t = self.time.iter().fold(0u64, |acc, &i| {
            acc * 10u64.pow(i.to_string().len() as u32) + i
        });
        let r = self.record.iter().fold(0u64, |acc, &i| {
            acc * 10u64.pow(i.to_string().len() as u32) + i
        });
        Self::thing(t, r)
    }
}

#[cfg(test)]
mod tests {
    use super::Records;

    #[test]
    fn solve_one() {
        let a = Records::parse("Time:      7  15   30\nDistance:  9  40  200");
        assert_eq!(a.solve_one(), 288)
    }
    #[test]
    fn solve_two() {
        let a = Records::parse("Time:      7  15   30\nDistance:  9  40  200");
        assert_eq!(a.solve_two(), 71503)
    }
}
