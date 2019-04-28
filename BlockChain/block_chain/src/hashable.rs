//!
//! This is utility class.
//!
//! ToDo:
//! - remove it. redesign.



////////////////////////////// Hash type ////////////////////////////

const HASH_BYTE_SIZE: usize = 32;

pub type HashSha256 = [u8; HASH_BYTE_SIZE];

////////////////////////////// Hashable trait ////////////////////////////

pub trait Hashable {
    fn hash(&self) -> HashSha256;
}

//////////////////////////////// Helper functions ////////////////////////////

///
///  <br>[code reference](https://stackoverflow.com/a/37679019/5322506)
///
pub fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

///
/// Concerts `u32` number as `u8` array
/// <br>[code reference](https://freestartupkits.com/articles/technology/cryptocurrency-news-and-tips/ultimate-rust-blockchain-tutorial/)
///
pub fn convert_u32_to_u8_array(val: u32) -> [u8; 4] {
    return [
        val as u8,
        (val >> 8 * 1) as u8,
        (val >> 8 * 2) as u8,
        (val >> 8 * 3) as u8,
    ];
}

///
/// Concerts `u32` number as `u8` array
/// <br>[code reference](https://freestartupkits.com/articles/technology/cryptocurrency-news-and-tips/ultimate-rust-blockchain-tutorial/)
///
pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8 * 1) as u8,
        (val >> 8 * 2) as u8,
        (val >> 8 * 3) as u8,
        (val >> 8 * 4) as u8,
        (val >> 8 * 5) as u8,
        (val >> 8 * 6) as u8,
        (val >> 8 * 7) as u8,
    ];
}

//////////////////////////////// Tests /////////////////////////////////////////////////
