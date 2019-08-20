use crate::absolute_moves::{AbsMove, MoveDirection, Side};
use std::fmt::{Display, Formatter, Result as FResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Finger {
    Thumb,
    Index,
    Ring,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Push,
    Pull,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FingerMove(pub Finger, pub Hand, pub Direction);

impl Display for FingerMove {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        let finger = match self.0 {
            Finger::Thumb => "t",
            Finger::Index => "s",
            // Finger::Middle => "m",
            Finger::Ring => "r",
            // Finger::Little => "l",
        };
        let finger_w_hand = if self.1 == Hand::Left {
            finger.to_uppercase()
        } else {
            finger.to_string()
        };

        write!(f, "{}", finger_w_hand)?;
        if self.2 == Direction::Push {
            write!(f, "\'")?;
        }
        FResult::Ok(())
    }
}

impl FingerMove {
    pub fn into_absolute(self, hgrip: HandGrip) -> AbsMove {
        // Start with the move where grip = G2
        use crate::absolute_moves::MoveDirection::*;
        use crate::absolute_moves::Side::*;
        let move_g2 = match (self.0, self.1, self.2) {
            (Finger::Thumb, Hand::Left, Direction::Push) => AbsMove(F, CW),
            (Finger::Index, Hand::Left, Direction::Push) => AbsMove(U, CW),
            (Finger::Ring, Hand::Left, Direction::Push) => AbsMove(D, CCW),
            (Finger::Thumb, Hand::Left, Direction::Pull) => AbsMove(F, CCW),
            (Finger::Index, Hand::Left, Direction::Pull) => AbsMove(U, CCW),
            (Finger::Ring, Hand::Left, Direction::Pull) => AbsMove(D, CW),
            (Finger::Thumb, Hand::Right, Direction::Push) => AbsMove(F, CCW),
            (Finger::Index, Hand::Right, Direction::Push) => AbsMove(U, CCW),
            (Finger::Ring, Hand::Right, Direction::Push) => AbsMove(D, CW),
            (Finger::Thumb, Hand::Right, Direction::Pull) => AbsMove(F, CW),
            (Finger::Index, Hand::Right, Direction::Pull) => AbsMove(U, CW),
            (Finger::Ring, Hand::Right, Direction::Pull) => AbsMove(D, CCW),
        };

        let grip = if self.1 == Hand::Left {
            hgrip.0
        } else {
            hgrip.1
        };;

        use crate::rotation::FullCubeRotation;

        let rot = match grip {
            Grip::G0 => FullCubeRotation::X2,
            Grip::G1 => FullCubeRotation::X,
            Grip::G2 => FullCubeRotation::Ident,
            Grip::G3 => FullCubeRotation::Xi,
        };

        AbsMove(rot.apply_to_side(move_g2.0), move_g2.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Grip {
    G0 = 0,
    G1 = 1,
    G2 = 2,
    G3 = 3,
}

impl Grip {
    #[inline]
    fn increase(self) -> Option<Grip> {
        match self {
            Grip::G0 => Some(Grip::G1),
            Grip::G1 => Some(Grip::G2),
            Grip::G2 => Some(Grip::G3),
            Grip::G3 => None,
        }
    }

    #[inline]
    fn decrease(self) -> Option<Grip> {
        match self {
            Grip::G0 => None,
            Grip::G1 => Some(Grip::G0),
            Grip::G2 => Some(Grip::G1),
            Grip::G3 => Some(Grip::G2),
        }
    }

    fn inc2(self) -> Option<Grip> {
        self.increase().and_then(Grip::increase)
    }

    fn dec2(self) -> Option<Grip> {
        self.decrease().and_then(Grip::decrease)
    }

    fn changeby(self, change: VMove) -> Option<Grip> {
        match change {
            VMove::Plus1 => self.increase(),
            VMove::Plus2 => self.inc2(),
            VMove::Minus1 => self.decrease(),
            VMove::Minus2 => self.dec2(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HandGrip(pub Grip, pub Grip);

impl HandGrip {
    fn change_by(self, change: HandMove) -> Option<HandGrip> {
        if change.1 == Hand::Left {
            self.0.changeby(change.0).map(|x| HandGrip(x, self.1))
        } else {
            self.1.changeby(change.0).map(|x| HandGrip(self.0, x))
        }
    }
}

impl Display for HandGrip {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{}", self.0 as u8)?;
        write!(f, "/")?;
        write!(f, "{}", self.1 as u8)
    }
}

/// A vertical move, independent of hand
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VMove {
    Plus1,
    Minus1,
    Plus2,
    Minus2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HandMove(pub VMove, pub Hand);

impl Display for HandMove {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        let h = match self.1 {
            Hand::Left => "H",
            Hand::Right => "h",
        };
        let dir = match self.0 {
            VMove::Plus1 => "",
            VMove::Plus2 => "2",
            VMove::Minus1 => "'",
            VMove::Minus2 => "2'",
        };
        write!(f, "{}{}", h, dir)
    }
}

impl HandMove {
    fn into_absolute(self) -> AbsMove {
        use crate::absolute_moves::MoveDirection::*;
        use crate::absolute_moves::Side::*;

        match (self.0, self.1) {
            (VMove::Plus1, Hand::Left) => AbsMove(L, CCW),
            (VMove::Minus1, Hand::Left) => AbsMove(L, CW),
            (VMove::Plus2, Hand::Left) | (VMove::Minus2, Hand::Left) => AbsMove(L, Double),
            (VMove::Plus1, Hand::Right) => AbsMove(R, CW),
            (VMove::Minus1, Hand::Right) => AbsMove(R, CCW),
            (VMove::Plus2, Hand::Right) | (VMove::Minus2, Hand::Right) => AbsMove(R, Double),
        }
    }
}

#[derive(Debug)]
pub enum Move {
    FingerMove(FingerMove),
    HandMove(HandMove),
    Regrip(HandGrip),
}

#[derive(Debug)]
pub struct Algorithm {
    pub init_grip: HandGrip,
    pub moves: Vec<Move>
}

impl Algorithm {
    pub fn into_absolute(self) -> Option<Vec<AbsMove>> {
        let mut res = Vec::new();

        // TODO: Set capacity?
        let mut current_grip = self.init_grip;

        for mov in self.moves {
            match mov {
                Move::FingerMove(fm) => {
                    res.push(fm.into_absolute(current_grip));
                }
                Move::HandMove(hm) => {
                    res.push(hm.into_absolute());
                    if let Some(ngrip) = current_grip.change_by(hm) {
                        current_grip = ngrip;
                    } else {
                        return None;
                    }
                }
                Move::Regrip(grip) => {
                    current_grip = grip;
                }
            }
        }
        Some(res)
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{}", self.init_grip)?;
        for mov in &self.moves {
            match mov {
                Move::FingerMove(fm) => write!(f, " {}", fm)?,
                Move::HandMove(hm) => write!(f, " {}", hm)?,
                Move::Regrip(rg) => write!(f, " {}", rg)?,
            }
        }
        FResult::Ok(())
    }
}
