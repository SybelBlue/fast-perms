pub trait Permutation {
    /// `v` may be in range [`1`, `self.order()`]
    fn apply(&self, v: u8) -> u8;

    /// The n in S_n
    fn order(&self) -> u8;

    /// Composes self on the left of `right`, returns
    fn compose(&self, right: &Self) -> Self;
}

pub trait Invertible {
    fn inverse(&self) -> &Self;
}

pub trait Identity {
    fn identity(ord: u8) -> Self;
}

// pub(crate) trait WellOrdered {}