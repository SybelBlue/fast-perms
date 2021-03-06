use std::{cmp::*, collections::VecDeque, fmt::Display, ops::{Mul, MulAssign}};

use crate::{one_line::OneLine, traits::{Identity, Mapping}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Swap(u8, u8);

pub type Involution = Swap;

impl Swap {
    pub fn new(a: u8, b: u8) -> Self {
        Swap(a.min(b), a.max(b))
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

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

impl Mapping for Swap {
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
    fn from_involutions(left: &Swap, right: &Swap) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwapSeq(VecDeque<Swap>);

impl SwapSeq {
    pub fn new() -> Self {
        Self(VecDeque::with_capacity(4))
    }

    pub fn from_cycle_notation(cycles: Vec<Vec<u8>>) -> Self {
        let mut out = Self(VecDeque::with_capacity(8));
        for cyc in cycles {
            if cyc.len() < 2 { continue; }
            
            let mut last = None;
            for &x in cyc.iter() {
                if let Some(l) = last {
                    out *= Swap::new(l, x);
                }
                last = Some(x);
            }
        }
        out
    }

    pub fn compose_left(&mut self, other: Swap) {
        self.0.push_front(other);
    }

    pub fn compose_right(&mut self, other: Swap) {
        self.0.push_back(other);
    }

    pub fn evaluate(&self) -> OneLine {
        let data = &self.0;
        match data.len() {
            0 => OneLine::identity(1),
            1 => {
                let mut out = OneLine::identity(data[0].order());
                out.compose_swap_right(&data[0]);
                out
            },
            2 => OneLine::from_involutions(&data[0], &data[1]),
            _ => {
                let mut perm = OneLine::identity(self.order());
                for swap in data {
                    perm.compose_swap_right(swap);
                }
                perm
            }
        }
    }
}

impl Mul<Swap> for SwapSeq {
    type Output = Self;

    fn mul(self, rhs: Swap) -> Self::Output {
        let mut out = self.clone();
        out.compose_right(rhs);
        out
    }
}

impl Mul<SwapSeq> for Swap {
    type Output = SwapSeq;

    fn mul(self, rhs: SwapSeq) -> Self::Output {
        let mut out = rhs.clone();
        out.compose_left(self);
        out
    }
}

impl MulAssign<Swap> for SwapSeq {
    fn mul_assign(&mut self, rhs: Swap) {
        self.compose_right(rhs);
    }
}

impl Mapping for SwapSeq {
    fn apply(&self, v: u8) -> u8 {
        self.0.iter().rev().fold(v, |x, swap| swap.apply(x))
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

impl FromInvolutions for SwapSeq {
    fn from_involutions(left: &Swap, right: &Swap) -> Self {
        let mut v = VecDeque::with_capacity(2);
        v.push_back(left.clone());
        v.push_back(right.clone());
        SwapSeq(v)
    }
}
