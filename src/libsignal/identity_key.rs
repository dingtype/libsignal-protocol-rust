use crate::libsignal::ecc;

pub struct IdentityKey {
    pub public_key: ecc::PublicKey,
}

impl IdentityKey {
    /*
    pub fn new(bytes: &[u8: 32], offset: i32) -> Self {
        Self {
            public_key: ecc::Curve::decode_point(bytes, offset),
        }
    }

    pub fn from(public_key: impl ecc::ECPublicKey<'a>) -> Self {
        Self { public_key: public_key }
    }

    pub fn get_public_key(&self) -> impl ecc::ECPublicKey {
        self.public_key
    }

    pub fn serialize(&self) -> &[u8] {
        self.public_key.serialize()
    }
    */

    fn get_fingerprint(&self) -> String {
        let ecc::PublicKey(key) = self.public_key;
        let hex_string = std::str::from_utf8(&key[..]).unwrap();
        // hex::decode(hex_string).unwrap()

        println!("hex string -> {}", hex_string);

        String::from(hex_string)
    }
}
