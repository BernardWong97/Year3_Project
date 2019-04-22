//!
//! Node library responsible for network communications
//!

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug)]
pub struct NodeError<'a>{
    err: &'a str,
}

impl<'a> NodeError<'a> {
    pub fn new(err:&'a str) -> Self {
        Self{ err }
    }
}

impl<'a> Error for NodeError<'a> {
}

impl<'a> fmt::Display for NodeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node error is here!")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<T> {    // T for messages type. Can be a simple as string or custom struct
    in_buffer: VecDeque<T>,     // https://doc.rust-lang.org/std/collections/struct.VecDeque.html
    out_buffer: VecDeque<T>,
    // ... more ??
}

#[allow(dead_code)]
impl<T> Node <T> {
    ///
    /// Create new node
    ///
    pub fn new() -> Self {
        Self {
            in_buffer: VecDeque::new(),
            out_buffer: VecDeque::new(),
        }
    }

    ///
    /// Connect node to the net.
    ///
    /// ..Probably we need to pass addresses were to connect..
    pub fn connect(&self) -> Result<(), Box<dyn Error>> {   // return is same as in CLI tutorial
        // if smth wrong: Err("Shit! Something went wrong!")

        Ok(())
    }

    /// get network status
    pub fn get_status(&self) -> Result<String, Box<dyn Error>> { // ??custom error, response
        let status: bool = true; // check status
        if status {
            Ok(String::from("All good!"))
        }
        else {
            Err(Box::new(NodeError::new("node error")))
        }
    }

    ///
    /// Get message
    ///
    pub fn get_message(&mut self) -> Option<T> {
        self.in_buffer.pop_front()
    }

    ///
    /// Send message
    ///
    pub fn send_message(&mut self, message: T) -> Result<(), Box<dyn Error>> {  // ?? add error if buffer is full
        Ok(self.out_buffer.push_back(message))
    }

    // ??... other methods.. ??
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert!(false, "unimplemented");
    }
}
