mod model;
use model::*;

mod algorithm;
use algorithm::{AlgoKind, ModelConfig, Path, bfs};

pub struct Config {
    algo: AlgoKind,
    initial_node: Cube2,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != CUBE2_STATE_LEN+2 {
            return Err(String::from("Please provide algorithm and 6 initial states."));
        }

        let algo;
        match args[1].as_str() {
            "BFS" => algo = AlgoKind::BFS,
            "IDA*" => algo = AlgoKind::IDAStar,
            _ => return Err(format!("No algorithm \"{}\".", &args[1])),
        }

        let mut initial_state = [0 as u8; CUBE2_STATE_LEN];
        for i in 0..CUBE2_STATE_LEN {
            initial_state[i] = args[i+2].parse::<u8>().unwrap();
        }

        Ok(Config { algo, initial_node: Cube2::new(initial_state) })
    }
}

pub fn run(config: Config) -> Result<Path, String> {
    let model_config = ModelConfig {
        initial_node: config.initial_node,
        target_node: Cube2::new(SOLVED_CUBE2_STATE),
        edge_num: CUBE2_TRANS_NUM,
    };
    
    match config.algo {
        AlgoKind::BFS => bfs::search(model_config,
            |edge_idx, opt_edge| {
                if let Some(bfs_edge) = opt_edge {
                    return edge_idx / 3 == bfs_edge.edge_idx / 3;
                }
                return false;
            }
        ),
        AlgoKind::IDAStar => Err(String::from("Algorithm not implemented yet.")),
    }
}
