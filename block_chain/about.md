## Blockchain design

#### Here is blockchain desigh. Will be updted as project progesses.

``` bash
Blockchain: {
    uuid: Uuid                          # Blockchain Universally Unique Identifier (https://crates.io/crates/uuid)
    chain: [                            # Basic: colection storing blockchain blocks
                                        # Advance: some DB for storing blockchain blocks
        Block: {
            BlockHeader: {
                ## Bacic: fields
                index: u32,             # index of the block in the blockchain
                blockchan: UUID         # blockchan uuid block belongs to
                prev_hash:              # previous block hash
                time_stamp:             # time then block was created

                ## Advance: fields                
                merkle:                 # merkle trees speed ups block verification process

                ## Mining related fields
                difficuty:              # mining difficulty
                nonce:                  # miniers adjustment variable                  
            },
            load: Vec<T>                # Block load ( eg. transactions)
                                        # collection for storing generic T type data ( eg. transactions)
                                        # T probably should implement Hashable trait
                                        # Advance: limit size of the data

                                        # ?? Should merkle be here ??
        }
    ],
    transactions:[
        <T> where T: Hashable           # eg. Transaction<Message>
    ]
}

```

#### Project file tructure. Will be updated as project progress.

``` bash
    src:
        - blockchain:               # blockchain related files here
            - block
            - block_header          
            - hashable
            - message
            - transaction
            - lib.rs                # blockchain library file

        - node:                     # blockchain node related files
            - lib.rs

    Cargo.toml                      # Rust project file
    aboout.md                       # this file
```
