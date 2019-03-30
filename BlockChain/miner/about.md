## Miner (not implemented)
Miner is responsible for block [mining](https://en.wikipedia.org/wiki/Bitcoin#Mining).


Requirements (not final):
> - Pass `Hash (HashSha256)` to miner.
> - Then `Hash` is guessed (mined) notify user about it (?? use [`Future`](https://doc.rust-lang.org/nightly/std/future/index.html)??).
> - Response Should include [nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce) as a prove.
> - Miner should have ability to cancel job. To avoid working on already mined block.
