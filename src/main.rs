mod moves;
mod rotation;

use crate::moves::*;

fn main() {
    let m = FingerMove(Finger::Index, Hand::Left, Direction::Push);
    println!("Move {:?} = {}", m, m);

    let m = FingerMove(Finger::Little, Hand::Right, Direction::Pull);
    println!("Move {:?} = {}", m, m);
}
