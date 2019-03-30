//!
//! Node library responsible for network communications
//!

use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug)]
pub struct Node<T> {    // T for messages type. Can be a simple as string or custom struct
    in_buffer: VecDeque<T>,     // https://doc.rust-lang.org/std/collections/struct.VecDeque.html
    out_buffer: VecDeque<T>,
    // ... more ??
}



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
    pub fn get_status(&self) -> Result<String, Error> { // ??custom error, response
        let status: bool = true; // check status
        if status {
            Ok(String::from("All good!"))
        }
        else {
            Err("Oh no!")
        }
    }

    ///
    /// Get message
    ///
    pub fn get_message(&mut self) -> Option<T> {
        self.in_buffer.pop_front()?
    }

    ///
    /// Send message
    ///
    pub fn send_message(&mut self, message: T) -> Result<(), Error> {  // ?? add error if buffer is full
        self.out_buffer.push_back(message)
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
