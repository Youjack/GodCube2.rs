use std::{fs, io::{self, Read, Write}};

use super::*;

const CUBE2_MAX_STATE: usize = 21;

static mut ASTAR_DATA: Vec<u8> = vec![];
const ASTAR_DATA_LEN: usize = 21*21*21*21*21*21;
const ASTAR_DATA_FILENAME: &str = "cube2.astar_data";

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

fn generate_astar_data() {
    let astar_data: &mut [u8];
    unsafe {
        // GOD_NUMBER2+1 represents a state unreachable or not reached yet
        ASTAR_DATA = vec![(GOD_NUMBER2 + 1) as u8; ASTAR_DATA_LEN];
        astar_data = &mut ASTAR_DATA;
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

fn read_astar_data() -> Result<(), io::Error> {
    let mut file = fs::File::open(ASTAR_DATA_FILENAME)?;
    unsafe {
        ASTAR_DATA = vec![0u8; ASTAR_DATA_LEN];
        file.read(&mut ASTAR_DATA)?;
    }
    Ok(())
}
fn save_astar_data() {
    let mut file = fs::File::create(ASTAR_DATA_FILENAME).unwrap();
    unsafe { file.write(&ASTAR_DATA).unwrap(); }
}

pub fn initialize() {
    if read_astar_data().is_ok() {
        return;
    } else {
        generate_astar_data();
        save_astar_data();
    }
}

/// give a least estimate of #steps to `SOLVED_CUBE2`
pub fn estimate(node: &Cube2) -> usize {
    unsafe { ASTAR_DATA[convert(node)] as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let repre = convert(&Cube2::new([0,3,6,9,15,12]));

        generate_astar_data();
        unsafe { println!("{:?}", &ASTAR_DATA[repre-5..repre+5]); }

        save_astar_data();
        read_astar_data().unwrap();
        unsafe { println!("{:?}", &ASTAR_DATA[repre-5..repre+5]); }
    }
}
