use super::*;

const CUBE2_MAX_STATE: usize = 21;

static mut ASTAR_DATA: Vec<u8> = vec![];

/// convert Cube2 to a 1-D representation
fn convert(cube2: &Cube2) -> usize {
    let mut repre:usize = 0;
    let mut base:usize = 1;
    for i in 0..CUBE2_STATE_LEN {
        repre += (cube2.state[i] as usize) * base;
        base *= CUBE2_MAX_STATE;
    }
    return repre;
}

pub fn initialize() {
    let astar_data: &mut [u8];
    unsafe {
        ASTAR_DATA = vec![(GOD_NUMBER2 + 1) as u8; 21*21*21*21*21*21];
        // GOD_NUMBER2+1 represents a state unreachable or not reached yet
        astar_data = &mut ASTAR_DATA[..];
    }

    let mut curt_level = vec![Cube2::new(SOLVED_CUBE2_STATE)];
    astar_data[convert(&curt_level[0])];
    for step in 1..(GOD_NUMBER2 + 1) as u8 {
        let mut next_level: Vec<Cube2> = vec![];
        for node in &curt_level {
            for edge_idx in 0..CUBE2_TRANS_NUM {
                next_level.push(node.go_along(edge_idx));
                let repre = convert(&next_level.last().unwrap());
                if astar_data[repre] > GOD_NUMBER2 as u8 { // not searched
                    astar_data[repre] = step;
                } else { // searched
                    next_level.pop();
                }
            }
        }
        curt_level = next_level;
    }
}

/// give a least estimate of #steps to `SOLVED_CUBE2`
pub fn estimate(node: &Cube2) -> usize {
    unsafe { ASTAR_DATA[convert(node)] as usize }
}