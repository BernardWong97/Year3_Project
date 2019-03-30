## Node
Node is responsible for controlling p2p network.

**Public functions**:
 ```Rust
/// create new node instance
Node::new() -> Self
/// connect node to the network
Node::connect(&self) -> Result<(), Box<dyn Error>>
/// disconnect node from the network
Node::disconnect(&self) -> Result<(), Box<dyn Error>>
/// get network status
Node::get_status(&self) -> Result<String> 
/// get pending network messages (new blocks, transaction, ..)
Node::get_message(&mut self) -> Option<T>
/// send message to the network (transaction, new block, ..)
Node::send_message(&mut self, message: T) -> Result<(), Error>
```
