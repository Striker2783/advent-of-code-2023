use std::{fs, path::Path};

pub fn run(file: &Path) {
    let contents = match fs::read_to_string(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
}
#[derive(Default)]
struct Maps {
    seeds: Vec<u32>,
    fertilizer: Vec<(u32, u32, u32)>,
    water: Vec<(u32, u32, u32)>,
    light: Vec<(u32, u32, u32)>,
    temperature: Vec<(u32, u32, u32)>,
    humidity: Vec<(u32, u32, u32)>,
    location: Vec<(u32, u32, u32)>,
}
impl Maps {
    pub fn parse(input: &str) -> Self {
        let mut this = Self::default();
        this
    }
}
