use crate::{involution::{FromInvolutions, Involution}, traits::{Composable, Identity, Mapping}};

/// OneLine representation for < S16
/// Each nibble is an entry ie:
/// `0xfedcba9876543210` is identity, where
/// mapping for x is at (x-1)th nibble + 1.
///
/// eg Involution(2, 4) is
/// `0xfe...541230`
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Perm64(u64);

impl Mapping for Perm64 {
    fn apply(&self, v: u8) -> u8 {
        if v > 16 {
            v
        } else {
            ((self.0 >> (4 * (v - 1))) & 0xF) as u8 + 1
        }
    }

    fn order(&self) -> u8 {
        16
    }
}

impl Composable<Perm64> for Perm64 {
    fn compose(&self, right: &Self) -> Perm64 {
        let mut data = 0;
        for i in 1..=16 {
            data |= (self.apply(right.apply(i)) as u64 - 1) << (4 * (i - 1));
        }
        Perm64(data)
    }
}

impl FromInvolutions for Perm64 {
    fn from_involutions(left: &Involution, right: &Involution) -> Self {
        let mut data = 0;
        for i in 1..=16 {
            data |= (left.apply(right.apply(i)) as u64 - 1) << (4 * (i - 1));
        }
        Perm64(data)
    }
}

impl Identity for Perm64 {
    fn identity(_ord: u8) -> Self {
        Perm64(0xFEDCBA9876543210)
    }

    fn is_identity(&self) -> bool {
        self.0 == 0xFEDCBA9876543210
    }
}