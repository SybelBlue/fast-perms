use std::usize;

use crate::{involution::*, traits::*, utils::{new_boxed_slice, super_perm6}};

#[derive(Debug, PartialEq, Clone)]
pub struct OneLine(pub Box<[u8]>);

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

impl Mapping for OneLine {
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
}

impl Composable<OneLine> for OneLine {
    fn compose(&self, right: &Self) -> OneLine {
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

impl FromInvolutions for OneLine {
    fn from_involutions(left: &Involution, right: &Involution) -> Self {
        let ord = left.order().max(right.order());
        OneLine::new((1..=ord).map(|v| left.apply(right.apply(v))).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct OneLineSlice(pub &'static [u8]);

impl Mapping for OneLineSlice {
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
}

impl Identity for OneLineSlice {
    fn identity(ord: u8) -> Self {
        if ord > 6 {
            todo!("ord 7 and greater Slices don't exist")
        } else {
            OneLineSlice(&super_perm6[0..ord as usize])
        }
    }

    /// O(n)
    fn is_identity(&self) -> bool {
        self.0.as_ptr() == &super_perm6[0]
    }
}
