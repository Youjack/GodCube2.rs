use std::{io, env, process};
use std::time::Instant;

use god_cube2::*;

fn parse_command(args: &[String]) -> Result<Config, String> {
    if args.len() != CUBE2_STATE_LEN+2 {
        return Err(format!("Please provide algorithm and 6 initial states."));
    }

    let algo;
    match args[1].as_str() {
        "BFS" => algo = AlgoKind::BFS,
        "IDA*" => {
            algo = AlgoKind::IDAStar;
            print!("Initializing A* for Cube2... ");
            io::Write::flush(&mut io::stdout()).ok();
            astar::initialize();
            println!("DONE!");
        },
        _ => return Err(format!("No algorithm \"{}\".", &args[1])),
    }

    let mut initial_state = [0 as u8; CUBE2_STATE_LEN];
    for i in 0..CUBE2_STATE_LEN {
        let state = args[i+2].parse::<i32>().unwrap();
        if 0 <= state && state < CUBE2_MAX_STATE as i32 {
            initial_state[i] = state as u8;
        } else {
            return Err(format!("State \"{}\" does not exist.", state));
        }
    }

    Ok(Config { algo, initial_node: Cube2::new(initial_state) })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_command(&args).unwrap_or_else(
        |err| {
            println!("Arguments error: {}", err);
            process::exit(1);
        }
    );

    let start_instant = Instant::now();
    let sol = god_cube2::search(config).unwrap_or_else(
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
