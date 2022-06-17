use super::algorithm::DirectedGraph;
use std::mem::MaybeUninit;

pub mod astar;

pub const GOD_NUMBER2: usize = 11;

// "trans" for transformation
pub const CUBE2_TRANS_NUM: usize = 9;
const CUBE2_TRANS_INVERSION: [usize; CUBE2_TRANS_NUM] = [0,2,1,3,5,4,6,8,7];

pub const CUBE2_STATE_LEN: usize = 6;
pub type Cube2State = [u8; CUBE2_STATE_LEN];
/// Cube2 will be moved and cannot be copied!
pub struct Cube2 { state: Cube2State }
pub const CUBE2_MAX_STATE: usize = 21;
pub const SOLVED_CUBE2_STATE: Cube2State = [0,3,6,9,12,15];
const CUBE2_TRANS_RULE: [[u8;CUBE2_TRANS_NUM];CUBE2_MAX_STATE] = [
    [ 12,  3, 10, 15,  6,  4, 18,  9,  7 ],
    [ 13,  4, 11, 16,  7,  5, 19, 10,  8 ],
    [ 14,  5,  9, 17,  8,  3, 20, 11,  6 ],
    [ 10, 12,  0,  8,  2, 17,  3,  3,  3 ],
    [ 11, 13,  1,  6,  0, 15,  4,  4,  4 ],
    [  9, 14,  2,  7,  1, 16,  5,  5,  5 ],
    [  6,  6,  6,  4, 15,  0, 11,  2, 20 ],
    [  7,  7,  7,  5, 16,  1,  9,  0, 18 ],
    [  8,  8,  8,  3, 17,  2, 10,  1, 19 ],
    [  5,  2, 14,  9,  9,  9,  7, 18,  0 ],
    [  3,  0, 12, 10, 10, 10,  8, 19,  1 ],
    [  4,  1, 13, 11, 11, 11,  6, 20,  2 ],
    [  0, 10,  3, 12, 12, 12, 12, 12, 12 ],
    [  1, 11,  4, 13, 13, 13, 13, 13, 13 ],
    [  2,  9,  5, 14, 14, 14, 14, 14, 14 ],
    [ 15, 15, 15,  0,  4,  6, 15, 15, 15 ],
    [ 16, 16, 16,  1,  5,  7, 16, 16, 16 ],
    [ 17, 17, 17,  2,  3,  8, 17, 17, 17 ],
    [ 18, 18, 18, 18, 18, 18,  0,  7,  9 ],
    [ 19, 19, 19, 19, 19, 19,  1,  8, 10 ],
    [ 20, 20, 20, 20, 20, 20,  2,  6, 11 ]
];

impl Cube2 {
    pub fn new(state: Cube2State) -> Cube2 {
        Cube2 { state }
    }

    pub fn _print(&self) {
        print!("{}", "[ ");
        for s in self.state {
            print!("{} ", s);
        }
        println!("{}", "]");
    }

    /// transform the state and store it in the original instance
    pub fn _trans(&mut self, trans: usize) {
        for i in 0..CUBE2_STATE_LEN {
            self.state[i] = CUBE2_TRANS_RULE[self.state[i] as usize][trans];
        }
    }
}
impl DirectedGraph for Cube2 {
    fn go_along(&self, trans: usize) -> Cube2 {
        let mut cube2: Cube2 = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..CUBE2_STATE_LEN {
            cube2.state[i] = CUBE2_TRANS_RULE[self.state[i] as usize][trans];
        }
        cube2
    }

    fn is_eq(cube2_1: &Cube2, cube2_2: &Cube2) -> bool {
        cube2_1.state == cube2_2.state
    }

    fn edge_idx_invert(trans: usize) -> usize {
        CUBE2_TRANS_INVERSION[trans]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn santa_claus() {
        let mut cube2 = Cube2::new([0,3,6,9,12,15]);
        let path = [1,5,2,6,0,3,8];
        for trans in path {
            cube2._trans(trans);
        }
        cube2._print();
    }
}
