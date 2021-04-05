use crate::traits::Mapping;

/// OneLine representation for < S16
/// Each nibble is an entry ie:
/// `0xfedcba9876543210` is identity, where
/// mapping for x is at (x-1)th nibble + 1.
///
/// Involution(2, 4) is
/// `0xfe...541230`
pub struct Perm64(pub u64);

impl Mapping for Perm64 {
    fn apply(&self, v: u8) -> u8 {
        if v > 16 {
            v
        } else {
            ((self.0 >> (4 * (v - 1))) & 0xF) as u8 + 1
        }
    }

    fn order(&self) -> u8 {
        todo!()
    }
}