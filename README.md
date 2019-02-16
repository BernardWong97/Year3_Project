### About
This repository is for Year 3 group project.  
Project Idea is to learn Rust Programming Language while creating a BlockChain program using Rust and possibly build into a Xamarin App or deploy into Docker for data access.

### Project ideas
Here are a few ideas we have/had:
- Data mining
- Blockchain &#x2611;

Few personal goals:
- Multilingual project
- Learn a new language:
  - Rust &#x2611;
  - Bash
  - Python
  - Scala
- Use Docker (Kubernetes)

### About Rust
Rust is a statically typed language.
- ["The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection."](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- ["This is called destructuring because it breaks the single tuple into three parts."](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### About BlockChain

Parts:
- Block:
  - Block Header:
    - A hash of the previous header
    - A timestamp
    - A mining difficulty value
    - A proof of work nonce
    - A root hash for the Merkle tree containing the transactions for that block.

  - **nonce**:  A 64-bit hash, which proves, combined with the mix-hash, that a sufficient amount of computation has been carried out on this block.

  - **timestamp**: A scalar value equal to the reasonable output of Unix time() function at this block inception.

  - **mixhash**: A 256-bit hash which proves, combined with the nonce, that a sufficient amount of computation has been carried out on this block.

  - **difficulty**: A scalar value corresponding to the difficulty level applied during the nonce discovering of block.

  - **alloc**: Allows defining a list of pre-filled wallets. It’s an Ethereum specific functionality to handle the “Ether pre-sale” period.

  - **parentHash**: The Keccak 256-bit hash of the entire parent block header (including its nonce and mixhash).

  - **extraData**: An optional free, but max. 32-byte long space to conserve smart things for ethernity.

  - **gasLimit**: A scalar value equal to the current chain-wide limit of Gas expenditure per block.

  - **coinbase**: The very first transaction included in the block by the miners.

  - **merkle**: A merkle tree, also known as a binary hash tree, is a data structure used for efficiently summarizing and verifying the integrity of large sets of data. - quote Andreas M. Antonopoulos




### Resources
##### Docker
- [**What is docker** Learn Enough Docker to be Useful 1.](https://towardsdatascience.com/learn-enough-docker-to-be-useful-b7ba70caeb4b)
- [**Docker terminology** Learn Enough Docker to be Useful 2.](https://towardsdatascience.com/learn-enough-docker-to-be-useful-1c40ea269fa8)
- [**Docker files** Learn Enough Docker to be Useful 3.](https://towardsdatascience.com/learn-enough-docker-to-be-useful-b0b44222eef5)
- [Docker Tutorial for Beginners](https://hashnode.com/post/docker-tutorial-for-beginners-cjrj2hg5001s2ufs1nker9he2)

##### Big Data
- [Deep learning with Python](https://towardsdatascience.com/deep-learning-with-python-703e26853820)

##### BlockChain
- [Creating Your First Blockchain with Java 1.](https://medium.com/programmers-blockchain/create-simple-blockchain-java-tutorial-from-scratch-6eeed3cb03fa)
- [Creating Your First Blockchain with Java 2.](https://medium.com/programmers-blockchain/creating-your-first-blockchain-with-java-part-2-transactions-2cdac335e0ce)
- [Blockchain Tutorial – A Beginner’s Guide to Blockchain Technology](https://www.edureka.co/blog/blockchain-tutorial/)
- [Merkle Trees](https://www.blockchain-council.org/blockchain/what-is-merkel-tree-merkel-root-in-blockchain/)
- [Blockchain Fundamentals #1: What is a Merkle Tree?](https://medium.com/byzantine-studio/blockchain-fundamentals-what-is-a-merkle-tree-d44c529391d7)
- [Merkling in Ethereum](https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/)


##### Rust language
- [The Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust Docks](https://doc.rust-lang.org/1.2.0/book/ffi.html)
- [Rust SHA256 - crates.io](https://crates.io/crates/byte_sha)
- [Rust String conversions](https://stackoverflow.com/questions/41034635/idiomatic-transformations-for-string-str-vecu8-and-u8)
- [Bitcoin Hashes Library](https://crates.io/crates/bitcoin_hashes)
- [New in Rust 1.31 / Rust2018](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
- [Rust2018](https://rust-lang-nursery.github.io/edition-guide/rust-2018/cargo-and-crates-io/cargo-can-use-a-local-registry-replacement.html)
- [New in Rust 1.32](https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html)
- [Use Rust from Java](https://github.com/sureshg/java-rust-ffi)
- [Rust Awesome List](https://github.com/rust-unofficial/awesome-rust#cryptocurrencies)
- [Regex in Rust](https://docs.rs/regex/1.1.0/regex/)
- [Rust Youtube Tutorial Playlist](https://www.youtube.com/playlist?list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW)
