use crate::constant::*;

pub struct Packet {
    p_type: usize,
    p_sequence: usize,
    p_data: Box<[u8]>,
}

impl Packet {
    pub fn new(p_type: usize, p_sequence: usize, p_data: &[u8]) -> Packet {
        //assert!(AVAILABLE_TYPES.contains(p_type));
        assert!(0 <= p_data.len() && p_data.len() <= MAX_LENGTH);
        assert!(0 <= p_sequence && p_sequence <= MAX_SEQUENCE);

        if [TYPE_ACK, TYPE_FINACK].contains(&p_type) {
            assert!(p_data.len() == 0);
        }

        Packet {
            p_type,
            p_sequence,
            p_data: Box::from(p_data),
        }
    }

    // pub fn generate_checksum(&self, head: &[u8]) -> usize {
    //     let check_data = head + self.p_data;
    //     if check_data.len() % 2 != 0 {
    //         check_data += b'\x00';
    //     }
    //     let checksum = 0;
    // }

    pub fn to_bytes(&self) -> &[u8] {
        todo!();
    }

    pub fn to_packet(raw: &[u8]) -> Packet {
        todo!();
    }
}
