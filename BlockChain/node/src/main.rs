use node::Node;
use std::thread;
use std::time::Duration;

fn main(){
    let node_server: Node<String> = Node::new();
    let node_client: Node<String> = Node::new();
    let mut children = Vec::new();

    children.push(thread::spawn(move || {
        node_server.server();
    }));

    children.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        node_client.connect("10.12.10.25");
    }));

    // collect each thread's result
    for child in children {
        child.join().expect("Failed to join threads");
    } // for

}