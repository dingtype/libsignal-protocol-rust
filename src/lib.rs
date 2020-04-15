// pub mod libsignal;
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
 */

extern crate rand;
extern crate byteorder;
extern crate x25519_dalek;

mod libsignal;

// Publicize all child instances.
pub use crate::libsignal::*;
