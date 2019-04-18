## This is controller for the blockchain app.
Controller should be responsible for controlling:
- chain (blockchain crate),
- node (node crate),
- miner (miner crate).


**Implementation**:
- initialize `Controller` ( probably it should be named as `App`).
- `Controller` fields:
    - `Node`
    - `BlockChain`
    - `Miner` (Optional)
- load previously saved details (blockchain, known nodes)
- if no previously saved details, create new `BlockChain`
- connect to the `DEFAULT_NODE` if none is given
- if connection successful:
    - ask network for last `Block`
    - if block is newer then the we have: ask for update.
    - after synchronization start mining block if applicable.
    - or start sending messages.
    
### config.rs
This is `AppConfig` - `App` configuration structure.

```rust
pub struct AppConfig<'a>{
    settings_file_name: &'a str,    // settings file name
    settings: Vec<String>
}

```
