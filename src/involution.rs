use std::fmt::Display;

use crate::traits::Mapping;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Involution(u8, u8);

impl Involution {
    pub fn new(a: u8, b: u8) -> Self {
        Involution(a.min(b), a.max(b))
    }
}

impl Display for Involution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

impl Mapping for Involution {
    fn apply(&self, v: u8) -> u8 {
        if v == self.0 {
            self.1
        } else if v == self.1 {
            self.0
        } else {
            v
        }
    }

    fn order(&self) -> u8 {
        self.1
    }
}

pub trait FromInvolutions {
    fn from_involutions(left: &Involution, right: &Involution) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvlSeq(Vec<Involution>);

