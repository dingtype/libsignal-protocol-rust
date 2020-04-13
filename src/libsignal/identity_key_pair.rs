use crate::libsignal;
use crate::libsignal::ecc;

pub struct IdentityKeyPair {
    public_key: libsignal::IdentityKey,
    private_key: ecc::PrivateKey,
}
