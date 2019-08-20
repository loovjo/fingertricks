mod absolute_moves;
mod moves;
mod rotation;

use crate::moves::*;

fn main() {
    let grip = HandGrip(Grip::G1, Grip::G2);
    let m = FingerMove(Finger::Index, Hand::Left, Direction::Push);
    println!(
        "Move {:?} = {} -> {} @ {}",
        m,
        m,
        m.into_absolute(grip),
        grip
    );

    let m = FingerMove(Finger::Ring, Hand::Right, Direction::Pull);
    println!(
        "Move {:?} = {} -> {} @ {}",
        m,
        m,
        m.into_absolute(grip),
        grip
    );

    // T-perm: 2/2 h S' h' S h' s h2 S h' S h S' h' t'
    let moves = vec![
        Move::HandMove(HandMove(VMove::Plus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Left, Direction::Push)),
        Move::HandMove(HandMove(VMove::Minus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Left, Direction::Pull)),
        Move::HandMove(HandMove(VMove::Minus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Right, Direction::Pull)),
        Move::HandMove(HandMove(VMove::Plus2, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Left, Direction::Pull)),
        Move::HandMove(HandMove(VMove::Minus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Left, Direction::Pull)),
        Move::HandMove(HandMove(VMove::Plus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Index, Hand::Left, Direction::Push)),
        Move::HandMove(HandMove(VMove::Minus1, Hand::Right)),
        Move::FingerMove(FingerMove(Finger::Thumb, Hand::Right, Direction::Push)),
    ];
    let alg = Algorithm {
        init_grip: HandGrip(Grip::G2, Grip::G2),
        moves,
    };

    println!("Alg: {:}", alg);
    println!(
        " -> {}",
        alg.into_absolute()
            .unwrap()
            .into_iter()
            .map(|x| format!("{} ", x))
            .collect::<String>()
    )
}
