## Blockchain
Blockchain is responsible for:
- storing blocks,
- verification new blocks,
- retrieving transaction,

## Blockchain design

#### Here is blockchain design. Will be updated as project progress.

``` bash
Blockchain: {
    chain: [                            # Basic: collection storing blockchain blocks
                                        # Advance: some DB for storing blockchain blocks
        Block: {
            BlockHeader: {
                ## Bacic: fields
                index: u32,             # index of the block in the blockchain
                prev_hash:              # previous block hash
                time_stamp:             # time then block was created

                ## Advance: fields
                blockchan: UUID         # blockchan uuid block belongs to
                merkle:                 # merkle trees speed ups block verification process

                ## Mining related fields
                difficulty:              # mining difficulty
                nonce:                  # miners adjustment variable                  
            },
            BlockLoad: {
                data: Vec<T>            # collection for storing generic T type data
                                        # T probably should implement Hashable trait
                                        # Advance: limit size of the data

                                        # ?? Should merkle be here ??
            },
        }
    ]
}

```

#### Project file structure. Will be updated as project progress.

``` bash
    src:
        - blockchain:               # blockchain related files here
            - blockchain
            - lib.rs

        - block:                    # block related files here
            - block_header
            - block_load
            - block
            - lib.rs

        - node:                     # blockchain node related files
            - lib.rs

    Cargo.toml                      # Rust project file
    about.md                       # this file
```
