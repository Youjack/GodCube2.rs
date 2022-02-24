use std::{env, process};
use std::time::Instant;

use god_cube2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("Arguments error: {}", err);
            process::exit(1);
        }
    );

    let start_instant = Instant::now();
    let sol = god_cube2::run(config).unwrap_or_else(
        |err| {
            println!("Algorithm error: {}", err);
            process::exit(1);
        }
    );
    let finish_instant = Instant::now();

    println!("time: {} ms", finish_instant.duration_since(start_instant).as_millis());
    println!("#steps: {}", sol.len());
    print!("solution: ");
    for trans in sol.iter() {
        print!("{} ", *trans);
    }
    println!();
}
