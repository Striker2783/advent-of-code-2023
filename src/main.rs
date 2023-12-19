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
    let e = match num {
        1 => one::run(path),
        2 => two::run(path),
        3 => three::run(path),
        4 => four::run(path),
        5 => five::run(path),
        6 => six::run(path),
        7 => seven::run(path),
        8 => eight::run(path),
        9 => nine::run(path),
        10 => ten::run(path),
        11 => eleven::run(path),
        12 => twelve::run(path),
        13 => thirteen::run(path),
        14 => fourteen::run(path),
        15 => fifteen::run(path),
        16 => sixteen::run(path),
        17 => seventeen::run(path),
        18 => eighteen::run(path),
        19 => nineteen::run(path),
        _ => {
            eprintln!("Not a valid day");
            return;
        }
    };
    if let Err(e) = e {
        eprintln!("{e}");
    }
}
