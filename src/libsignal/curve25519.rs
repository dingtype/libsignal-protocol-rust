use byteorder::{ByteOrder, LittleEndian};
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::EphemeralSecret;
use x25519_dalek::PublicKey;

use crate::libsignal::ecc;

pub struct Curve25519;

impl Curve25519 {
    pub fn generate_key_pair() -> ecc::KeyPair {
        let mut gen = OsRng::default();
        let mut buf = [0; 32];

        let priv_key = gen.next_u32();

        LittleEndian::write_u32(&mut buf, priv_key);

        // let secret = EphemeralSecret::new(&mut priv_key);

        let secret = EphemeralSecret::new(&mut gen);

        let pub_key = *PublicKey::from(&secret).as_bytes();

        ecc::KeyPair::new(
            // ecc::PublicKey::from(pub_key),
            ecc::PublicKey(pub_key),
            ecc::PrivateKey::new(&mut buf),
        )
    }
}
