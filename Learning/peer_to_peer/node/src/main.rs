// use std::io;
use std::thread;
use node::connector::connect;
use node::server::ini_server;

fn main() {

    let mut children = vec![]; // A vector to hold the child-threads.

    children.push(thread::spawn(move || {
        ini_server();
    }));

    children.push(thread::spawn(move || {
        connect();
    }));

    // collect each thread's result
    for child in children {
        child.join().expect("Failed to join");
    } // for

} // main