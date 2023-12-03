use std::path::Path;

use advent2023::*;
fn main() {
    let mut args = std::env::args();
    args.next();
    let num = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Input day number");
            return;
        }
    };
    let num = match num.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Input a number");
            return;
        }
    };
    let file = match args.next() {
        Some(f) => f,
        None => {
            eprintln!("Input a file");
            return;
        }
    };
    let path = Path::new(&file);
    match num {
        1 => one::run(path),
        2 => two::run(path),
        3 => three::run(path),
        _ => eprintln!("Not a valid day"),
    }
}
