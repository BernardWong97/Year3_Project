###### 22-01-2019
Begin research on projects ideas.

###### 23-01-2019
Created git repo on GitLab.com
Added few interesting articles to it.

###### 26-01-2019
Bernard got sick ;(

###### 28-01-2019 to 01-02-2019
Our mentor is away :(

###### 01-02-2019 to 03-02-2019
[The Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
[Rust Docks](https://doc.rust-lang.org/1.2.0/book/ffi.html)

###### 04-02-2019
Had first meeting.</p>
Preplanned plan:
- Use GitLab as git repo provider
- Build project on BlockChain
- Learn Rust language
- Use Rust to build BlockChain
- Use Docker for BlockChain and DB
- Backup data (block chain) to DB
- Use Mongo for DB

Need to decide:
- What kind of data we want to store on BlockChain.
- Finalize how our git repo should be structured (folder structure)
- Decide on front-end (do we need it? what kind of info it should display?)

We aggregated to prove what we learned Rust we need to create small project in Rust. But we need to decide what kind of project we should do.

###### 05-02-2019
We met our mentor. He is happy with our project. We need to make Project Plan by next week.
Mindaugas:
- Found out what JetBrains CLion IDE has a good support for Rust language.
- Did some research for Rust SHA256 library (added to resources).

###### 08-02-2019
Created a BlockChain (2 of them) by following tutorials.

###### 09-02-2019
Researching on BlockChain. Preparing main pars for the Block in the BlockChain.
Did research on Merkle Trees.

###### 09-02-2019
Learning Rust by solving [RoboScript-1](https://www.codewars.com/kata/roboscript-number-1-implement-syntax-highlighting/rust)

###### 11-02-2019
Updated the project milestone.

###### 13-02-2019
Added a project plan for the BlockChain project

###### 14-02-2019
Learning Rust by solving [RoboScript-2](https://www.codewars.com/kata/roboscript-number-2-implement-the-rs1-specification/train/rust)

###### 15-02-2019
Asked question on [Stack Overflow](https://stackoverflow.com/questions/54697274/how-to-update-all-the-values-in-a-btreeset?noredirect=1#comment96183233_54697274)

###### 18-02-2019
Started Block implementation.

###### 23-02-2019
Implemented blockchain.
Serialization works, Sha256 hashing works.
Testing supported development

##### 25-02-2019
Research on peer to peer BlockChain network.  
Referencing Go programming language [tutorial](https://medium.com/@mycoralhealth/code-a-simple-p2p-blockchain-in-go-46662601f417).  
Develop an server/client application by referencing this [tutorial](https://steemit.com/technology/@tensor/rust-project-native-chat-app).  
Create a branch focusing on developing and learning peer2peer application.

##### 26-02-2019
We had a meeting. During the meeting we decided to allow our BlockChain to be used for sending messages.
We planed next week tasks:
- refactor existing blockchain implementation (brake source code it in to files)
- to build basic node for our p2p network.

##### 27-02-2019
Researched on [Rust Work-spaces](https://doc.rust-lang.org/1.30.0/book/second-edition/ch14-03-cargo-workspaces.html). It will be needed to help manage project.</br>
Did some research on available Rust crypto crates:
- [rust-crypto](https://crates.io/crates/rust-crypto) - loads of cryptos, but last time updated 3 years ago.
- [ring](https://crates.io/crates/ring) - recomended by a person from an IRC chat.
- [bcrypt](https://crates.io/crates/bcrypt)

</br>
***Found a very nice blog with a loads of examples, crate uses, explanations and many more.
[24 days of Rust](https://siciarz.net/24-days-rust-rayon/)*** It has even a [book](http://zsiciarz.github.io/24daysofrust/book/vol1/index.html).

##### 01-03-2019
Refactored blockchan implementation:
- spited into files
- removed `struct BlockLoad`
- added `struct Message`
- gegericlized code (add more generics)

##### 04-03-2019
Asked a question on [StackOverflow](https://stackoverflow.com/questions/54992166/in-rust-cargo-can-i-run-two-main-in-a-single-execution).
Research on spawning processes for background process execution. [Link](https://doc.rust-lang.org/std/process/index.html)
Came out ideas for peer to peer network:
- build TCP listener and connector libraries and modules.
- different cargo execution instance/process for listener and connector.

##### 08-03-2019
I was busy by "removing obtacles" from projects way, doind othe assignments.

##### 15-03-2019
Whached few videos about Rust. [Is It Time to Rewrite the Operating System in Rust?](https://youtu.be/HgtRAbE1nBM) was very nice.

##### 18-03-2019
Did some code clean-up.
Merged learning and blockchain branches to master.
