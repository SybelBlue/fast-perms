use crate::one_line::OneLine;

pub fn new_boxed_slice(size: usize) -> Box<[u8]> {
    Vec::with_capacity(size).into_boxed_slice()
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]

pub struct S(u8);

impl Iterator for S {
    type Item = OneLine;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}