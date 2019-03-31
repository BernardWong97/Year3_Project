//!
//! Hash miner
//!
//!

use sha2;
use block_chain::Block;

struct Miner{
    block: Block,
    hash: sha2::Sha256,

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
