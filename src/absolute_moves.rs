use std::fmt::{Display, Formatter, Result as FResult};

#[allow(unused)]
pub const SIDES: &[Side] = &[Side::R, Side::L, Side::U, Side::D, Side::F, Side::B];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    R,
    L,
    U,
    D,
    F,
    B,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveDirection {
    CW, CCW, Double
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AbsMove(pub Side, pub MoveDirection);

impl Display for AbsMove {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{:?}", self.0)?;
        match self.1 {
            MoveDirection::CCW => write!(f, "'"),
            MoveDirection::Double => write!(f, "2"),
            MoveDirection::CW => FResult::Ok(()),
        }
    }
}
