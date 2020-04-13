use crate::libsignal::{protocol, curve25519};

pub mod curve {
    
    pub fn generate_key_pair() -> super::KeyPair {
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

    
