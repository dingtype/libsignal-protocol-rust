use crate::libsignal;

const VERSION: &str = "DeviceConsistencycommitment_V0";

#[derive(Default)]
pub struct DeviceConsistencyCommitment {
    generation: i32,
    serialized: Vec<u8>,
}

impl DeviceConsistencyCommitment {
    pub fn new(generation: i32, identityKeys: Vec<libsignal::IdentityKey>) -> DeviceConsistencyCommitment {
	// Stub
	DeviceConsistencyCommitment {
	    generation: 0,
	    serialized: Vec::new(),
	}
    }
}

#[derive(Default)]
pub struct DeviceConsistencySignature {
    signature: [u8; 32],
    vrf_output: [u8; 32],
}


