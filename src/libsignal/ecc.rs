use crate::helpers;
use crate::libsignal::{curve25519, ecc, protocol};
use std::convert::TryInto;

static DJB_TYPE: u8 = 0x05;

pub struct Curve;

pub struct InvalidKeyError(pub String);

impl Curve {
    pub fn generate_key_pair() -> KeyPair {
        super::curve25519::generate_key_pair()
    }
}

pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl KeyPair {
    pub fn new(public: PublicKey, private: PrivateKey) -> Self {
        Self {
            public_key: public,
            private_key: private,
        }
    }

    pub fn decode_point(
        bytes: &[u8],
        offset: usize,
    ) -> Result<impl ecc::ECPublicKey, InvalidKeyError> {
        if bytes.len() == 0 || bytes.len() - offset < 1 {
            Err(InvalidKeyError("No key type identifier".to_string()))
        } else {
            // Truncate the number to last 8 bits
            let type_ = &bytes[offset] & 0xff;

            println!("type_ {}", type_);

            if type_ == DJB_TYPE.try_into().unwrap() {
                if bytes.len() - offset < 33 {
                    Err(InvalidKeyError(
                        format!("Bad key length: {}", bytes.len()).to_string(),
                    ))
                } else {
                    let mut key_bytes = &[0; 32][..];
                    let start_pos = offset + 1;
                    let result =
                        helpers::slices::copy(&bytes, offset + 1, key_bytes, 0, key_bytes.len());
                    match result {
                        Ok(v) => match helpers::slices::to_array32(&(&v)) {
                            Ok(arr) => Ok(PublicKey(arr)),
                            Err(_) => Err(InvalidKeyError(format!("Bad key type: {}", type_))),
                        },
                        Err(_) => Err(InvalidKeyError(format!("Bad key type: {}", type_))),
                    }
                }
            } else {
                Err(InvalidKeyError(format!("Bad key type: {}", type_)))
            }
        }
    }
}

pub trait ECPublicKey {
    fn from(bytes: [u8; 32]) -> Self;
    fn serialize(&self) -> [u8; 32];
    fn get_type(&self) -> protocol::Type;
}

pub trait ECPrivateKey {
    fn serialize(&self) -> [u8; 32];
    fn get_type(&self) -> protocol::Type;
}

pub struct PrivateKey(pub [u8; 32]);

impl PrivateKey {
    pub fn new(bytes: &mut [u8; 32]) -> Self {
        let mut buf: [u8; 32] = [0; 32];
        buf.clone_from_slice(bytes);
        Self(buf)
    }
}

impl ECPrivateKey for PrivateKey {
    fn serialize(&self) -> [u8; 32] {
        [0; 32]
    }

    fn get_type(&self) -> protocol::Type {
        protocol::Type::Unknown
    }
}

pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    pub fn new(bytes: [u8; 32]) -> Self {
        PublicKey(bytes)
    }
}

impl ECPublicKey for PublicKey {
    fn from(bytes: [u8; 32]) -> Self {
        PublicKey(bytes)
    }

    fn serialize(&self) -> [u8; 32] {
        // FIXME: Stub
        [0; 32]
    }

    fn get_type(&self) -> protocol::Type {
        // FIXME: Stub
        protocol::Type::Unknown
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn test_keypair_decode_point() {
        let b1 = &[];
        let b2 = &[0; 10];
        let offset1 = 2;
        let offset2 = 10;
        let b3 = &helpers::slices::concat_2(&[0x00, 0x08, 0x05], &[0x00; 64][..]);

        match KeyPair::decode_point(b1, 2) {
            Ok(_) => panic!("Expected Error"),
            Err(InvalidKeyError(s)) => assert_eq!(s, "No key type identifier".to_string()),
        }

        match KeyPair::decode_point(b2, 10) {
            Ok(_) => panic!("Expected Error"),
            Err(InvalidKeyError(s)) => assert_eq!(s, "No key type identifier".to_string()),
        }

        match KeyPair::decode_point(b3, 2) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Expect Ok, found Err"),
        }
    }
}
