use crate::traits::*;

#[derive(Debug)]
pub struct OneLine(pub Box<[u8]>);

pub fn new_boxed_slice(size: usize) -> Box<[u8]> {
    Vec::with_capacity(size).into_boxed_slice()
}

impl Permutation for OneLine {
    fn apply(&self, v: u8) -> u8 {
        self.0[v as usize]
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
}
