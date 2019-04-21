use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};

use crate::hashable::clone_into_array;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::HashSha256;
use crate::hashable::Hashable;

//////////////////////////////// Transaction ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction<T> {
    sender: String,
    receiver: String,
    value: usize,
    load: T,    // in our case it will be message
}

#[allow(dead_code)]
impl<T> Transaction<T>
where
    T: Hashable,
{
    pub fn new(sender: &str, receiver: &str, load: T) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            value: 0,
            load,
        }
    }
}



impl<T> Hashable for Transaction<T>
where
    T: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(&self.sender);
        hasher.input(&self.receiver);
        hasher.input(convert_u64_to_u8_array(self.value as u64));
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}

//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_transaction_serde() {
    let tr = Transaction::new("s-1", "r-1", "message 1-1".to_string());
    // serialized
    let ser = serde_json::to_string(&tr).unwrap();
    println!("Serialifed = {}", ser);
    //deserialized
    let de:Transaction<String> = serde_json::from_str(&ser).unwrap();
    println!("Deserialized = {:?}", de);

    assert!(false);
}