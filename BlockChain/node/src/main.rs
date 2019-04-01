use node::Node;
use std::thread;

fn main(){
    let node: Node<String> = Node::new("".to_owned(), "".to_owned());
    let mut children = vec![];

    children.push(thread::spawn(move || {
        node.server();
    }));

    children.push(thread::spawn(move || {
        node.connect("192.168.70.1");
    }));

    // collect each thread's result
    for child in children {
        child.join().expect("Failed to join threads");
    } // for

}