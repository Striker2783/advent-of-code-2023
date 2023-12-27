use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
    ops::Range,
    path::Path,
};

pub fn run(p: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(p)?;
    let input = Input::parse(&a);
    println!("Star 1: {}", input.one());
    println!("Star 2: {}", input.two());
    Ok(())
}
fn split_range(a: Range<u32>, b: Range<u32>) -> (Range<u32>, Range<u32>, Range<u32>) {
    let sect = (a.start.max(b.start))..(a.end.min(b.end));
    let left = (a.start)..sect.start;
    let right = sect.start..b.start;
    (left, sect, right)
}
#[derive(Debug)]
struct Input {
    conditions: HashMap<String, Condition>,
    parts: Vec<Part>,
}
impl Input {
    fn one(&self) -> u64 {
        let start = self.conditions.get("in").unwrap();
        let mut sum = 0;
        for part in &self.parts {
            let mut curr = start;
            while curr.name != "A" || curr.name != "R" {
                let cond = curr.p1(part);
                if cond == "A" {
                    sum += part.get_sum() as u64;
                    break;
                } else if cond == "R" {
                    break;
                }
                curr = self.conditions.get(&cond).unwrap();
            }
        }
        sum
    }
    fn two(&self) -> u64 {
        let mut queue = VecDeque::new();
        queue.push_back(("in", [1..4000, 1..4000, 1..4000, 1..4000]));
        let mut sum = 0;
        'outer: while let Some((name, ranges)) = queue.pop_front() {
            if name == "A" {
                sum += ranges
                    .iter()
                    .map(|r| (r.end - r.start + 1) as u64)
                    .product::<u64>();
                continue;
            } else if name == "R" {
                continue;
            }
            let condition = self.conditions.get(name).unwrap();
            for (c, n, o, t) in &condition.l {
                let mut clone = ranges.clone();
                let r = match *c {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                if (*o == Ordering::Less && ranges[r].start >= *n)
                    || (*o == Ordering::Greater && ranges[r].end <= *n)
                {
                    continue;
                } else if (*o == Ordering::Less && ranges[r].end < *n)
                    || (*o == Ordering::Greater && ranges[r].start > *n)
                {
                    queue.push_back((t.as_str(), clone));
                    continue 'outer;
                } else if *o == Ordering::Less {
                    clone[r] = ranges[r].start..(*n - 1);
                    queue.push_back((t.as_str(), clone.clone()));
                    clone[r] = *n..ranges[r].end;
                    queue.push_back((name, clone));
                    continue 'outer;
                } else {
                    clone[r] = (*n + 1)..ranges[r].end;
                    queue.push_back((t.as_str(), clone.clone()));
                    clone[r] = ranges[r].start..*n;
                    queue.push_back((name, clone));
                    continue 'outer;
                }
            }
            queue.push_back((condition.end.as_str(), ranges));
        }
        sum
    }
    fn parse(a: &str) -> Self {
        let mut split = a.split("\n\n");
        Self {
            conditions: split
                .next()
                .unwrap()
                .lines()
                .map(Condition::from)
                .map(|c| (c.name.clone(), c))
                .collect(),
            parts: split.next().unwrap().lines().map(Part::from).collect(),
        }
    }
}
#[derive(Debug, Default)]
struct Condition {
    name: String,
    l: Vec<(char, u32, Ordering, String)>,
    end: String,
}
impl Condition {
    fn p1(&self, p: &Part) -> String {
        for ((_, n, order, to), on) in self.l.iter().map(|l| match l.0 {
            'x' => (l, p.x),
            'm' => (l, p.m),
            'a' => (l, p.a),
            's' => (l, p.s),
            _ => unreachable!(),
        }) {
            if *order == Ordering::Greater {
                if on > *n {
                    return to.to_string();
                }
            } else if on < *n {
                return to.to_string();
            }
        }
        self.end.clone()
    }
}
impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let mut new = Self::default();
        let mut split = value.split('{');
        new.name = split.next().unwrap().to_string();
        let conditions = split.next().unwrap();
        let conditions = &conditions[..(conditions.len() - 1)];
        for condition in conditions.split(',') {
            if !condition.contains(':') {
                new.end = condition.to_string();
                continue;
            }
            let mut split = condition.split(':');
            let condition = split.next().unwrap();
            let destination = split.next().unwrap();
            let mut split;
            let ordering = if condition.contains('<') {
                split = condition.split('<');
                Ordering::Less
            } else {
                split = condition.split('>');
                Ordering::Greater
            };
            let category = split.next().unwrap();
            let num = split.next().unwrap().parse().unwrap();
            match category {
                "x" => new.l.push(('x', num, ordering, destination.to_string())),
                "m" => new.l.push(('m', num, ordering, destination.to_string())),
                "a" => new.l.push(('a', num, ordering, destination.to_string())),
                "s" => new.l.push(('s', num, ordering, destination.to_string())),
                _ => unreachable!(),
            }
        }
        new
    }
}
#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn get_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = &value[1..(value.len() - 1)];
        let mut split = value.split(',');
        let mut closure = || {
            split
                .next()
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap()
        };
        Self {
            x: closure(),
            m: closure(),
            a: closure(),
            s: closure(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn one() {
        let input = Input::parse(
            "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}",
        );
        // println!("{input:?}");
        assert_eq!(input.one(), 19114);
    }
    #[test]
    fn two() {
        let input = Input::parse(
            "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}",
        );
        // println!("{input:?}");
        assert_eq!(input.two(), 167409079868000);
    }
}
