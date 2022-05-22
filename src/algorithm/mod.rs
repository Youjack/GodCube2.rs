pub mod bfs;

pub const MAX_DEPTH: u8 = 11;

pub enum AlgoKind {
    BFS,
    IDAStar,
}

pub struct ModelConfig<Node> {
    pub initial_node: Node,
    pub target_node: Node,
    pub edge_num: usize,
}

pub trait DirectedGraph {
    /// go along edge with index `edge_idx` and return the new node
    fn go_along(&self, edge_idx: usize) -> Self;

    fn is_eq(node1: &Self, node2: &Self) -> bool;

    fn edge_idx_invert(edge_idx: usize) -> usize;
}

/// path containing the index of edges
pub type Path = Vec<usize>;
