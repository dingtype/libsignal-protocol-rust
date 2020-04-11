const CURRENT_VERSION: i32 = 3;

const WHISPER_TYPE: i32                = 2;
const PREKEY_TYPE: i32                 = 3;
const SENDERKEY_TYPE: i32              = 4;
const SENDERKEY_DISTRIBUTION_TYPE: i32 = 5;

// This should be the worst case (worse than V2). So not always accurate, but good enough for padding.
const ENCRYPTED_MESSAGE_OVERHEAD: i32 = 53;

pub enum Type {
    Whisper(i32),
    Prekey(i32),
    Senderkey(i32),
    SenderkeyDistribution(i32),
    EncryptedMessageOverhead(i32),
}

pub trait CiphertextMessage {
    fn serialize() -> Vec<u8>;
    fn get_type() -> Type;
}

pub struct DeviceConsistencyMessage {}

pub struct PreKeySignalMessage {}

impl CiphertextMessage for PreKeySignalMessage {
    fn serialize() -> Vec<u8> {
	Vec::new()
    }
    
    fn get_type() -> Type {
	Type::Prekey(PREKEY_TYPE)
    }
}

pub struct SenderKeyDistributionMessage {}
impl CiphertextMessage for SenderKeyDistributionMessage {
    fn serialize() -> Vec<u8> {
	Vec::new()
    }
    
    fn get_type() -> Type {
	Type::SenderkeyDistribution(SENDERKEY_DISTRIBUTION_TYPE)
    }
}

pub struct SenderKeyMessage {}
impl CiphertextMessage for SenderKeyMessage {
    fn serialize() -> Vec<u8> {
	Vec::new()
    }
    
    fn get_type() -> Type {
	Type::Senderkey(SENDERKEY_TYPE)
    }
}

pub struct SignalMessage {}
impl CiphertextMessage for SignalMessage {
    fn serialize() -> Vec<u8> {
	Vec::new()
    }
    
    fn get_type() -> Type {
	Type::Whisper(WHISPER_TYPE)
    }
}

