#[macro_use] extern crate arrayref;
extern crate clear_on_drop;
extern crate curve25519_dalek;
extern crate rand;

pub mod hash;
pub mod tree_hash;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
