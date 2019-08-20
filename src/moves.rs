use std::fmt::{Display, Formatter, Result as FResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
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
            Finger::Middle => "m",
            Finger::Ring => "r",
            Finger::Little => "l",
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
pub struct Regrip(pub Grip, pub Grip);

impl Regrip {
    fn changeby(self, change: HandMove) -> Option<Regrip> {
        if change.1 == Hand::Left {
            self.0.changeby(change.0).map(|x| Regrip(x, self.1))
        } else {
            self.1.changeby(change.0).map(|x| Regrip(self.0, x))
        }
    }
}

impl Display for Regrip {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{}", self.0 as u8)?;
        write!(f, "/")?;
        write!(f, "{}", self.0 as u8)
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

enum Move {
    FingerMove(FingerMove),
    HandMove(HandMove),
    Regrip(Regrip),
}
