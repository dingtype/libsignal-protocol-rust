use byteorder::{ByteOrder, LittleEndian};
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::libsignal::ecc;

pub struct Curve25519;

impl Curve25519 {
    /// Calculates an ECDH agreement.
    /// public_key: The Curve25519 (typically remote party's) public key.
    /// private_key: The Curve25519 (typically yours) private key.
    /// Returns a 32-byte shared secret.
    pub fn calculate_agreement(public_key: [u8; 32], private_key: [u8; 32]) -> [u8; 32] {
        let their_public_key = PublicKey::from(public_key);
        let secret = StaticSecret::from(private_key);
        let shared_key = *secret.diffie_hellman(&their_public_key).as_bytes();
        shared_key
    }

    pub fn generate_key_pair() -> ecc::KeyPair {
        let mut gen = OsRng::default();
        let mut buf = [0; 32];

        let priv_key = gen.next_u32();

        LittleEndian::write_u32(&mut buf, priv_key);

        let secret = StaticSecret::new(&mut gen);

        let pub_key = *PublicKey::from(&secret).as_bytes();

        ecc::KeyPair::new(ecc::PublicKey(pub_key), ecc::PrivateKey::new(&mut buf))
    }
}
