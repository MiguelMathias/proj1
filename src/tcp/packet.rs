pub struct Packet {
    srcPort: u16,
    destPort: u16,
    seqNo: u32,
    ackNo: u32,
    flags: u8,
    window: u16,
    checksum: String,
}
