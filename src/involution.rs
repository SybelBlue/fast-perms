use std::{collections::VecDeque, fmt::Display, cmp::*};

use crate::traits::{Identity, Mapping};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Involution(u8, u8);

pub type Swap = Involution;

impl Involution {
    pub fn new(a: u8, b: u8) -> Self {
        Involution(a.min(b), a.max(b))
    }

    pub fn contains(&self, x: u8) -> bool {
        self.0 == x || self.1 == x
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        other.contains(self.0) || other.contains(self.1)
    }

    pub fn into_tuple(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    pub fn low(&self) -> u8 {
        self.0
    }

    pub fn high(&self) -> u8 {
        self.1
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
pub struct SwapSeq(VecDeque<Involution>);

impl SwapSeq {
    pub fn new() -> Self {
        Self(VecDeque::with_capacity(4))
    }

    pub fn compose_left(&mut self, other: Involution) {
        self.0.push_front(other);
    }

    pub fn compose_right(&mut self, other: Involution) {
        self.0.push_back(other);
    }

    pub fn reduce(&mut self) {
        let mut out: VecDeque<Swap> = VecDeque::with_capacity(self.0.len());
        for curr in self.0.iter().rev() {
            if out.is_empty() {
                out.push_back(*curr);
                continue;
            }
            let mut i = 0;
            loop {
                let item = out[i];
                
                // if found identical cycle, remove both
                if curr == &item {
                    out.remove(i);
                    break;
                }
                // if found non-traversable cycle, stop here and insert
                if curr.overlaps(&item) {
                    out.insert(i, *curr);
                    break;
                }
                
                // otherwise advance past the item
                i += 1;
                
                // if no matches in out, just push to the back
                if i == out.len() {
                    out.push_back(*curr);
                    break;
                }
            }
        }
        self.0 = out;
    }
}

impl Mapping for SwapSeq {
    fn apply(&self, v: u8) -> u8 {
        self.0.iter().rev().fold(v, |x, inv| inv.apply(x))
    }

    fn order(&self) -> u8 {
        self.0.iter().map(|inv| inv.order()).max().unwrap_or(1)
    }
}

impl Identity for SwapSeq {
    fn identity(_ord: u8) -> Self {
        Self::new()
    }

    fn is_identity(&self) -> bool {
        self.0.len() == 0
    }
}
