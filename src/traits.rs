pub trait Mapping {
    /// `v` may be in range [`1`, `self.order()`]
    /// may panic if `v == 0`
    fn apply(&self, v: u8) -> u8;

    /// The n in S_n
    fn order(&self) -> u8;
}

pub trait Composable<Result : Mapping> {
    /// Composes self on the left of `right`, returns
    fn compose(&self, right: &Self) -> Result;
}

pub trait Invertible {
    fn inverse(&self) -> &Self;
}

pub trait Identity {
    fn identity(ord: u8) -> Self;
    fn is_identity(&self) -> bool;
}

// pub(crate) trait WellOrdered {}