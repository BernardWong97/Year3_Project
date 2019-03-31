## Miner (not implemented)
Miner is responsible for block [mining](https://en.wikipedia.org/wiki/Bitcoin#Mining).


Requirements (not final):
> - &#x2611; Pass `BlockHeader` to miner.
> - &#x2610; Then `Hash` is guessed (mined) notify user about it (?? use [`Future`](https://doc.rust-lang.org/nightly/std/future/index.html)??).
> - &#x2611; Response Should include [nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce) as a prove.
> - &#x2610; Miner should have ability to cancel job. To avoid working on already mined block.

**Public functions**:
```rust
/// Create Miner
Miner::new(header: &'a BlockHeader) -> Self
/// Starts mining on current thread
Miner::start(&self) -> usize
/// Starts mining on new thread
Miner::start_thread(&self) -> JoinHandle<usize>
```
