/* #![feature(test)]
extern crate test; */

mod model;
use model::*;

mod algorithm;
use algorithm::*;

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
            "IDA*" => {
                algo = AlgoKind::IDAStar;
                print!("Initializing A* for Cube2... ");
                std::io::Write::flush(&mut std::io::stdout()).ok();
                astar::initialize();
                println!("DONE!");
            },
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
        max_step: GOD_NUMBER2,
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
        AlgoKind::IDAStar => id_astar::search(model_config,
            |edge_idx, nodes| {
                if let Some(parent_edge_idx) = nodes.last().unwrap().parent_edge_idx {
                    return edge_idx / 3 == parent_edge_idx / 3;
                }
                return false;
            },
            astar::estimate
        ),
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn santa_bench(b: &mut test::Bencher) {
        // astar::initialize();
        b.iter(|| {
            let config = Config {
                algo: AlgoKind::BFS,
                initial_node: Cube2::new([0,3,6,9,15,12]),
            };
            test::black_box(run(config))
        })
    }
} */
