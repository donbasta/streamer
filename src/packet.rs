pub struct Packet {
    p_type: usize,
    p_sequence: usize,
    p_data: [u8],
}

impl Packet {
    pub fn new(p_type: usize, p_sequence: usize, p_data: [u8]) -> Packet {
        assert!(AVAILABLE_TYPES.contains(p_type));
        assert!(0 <= len(p_data) <= MAX_LENGTH);
        assert!(0 <= p_sequence <= MAX_SEQUENCE);

        if vec![TYPE_ACK, TYPE_FINACK].contains(p_type) {
            assert!(len(p_data) == 0);
        }

        Packet {
            p_type,
            p_sequence,
            p_data,
        }
    }

    pub fn generate_checksum(&self, head: [u8]) -> usize {
        check_data = head + self.p_data;
        if len(check_data) % 2 != 0 {
            check_data += b'\x00';
        }
        checksum = 0;
    }
}
