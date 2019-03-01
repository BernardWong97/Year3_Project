
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};
use bcrypt::{DEFAULT_COST, hash, verify};


use crate::hashable::Hashable;
use crate::hashable::HashSha256;
use crate::hashable::clone_into_array;


//////////////////////////////// Message ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message{
    sender: String,
    receiver: String,
    text: String,
}

impl Message {
    pub fn new(sender:&str, receiver:&str, text:&str) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            text: text.to_string(),
        }
    }
}

impl Hashable for Message {
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(&self.sender);
        hasher.input(&self.receiver);
        hasher.input(&self.text);

        clone_into_array(hasher.result().as_slice())
    }
}


//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_message_serde(){
    let message = Message::new("mindaugas", "bernard", "how are you?");

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a object.
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized, message);

//    assert!(false);
}

#[test]
fn test_message_bcrypt(){
    let message = Message::new("mindaugas", "bernard", "how are you?");

    let hashed = hash(&message.text, 10);

    // <svartalf> I think it is better to validate result with `assert!(result.is_ok())`, because there is thin difference between "test failed because of assertion" and "test failed because of random panic" :)
    assert!(hashed.is_ok());
    let hashed = hashed.unwrap();

    let valid = verify(&message.text, &hashed).unwrap();

    println!("hashed: {:?} - {}", hashed, valid);

    assert!(valid);

    assert!(false);
}

#[test]
fn test_message_cyper(){
    // https://siciarz.net/24-days-of-rust-rust-crypto/
    let message = Message::new("mindaugas", "bernard", "how are you?");

    println!("{:?}", message);

    assert!(false);
}