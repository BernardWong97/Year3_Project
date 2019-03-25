# Report
 
## A small follow-up report for the project
### Summary of what we did in the past weeks for the project
#### Week 1 - 2 (4<sup>th</sup> February 2019 - 15<sup>th</sup> February 2019)
    We planned a week to learn and understand the fundamental of Rust Programming Language before diving straight into
    the project but we underestimate the complexity of it as it is one of the low level programming language.  

    We researched about the language itself and how different it is comparing to other mainstream programming languages.  
    In the meantime, we also study on how Blockchain works:
    - Learned basic of Rust enough to create basic programs.
    - Study Rust SHA256 library.
    - Created 2 Blockchains by following tutorial online.
    - Research on Merkle Trees.
    - Solving CodeWars challenge.
    - Ask question on StackOverflow.
#### Week 3 (18<sup>th</sup> February 2019 - 22<sup>th</sup> February 2019)
    We pushed the Blockchain implementation to this week as we spent two weeks on the programming language itself.
    
    We discussed about multiple issues arise during the implementation:
    - Data that stored inside transactions of the block:
        - Was planning to do some universities blockchain for students' data.
        - Decided to use a generic struct to hold data. ( like JSON )
    - What consensus algorithm to use (proof of what):
        - Proof of work algorithm is common on crypto-mining but makes no sense for universities to mine block.
        - Decided to not overcomplicate the project, just a small proof of work algorithm should do.
    - Split the project into two parts:
        - Blockchain itself ( assigned to Mindaugas )
        - Blockchain decentralized network ( assigned to Bernard )
        
    Mindaugas worked on serialization and hashing.
#### Week 4 (25<sup>th</sup> February 2019 - 1<sup>st</sup> March 2019)
    Discussed that we decided to allow our Blockchain to be used for sending messages (JSON).
    Did research on how Rust workspace operates.

    Bernard:
    - Researched on peer to peer BlockChain network from videos and Go programming language reference.
    - Developed a server/client application by referencing tutorials.  
    - Created a branch named "peer2peer" focusing on developing and learning peer to peer application.
    
    Mindaugas:
    - Researched on available Rust crypto crates:
        - rust-crypto
        - ring
        - bcrypt
    - Refactored blockchain implementation:
        - splited into files
        - generalized code to add more generics
#### Week 5 (4<sup>th</sup> March 2019 - 8<sup>th</sup> March 2019)
    Bernard:
    - Try to solve application stop when listening to incoming TCP connection.
        - Asked a question on StackOverflow that is it possible to run multiple main function in single execution.
    - Research on spawning processes for background process execution.
    - Came out ideas for peer to peer network:
        - build TCP listener and connector into different libraries and modules.
        - different cargo execution instance/process for listener and connector.

#### Week 6 (11<sup>th</sup> March 2019 - 15<sup>th</sup> March 2019)
    Mindaugas:
    - Watched few Youtube videos about Rust.
    
    Bernard:
    - Study Rust error handling.
    - Adviced to use threading rather than spawning processes on networking.
    - Research on how to treading in Rust.
    - Implement threading to TCP listener and connector.
    - Found that connections only work within a network but not with public IP.  
    - Created Azure virtual machines and connections work between them (Azure almost out of credit).
#### Week 7 - 8 (18<sup>th</sup> March 2019 - 25<sup>th</sup> March 2019)
    We discussed the usage MVC/MVVM concept.
    Decided to use a controller to link between components (view/network/Blockchain)
    
    Mindaugas:
    - Did some code clean-up.
    - Merged learning and blockchain branches to master.
    - Created skeleton for the Blockchain node.
    
    Bernard:
    - Peer2peer branch crate's function broke after Rust update.
        - seems to be a problem passing static variables into functions.
    - Learned that connections through router's public IP need to port forward the router
    - Discussed with Mindaugase and decided to make network local.
    - Created 1 virtual machine to test the network connections.
        - currently only did test between 1 virtual machine and host machine.  
    - Did some documentation, will polish it more.
    - Solved conflicts and merge peer2peer branch to master branch.

    Note: We both did not do much as other multiple course module projects are due in the meantime.