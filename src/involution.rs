use std::{collections::VecDeque, fmt::Display};

use crate::traits::{Identity, Mapping};

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
pub struct InvlSeq(VecDeque<Involution>);

impl InvlSeq {
    pub fn new() -> Self {
        InvlSeq(VecDeque::with_capacity(4))
    }

    pub fn compose_left(&mut self, other: Involution) {
        self.0.push_front(other);
    }

    pub fn compose_right(&mut self, other: Involution) {
        self.0.push_back(other);
    }
}

impl Mapping for InvlSeq {
    fn apply(&self, v: u8) -> u8 {
        self.0.iter().rev().fold(v, |x, inv| inv.apply(x))
    }

    fn order(&self) -> u8 {
        self.0.iter().map(|inv| inv.order()).max().unwrap_or(1)
    }
}

impl Identity for InvlSeq {
    fn identity(_ord: u8) -> Self {
        InvlSeq::new()
    }

    fn is_identity(&self) -> bool {
        self.0.len() == 0
    }
}
