use std::rc::Rc;

use super::*;

pub type OptEdge = Option<Rc<BFSEdge>>;
pub struct BFSEdge {
    pub parent_edge: OptEdge,
    /// indexing an edge in `edge_list`
    pub edge_idx: usize,
}
/// BFSNode should be allocated on heap.
/// 
/// The current level of nodes own their parent edges
/// which (as child edges) own their parent edges.
struct BFSNode<Node> {
    parent_edge: OptEdge,
    node: Node,
}

/// bidirectional breath-first search with pruning
pub fn search<Node, Prune>(model_config: ModelConfig<Node>, prune: Prune) -> Result<Path, String>
where
    Node: DirectedGraph,
    Prune: Fn(/*edge_idx: */usize, /*opt_edge: */&OptEdge) -> bool,
{
    if Node::is_eq(&model_config.initial_node, &model_config.target_node) {
        return Ok(vec![]);
    }

    // current level of nodes (initially branched from initial node)
    let mut curt_level = vec![
        BFSNode {
            parent_edge: None,
            node: model_config.initial_node,
        }
    ];
    // current level of nodes in the opposite tree (initially branched from target node)
    let mut opp_level = vec![
        BFSNode {
            parent_edge: None,
            node: model_config.target_node,
        }
    ];

    let mut depth: usize = 0;
    while depth < model_config.max_step {
        let mut next_level: Vec<BFSNode<Node>> = vec![];
        for bfs_node in curt_level.iter() {
            for edge_idx in 0..model_config.edge_num {
                // prune
                if prune(edge_idx, &bfs_node.parent_edge) { continue; }
                
                // get the new node
                next_level.push(
                    BFSNode {
                        parent_edge: Some(Rc::new(
                            BFSEdge {
                                parent_edge: bfs_node.parent_edge.clone(),
                                edge_idx,
                            }
                        )),
                        node: bfs_node.node.go_along(edge_idx),
                    }
                );
                let new_bfs_node = next_level.last().unwrap();
                
                // compare the new node to nodes in the opposite level
                for opp_bfs_node in opp_level.iter() {
                    if Node::is_eq(&new_bfs_node.node, &opp_bfs_node.node) {
                        // trace back to get the path in a reversed order
                        macro_rules! get_path {
                            ($path:ident, $opt_edge_expr:expr) => {
                                let mut $path: Path = vec![];
                                let mut opt_edge = $opt_edge_expr;
                                while let Some(bfs_edge/*: Rc*/) = opt_edge {
                                    $path.push(bfs_edge.edge_idx);
                                    opt_edge = &bfs_edge.parent_edge;
                                }
                            }
                        }
                        get_path!(path, &new_bfs_node.parent_edge);
                        get_path!(opp_path, &opp_bfs_node.parent_edge);
                        // concatenate the path from initial node and the path from target node
                        macro_rules! cat_path {
                            ($path_init:ident, $path_tgt:ident) => {
                                $path_init.reverse();
                                for edge_idx in $path_tgt.iter_mut() {
                                    *edge_idx = Node::edge_idx_invert(*edge_idx);
                                }
                                $path_init.append(&mut $path_tgt);
                                return Ok($path_init);
                            };
                        }
                        if depth % 2 == 0 {
                            // current level is from initial node
                            cat_path!(path, opp_path);
                        } else { // current level is from target node
                            cat_path!(opp_path, path);
                        }
                    }
                }
            }
        }
        // swap pointers of levels
        curt_level = opp_level;
        opp_level = next_level;
        depth += 1;
    }
    Err(format!("Cannot find target node within {} steps.", model_config.max_step))
}
