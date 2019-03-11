use crate::hashable::Hashable;
use serde::{Deserialize, Serialize};

pub trait DB<D> {
    fn new() -> Self;
    fn push(&mut self, data: D);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainDB<D> {
    data: Vec<D>,
}

impl<D> ChainDB<D> {
    pub fn last(&self) -> Option<&D> {
        self.data.last()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<D> DB<D> for ChainDB<D>
where
    D: Hashable,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, data: D) {
        self.data.push(data);
    }
}

//impl<D> Iterator for ChainDB<D> {
//    type Item = D;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let b = self.data.pop();
//
//        b
//    }
//}
