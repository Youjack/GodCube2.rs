use std::vec;

use super::*;

pub struct DFSNode<Node> {
    pub parent_edge_idx: Option<usize>,
    pub node: Node,
}

pub fn search<Node, Prune, AStar>(model_config: ModelConfig<Node>, prune: Prune, astar: AStar) -> Result<Path, String>
where
    Node: DirectedGraph,
    Prune: Fn(/*edge_idx: */usize, /*nodes: */&Vec<DFSNode<Node>>) -> bool,
    AStar: Fn(/*node: */&Node) -> usize, // give a least estimate of #steps to `target_node`
{
    if Node::is_eq(&model_config.initial_node, &model_config.target_node) {
        return Ok(vec![]);
    }
    
    // the last element in `nodes` is the node in tree that "the pointer" is pointing to
    let mut nodes = vec![
        DFSNode {
            parent_edge_idx: None,
            node: model_config.initial_node,
        }
    ];

    let mut target_depth = {
        let least_steps = astar(&nodes.last().unwrap().node);
        if least_steps == 0 { 1 } else { least_steps }
    };
    while target_depth <= model_config.max_step {
        enum DFSState { Forward, BackFrom(/*last_edge_idx: */usize) }
        let mut dfs_state = DFSState::Forward;
        'DFS: loop {
            let mut edge_idx: usize;
            match dfs_state {
                DFSState::Forward => edge_idx = 0,
                DFSState::BackFrom(last_edge_idx) => edge_idx = last_edge_idx + 1,
            }
            // find a possible edge to `go_along` and keep `Forward`,
            // or no edge is possible so go back
            loop {
                if edge_idx < model_config.edge_num { // there are children not searched
                    if !prune(edge_idx, &nodes) {
                        // get the new node
                        nodes.push(
                            DFSNode {
                                parent_edge_idx: Some(edge_idx),
                                node: nodes.last().unwrap().node.go_along(edge_idx),
                            }
                        );
                        let new_node = &nodes.last().unwrap().node;

                        // A*, `is_eq`
                        if nodes.len() - 1 < target_depth { // not deep enough
                            // A* not prune
                            if nodes.len() - 1 + astar(new_node) <= target_depth {
                                dfs_state = DFSState::Forward;
                                break;
                            }
                        } else { // deep enough
                            // `is_eq` not "prune"
                            if Node::is_eq(new_node, &model_config.target_node) {
                                let mut path: Path = vec![];
                                for node in &nodes {
                                    if let Some(edge_idx) = node.parent_edge_idx {
                                        path.push(edge_idx);
                                    }
                                }
                                return Ok(path);
                            }
                        }
                        
                        // prune the new node
                        //   according to A*,
                        //   or deep enough but not `is_eq`
                        nodes.pop();
                    }
                    edge_idx += 1; // prune
                } else { // all children are searched
                    if let Some(parent_edge_idx) = nodes.last().unwrap().parent_edge_idx {
                        nodes.pop();
                        dfs_state = DFSState::BackFrom(parent_edge_idx);
                        break;
                    } else { // already back to `initial_node`
                        break 'DFS;
                    }
                }
            }
        }
        target_depth += 1; // deepen
    }
    Err(format!("Cannot find target node within {} steps.", model_config.max_step))
}
