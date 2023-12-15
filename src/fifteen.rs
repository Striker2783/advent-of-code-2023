use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let c = fs::read_to_string(path)?;
    let input = Input::parse(&c);
    println!("Star 1: {}", input.solve_one());
    println!("Star 2: {}", input.solve_two());
    Ok(())
}
struct Input(Vec<String>);
impl Input {
    fn parse(i: &str) -> Self {
        Self(i.split(',').map(|s| s.to_string()).collect())
    }
    fn solve_one(&self) -> u64 {
        self.0.iter().map(|h| Self::hash(h)).sum()
    }
    fn solve_two(&self) -> u64 {
        let mut boxes = vec![vec![]; 256];
        for s in self.0.iter() {
            let s = s.as_str();
            let label = s.split(|c| c == '=' || c == '-').nth(0).unwrap();
            let hash = Self::hash(label) as usize;
            if s.contains('=') {
                let lens: u64 = s.split('=').nth(1).unwrap().parse().unwrap();
                if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
                    boxes[hash][i] = (label, lens);
                } else {
                    boxes[hash].push((label, lens));
                }
            } else if let Some(i) = boxes[hash].iter().position(|(x, _)| *x == label) {
                boxes[hash].remove(i);
            }
        }
        boxes
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let i = i + 1;
                v.iter()
                    .enumerate()
                    .map(|(j, (_, lens))| i * (j + 1) * (*lens as usize))
                    .sum::<usize>() as u64
            })
            .sum()
    }
    fn hash(a: &str) -> u64 {
        let mut curr = 0;
        for c in a.chars() {
            curr += c as u64;
            curr *= 17;
            curr %= 256;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_one() {
        let input = Input::parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(input.solve_one(), 1320);
    }
    #[test]
    fn test_two() {
        let input = Input::parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(input.solve_two(), 145);
    }
}
