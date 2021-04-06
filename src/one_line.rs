use std::usize;

use crate::{involution::*, traits::*, group_generators::SUPER_PERM6};

#[derive(Debug, PartialEq, Clone)]
pub struct OneLine(pub Box<[u8]>);

impl OneLine {
    pub fn new(data: Vec<u8>) -> Self {
        OneLine(data.into_boxed_slice())
    }

    /// performs self * swap (right composition)
    pub fn compose_swap_right(&mut self, swap: &Swap) {
        if swap.order() < self.order() {
            let data = self.0.as_mut();
            let (x, y) = swap.into_tuple();
            let temp = data[x as usize];
            data[x as usize] = data[y as usize];
            data[y as usize] = temp;
        } else {
            let order = self.order().max(swap.order());
            self.0 = (1..=order).map(|i| self.apply(swap.apply(i))).collect::<Vec<u8>>().into_boxed_slice();
        }
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
        if v <= self.0.len() as u8 {
            self.0[v as usize - 1]
        } else {
            v
        }
    }

    fn order(&self) -> u8 {
        self.0.len() as u8
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
    fn from_involutions(left: &Swap, right: &Swap) -> Self {
        let ord = left.order().max(right.order());
        OneLine::new((1..=ord).map(|v| left.apply(right.apply(v))).collect())
    }
}

impl<T> Composable<OneLine> for T where T : Mapping {
    fn compose(&self, right: &Self) -> OneLine {
        let ord = self.order().max(right.order());
        OneLine((1..=ord).map(|i| self.apply(right.apply(i))).collect::<Vec<u8>>().into_boxed_slice())
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
            OneLineSlice(&SUPER_PERM6[0..ord as usize])
        }
    }

    /// O(n)
    fn is_identity(&self) -> bool {
        self.0.as_ptr() == &SUPER_PERM6[0]
    }
}
