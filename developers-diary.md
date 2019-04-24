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
Researching on BlockChain. Preparing main parts for the Block in the BlockChain.
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
Develop a server/client application by referencing this [tutorial](https://steemit.com/technology/@tensor/rust-project-native-chat-app).  
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
- [ring](https://crates.io/crates/ring) - recommended by a person from an IRC chat.
- [bcrypt](https://crates.io/crates/bcrypt)

</br>
***Found a very nice blog with a loads of examples, crate uses, explanations and many more.
[24 days of Rust](https://siciarz.net/24-days-rust-rayon/)*** It has even a [book](http://zsiciarz.github.io/24daysofrust/book/vol1/index.html).

##### 01-03-2019
Refactored blockchain implementation:
- split into files
- removed `struct BlockLoad`
- added `struct Message`
- generalized code (add more generics)

##### 04-03-2019
Asked a question on [StackOverflow](https://stackoverflow.com/questions/54992166/in-rust-cargo-can-i-run-two-main-in-a-single-execution).  
Research on spawning processes for background process execution. [Link](https://doc.rust-lang.org/std/process/index.html)
Came out ideas for peer to peer network:
- build TCP listener and connector libraries and modules.
- different cargo execution instance/process for listener and connector.

##### 08-03-2019
I was busy by "removing obstacles" from projects way, doing other assignments.

##### 11-03-2019
Research on how to [treading](https://doc.rust-lang.org/std/thread/) in Rust.  
Implement threading to TCP listener and connector.
Study [Error Handling](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch09-00-error-handling.html).  
Found that connections only work within a network but not with public IP (consult mentor).

##### 15-03-2019
Watched few videos about Rust. [Is It Time to Rewrite the Operating System in Rust?](https://youtu.be/HgtRAbE1nBM) was very nice.  
Created virtual machines on cloud and connections work between them (Azure almost out of credit).

##### 18-03-2019
Did some code clean-up.  
Merged learning and blockchain branches to master.  
Peer2peer branch crate's function broke after Rust update.
- seems to be a problem passing static variables into functions.  
Learned that connections through router's public IP need to port forward the router, decided to make network local.

#### 21-03-2019
Created 1 virtual machine to test the network connections.  
- Currently only did test between 1 virtual machine and host machine.
- Planning on cloning another virtual machine and use another laptop to test (approx 4 machines).  
Did some documentation, will polish it more.

#### 25-03-2019
Researched [Rocket](https://rocket.rs/) examples.

#### 25-03-2019
Solved conflicts and merge peer2peer branch to master branch.
Did not do much as other multiple course module projects are due in the meantime.

#### 29-03-2019
Planed app structure.
- [Ceated skeleton](https://gitlab.com/My-/year3_project/commit/674f0723c59425982545d1af44b40c00bbdadc2e) for main app components.
- and added `about.md` files to each crate.

#### 30-03-2019
Worked on [Gan chart](https://gitlab.com/My-/year3_project/boards).
- Added issues for majority of app functionality.
- Closed some old irrelevant issues.
- Rearranged issues in Gan chart by they priority and current status.

#### 31-03-2019
Implemented block miner.

#### 31-03-2019 to 18-04-2019
Other projects, MSQ's, exams

#### 18-04-2019
Implemented [`Config`](https://gitlab.com/My-/year3_project/commit/4f2e134642f6933b97c11011dacba0026bae2744), save/load setting from file

#### 20-04-2019
[Tried](https://gitlab.com/My-/year3_project/issues/25) to create Docker container. (failed)

#### 19-04-2019 to 22-04-2019
Learning [Rocket](https://rocket.rs/). Implemented [API's](https://gitlab.com/My-/year3_project/issues/16#note_162974292):
- Miner http API
- Transaction http API
- Block http API

#### 23-04-2019
Majority API is done.  
Merged controller branch to master.  
Found out mpsc crate is not suitable for this project for sending data through socket.
Found an issue of the crate that it cannot send single string but need to wrap in a infinite loop to compensate the infinite listening.

#### 24-04-2019
Changes from mpsc channelling to TCP stream channelling.  
Study on how to [send JSON over websockets](https://thomask.sdf.org/blog/2017/07/02/sending-json-over-websockets-in-rust.html)