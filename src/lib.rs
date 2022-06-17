/* #![feature(test)]
extern crate test; */

mod model;
use model::*;
pub use model::{Cube2, CUBE2_STATE_LEN, CUBE2_MAX_STATE, astar};

mod algorithm;
use algorithm::*;
pub use algorithm::AlgoKind;

#[cfg(target_arch = "wasm32")]
mod web;

pub struct Config {
    pub algo: AlgoKind,
    pub initial_node: Cube2,
}

pub fn search(config: Config) -> Result<Path, String> {
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
