//!
//! Node library responsible for network communications
//!
extern crate eventual;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use eventual::Timer;
use std::io::{Write, Read};
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
    pub fn connect(&self, ip_address: &'static str) {
        println!("Trying to connect {}...", ip_address);
        let ip_port = format!("{}:{}", ip_address, "6000"); // combine ip address and port

        match TcpStream::connect(ip_port) {
            Ok(mut stream) => {
                println!("Successfully connected to {}", ip_address);

                //let msg = b"keep alive";

                self.close_connection(stream);

                //stream.write(msg).unwrap();

            },
            Err(_e) => {
                println!("Failed to connect to {}", ip_address);
            }
        } // match
    } // connect()

    pub fn server(&self) {
        let listener = TcpListener::bind("0.0.0.0:6000").unwrap();
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 6000...");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move|| {
                        // connection succeeded
                        let mut data = [0 as u8; 50];

                        while match stream.read(&mut data) {
                            Ok(_) => {
                                println!("{}", from_utf8(&data).unwrap());
                                true
                            },
                            Err(_) => {
                                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                                stream.shutdown(Shutdown::Both).unwrap();
                                false
                            }
                        } {}
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            } // match
        } // for
        // close the socket server
        drop(listener);
    } // server()

    fn close_connection(&self, stream: TcpStream){
        stream.shutdown(Shutdown::Both);
    }

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
