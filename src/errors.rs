#[derive(Debug)]
pub enum PacketError {
    ChecksumError,
    LengthError,
}
