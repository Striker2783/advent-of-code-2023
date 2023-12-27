use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
    path::Path,
};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let a = fs::read_to_string(path)?;
    Ok(())
}
#[derive(Debug, Default, Clone)]
struct Input(HashMap<String, Module>);
impl Input {
    fn solve_one(&self) -> u64 {
        let mut low = 0;
        let mut high = 0;
        let mut new = Self(self.0.clone());
        // for _ in 0..1000 {
        let (l, h) = new.press_button();
        low += l;
        high += h;
        // }
        low * high
    }
    fn press_button(&mut self) -> (u64, u64) {
        let (mut low, mut high) = (1, 0);
        let mut queue = VecDeque::new();
        queue.push_back((Pulse::Low, "broadcaster".to_string()));
        while let Some((pulse, name)) = queue.pop_front() {
            let a = match self.0.get_mut(&name) {
                Some(a) => a,
                None => continue,
            };
            let pulse = a.get_pulse(pulse);
            if let Some(pulse) = pulse {
                if pulse == Pulse::High {
                    high += a.dest.len() as u64;
                } else {
                    low += a.dest.len() as u64;
                }
                for dest in a.dest.clone() {
                    println!("{} -{pulse:?}-> {dest}", a.name);
                    queue.push_back((pulse, dest));
                }
            }
        }
        (low, high)
    }
}
impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        for line in value.lines() {
            let m = Module::from(line);
            map.insert(m.name.clone(), m);
        }
        Self(map)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}
#[derive(Debug, Clone)]
enum Type {
    FlipFlop(bool),
    Conjunction(Pulse),
    Broadcast,
}
#[derive(Debug, Clone)]
struct Module {
    name: String,
    dest: Vec<String>,
    t: Type,
}
impl Module {
    fn get_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match self.t {
            Type::FlipFlop(on) => {
                if pulse == Pulse::High {
                    None
                } else {
                    self.t = Type::FlipFlop(!on);
                    if !on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
            }
            Type::Conjunction(prev) => {
                if prev == Pulse::High && pulse == Pulse::High {
                    self.t = Type::Conjunction(pulse);
                    Some(Pulse::Low)
                } else {
                    self.t = Type::Conjunction(pulse);
                    Some(Pulse::High)
                }
            }
            Type::Broadcast => Some(pulse),
        }
    }
}
impl Default for Module {
    fn default() -> Self {
        Self {
            name: Default::default(),
            dest: Default::default(),
            t: Type::Broadcast,
        }
    }
}
impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let mut new = Self::default();
        let mut split = value.split(" -> ");
        let name = split.next().unwrap();
        let char = name.chars().nth(0).unwrap();
        if char == '%' {
            new.t = Type::FlipFlop(false);
            new.name = name[1..].to_string();
        } else if char == '&' {
            new.t = Type::Conjunction(Pulse::Low);
            new.name = name[1..].to_string();
        } else {
            new.t = Type::Broadcast;
            new.name = name.to_string();
        }
        let dests = split.next().unwrap();
        for dest in dests.split(", ") {
            new.dest.push(dest.to_string());
        }
        new
    }
}
#[cfg(test)]
mod tests {
    use crate::twenty::Input;

    #[test]
    fn one() {
        let input = Input::from("broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a");
        assert_eq!(input.solve_one(), 32000000);
        // let input =
        //     Input::from("broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output");
        // assert_eq!(input.solve_one(), 11687500);
    }
}
