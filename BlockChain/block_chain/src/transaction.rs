
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};

use crate::hashable::Hashable;
use crate::hashable::HashSha256;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::clone_into_array;


//////////////////////////////// Transaction ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction <D> {
    sender: String,
    receiver: String,
    amount: usize,  // TODO: change it to transaction fee
    load: D,
}

#[allow(dead_code)]
impl <D>Transaction<D>
    where
        D: Hashable
{
    pub fn set_sender(&mut self, sender:String) -> &mut Self {
        self.sender = sender;

        self
    }

    pub fn set_receiver(&mut self, receiver:String) -> &mut Self {
        self.receiver = receiver;

        self
    }

    pub fn set_amount(&mut self, amount:usize) -> &mut Self {
        self.amount = amount;

        self
    }

    pub fn set_load(&mut self, load:D) -> &mut Self {
        self.load = load;

        self
    }

}

impl <D>Default for Transaction<D>
    where
        D: Default
{
    fn default() -> Self {
        Self {
            sender: String::new(),
            receiver: String::new(),
            amount: 0usize,
            load: D::default(),
        }
    }
}

impl <D>Hashable for Transaction <D>
    where
        D: Hashable
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.sender.as_str());
        hasher.input(self.receiver.as_str());
        hasher.input(convert_u64_to_u8_array(self.amount as u64));
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}


//////////////////////////////// Tests /////////////////////////////////////////////////


