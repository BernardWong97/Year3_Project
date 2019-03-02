
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};

use crate::hashable::Hashable;
use crate::hashable::HashSha256;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::clone_into_array;


//////////////////////////////// Transaction ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction <D, V> {
    sender: String,
    receiver: String,
    value: V,
    load: D,
}

#[allow(dead_code)]
impl <D, V>Transaction<D, V>
    where
        D: Hashable,
        V: Hashable,
{

    pub fn add_sender(&mut self, sender:String) -> &mut Self {
        self.sender = sender;

        self
    }

    pub fn add_receiver(&mut self, receiver:String) -> &mut Self {
        self.receiver = receiver;

        self
    }

    pub fn add_value(&mut self, value:V) -> &mut Self {
        self.value = value;

        self
    }

    pub fn add_load(&mut self, load:D) -> &mut Self {
        self.load = load;

        self
    }

}

impl <D, V>Default for Transaction<D, V>
    where
        D: Default,
        V: Default,
{
    fn default() -> Self {
        Self {
            sender: String::new(),
            receiver: String::new(),
            value: V::default(),
            load: D::default(),
        }
    }
}

impl <D, V>Hashable for Transaction <D, V>
    where
        D: Hashable,
        V: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.sender.as_str());
        hasher.input(self.receiver.as_str());
        hasher.input(self.value.hash());
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}


//////////////////////////////// Tests /////////////////////////////////////////////////


