//!
//! Node library responsible for network communications
//!
extern crate local_ip;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{Read, Write};
use std::str::from_utf8;

#[derive(Debug)]
pub struct Node<T> {    // T for messages type. Can be a simple as string or custom struct
    in_buffer: Vec<T>,     // https://doc.rust-lang.org/std/collections/struct.VecDeque.html
    out_buffer: Vec<T>,
    // ... more ??
}

impl<T> Node <T> {
    ///
    /// Create new node
    ///
    pub fn new() -> Self {
        Self {
            in_buffer: Vec::new(),
            out_buffer: Vec::new(),
        }
    }

    ///
    /// Connect node to the net.
    ///
    /// ..Probably we need to pass addresses were to connect..
    pub fn send_message(&self, ip_address: &'static str, message: String) {
        let self_local_ip = local_ip::get().unwrap().to_string();

        // if ip match self ip
        if self_local_ip != ip_address{
            println!("Trying to connect {}...", ip_address);
            let ip_port = format!("{}:{}", ip_address, "6000"); // combine ip address and port

            match TcpStream::connect(ip_port) {
                Ok(mut stream) => {
                    println!("Successfully connected to {}", ip_address);
                    let msg = message.as_bytes();
                    stream.write(msg).unwrap();
                },
                Err(_e) => {
                    println!("Failed to connect to {}", ip_address);
                }
            } // match
        } // if
    } // connect()

    pub fn ping_server(&self, ip_address: &'static str) {
        let self_local_ip = local_ip::get().unwrap().to_string();

        // if ip match self ip
        if self_local_ip != ip_address{
            println!("Trying to connect {}...", ip_address);
            let ip_port = format!("{}:{}", ip_address, "6000"); // combine ip address and port

            match TcpStream::connect(ip_port) {
                Ok(mut stream) => {
                    let msg = b"ping";
                    stream.write(msg).unwrap();

                    let mut data = [0 as u8; 4];
                    match stream.read(&mut data) {
                        Ok(_) => {
                            if &data == msg {
                                println!("Pinged {} successful", ip_address);
                            } else {
                                let text = from_utf8(&data).unwrap();
                                println!("Unexpected reply: {}", text);
                            }
                        },
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    } // match

                },
                Err(_e) => {
                    println!("Failed to connect to {}", ip_address);
                }
            } // match
        } // if
    } // ping_server()

    pub fn server(&self) {
        let listener = TcpListener::bind("0.0.0.0:6000").unwrap();
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 6000...");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move|| {
                        // connection succeeded
                        let mut data = [0 as u8; 50];

                        match stream.read(&mut data) {
                            Ok(size) => {
                                if from_utf8(&data[0..size]).unwrap() == "ping"{
                                    stream.write(&data[0..size]).unwrap();
                                    println!("Pinged from {}", stream.peer_addr().unwrap().ip());
                                } // if
                                true
                            },
                            Err(_) => {
                                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap().ip());
                                stream.shutdown(Shutdown::Both).unwrap();
                                false
                            }
                        }
                    });
                }
                Err(e) => {
                    println!("Connection Failed: {}", e);
                }
            } // match
        } // for
        // close the socket server
        drop(listener);
    } // server()

    //// get network status
  //  pub fn get_status(&self) -> Result<String, Error> { // ??custom error, response
 //       let status: bool = true; // check status
      //  if status {
       //     Ok(String::from("All good!"))
       // }
        //else {
           // Err("Oh no!")
        //}
    //}

    ////
    //// Get message
    ////
    //pub fn get_message(&mut self) -> Option<T> {
        //self.in_buffer.pop_front()?
    //}

//    ///
//    /// Send message
//    ///
//    pub fn send_message(&mut self, message: T) -> Result<(), Error> {  // ?? add error if buffer is full
//        self.out_buffer.push_back(message)
//
//
//    }

    // ??... other methods.. ??

}



#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn it_works() {
        let node: Node<String> = Node::new();
        assert!(true, node.connect("192.168.71.1"))
    }
}
