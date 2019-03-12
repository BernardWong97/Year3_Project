// use std::io;
use std::thread;
use std::time::Duration;
use node::connector::connect;
use node::server::ini_server;

const IP_ADDRESS: &str = "192.168.1.21";
static NETWORK: [&str; 2] = ["192.168.1.21", "192.168.1.24"];

fn main() {
    let mut children = vec![]; // A vector to hold the child-threads.

    children.push(thread::spawn(move || {
        ini_server();
    }));

    for i in NETWORK.iter() {
        match i {
            &IP_ADDRESS=> continue,
            _ => children.push(thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                connect(i);
            })),
        } // match
    } // for

    // collect each thread's result
    for child in children {
        child.join().expect("Failed to join threads");
    } // for

} // main