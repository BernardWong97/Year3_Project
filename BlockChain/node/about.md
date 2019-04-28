## Node
Node is responsible for controlling p2p network.

**Public functions**:
 ```Rust
/// create new node instance
Node::new() -> Self
/// send a string of message to the ip_address
Node::send_message(&self, ip_address: &'static str, message: String)
/// check the connection of the ip address by pinging the server's port 6000
Node::ping_server(&self, ip_address: &'static str)
/// instantiate the node's server listening from port 6000
Node::server(&self)
```
