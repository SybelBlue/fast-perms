use std::usize;

use crate::traits::*;

#[derive(Debug, PartialEq)]
pub struct OneLine(pub Box<[u8]>);

pub fn new_boxed_slice(size: usize) -> Box<[u8]> {
    Vec::with_capacity(size).into_boxed_slice()
}

impl OneLine {
    pub fn new(data: Vec<u8>) -> Self {
        OneLine(data.into_boxed_slice())
    }

    pub fn validate(&self) {
        if self.order() == 0 { return; }

        let mut seen = self.0.iter().map(|_| false).collect::<Vec<bool>>();
        for i in self.0.iter() {
            if *i == 0 || *i > self.order() || seen[*i as usize - 1] {
                panic!(format!("Invalid OneLine: {:?}", self))
            }
            seen[*i as usize - 1] = true;
        }
    }
}

impl Permutation for OneLine {
    fn apply(&self, v: u8) -> u8 {
        if v > 0 && v <= self.0.len() as u8 {
            self.0[v as usize - 1]
        } else {
            v
        }
    }

    fn order(&self) -> u8 {
        self.0.len() as u8
    }

    fn compose(&self, right: &Self) -> Self {
        let ord = self.order().max(right.order());
        let mut data: Box<[u8]> = new_boxed_slice(ord as usize);
        for (i, d) in data.iter_mut().enumerate() {
            *d = self.apply(right.apply(i as u8 + 1));
        }
        OneLine(data)
    }
}

impl Identity for OneLine {
    fn identity(ord: u8) -> Self {
        OneLine((1..=ord).collect::<Vec<u8>>().into_boxed_slice())
    }

    /// O(n)
    fn is_identity(&self) -> bool {
        for (i, d) in self.0.iter().enumerate() {
            if i as u8 + 1 != *d {
                return false;
            }
        }
        true
    }
}
