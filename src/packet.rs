use bytes::{BufMut, BytesMut};

use crate::constant::*;

pub struct Packet {
    p_type: usize,
    p_sequence: usize,
    p_data: BytesMut,
}

impl Packet {
    pub fn new(p_type: usize, p_sequence: usize, p_data: &[u8]) -> Packet {
        assert!(AVAILABLE_TYPES.contains(&p_type));
        assert!(p_data.len() <= MAX_LENGTH);
        assert!(p_sequence <= MAX_SEQUENCE);

        if [TYPE_ACK, TYPE_FINACK].contains(&p_type) {
            assert!(p_data.len() == 0);
        }

        let mut data = BytesMut::new();
        data.put(p_data);

        Packet {
            p_type,
            p_sequence,
            p_data: data,
        }
    }

    pub fn generate_checksum(&self, head: &mut BytesMut) -> usize {
        let mut check_data = BytesMut::new();
        check_data.put(head);
        check_data.put(&*self.p_data);

        if check_data.len() % 2 != 0 {
            check_data.put(&b"\x00"[..]);
        }
        let checksum = 0usize;

        checksum
    }

    pub fn to_bytes(&self) -> BytesMut {
        let mut head = BytesMut::new();
        head.put_u8(self.p_type.try_into().unwrap());
        head.put_u16(self.p_data.len().try_into().unwrap());
        head.put_u16(self.p_sequence.try_into().unwrap());

        let checksum = self.generate_checksum(&mut head);
        let mut packet = head;

        packet.put_u16(checksum.try_into().unwrap());
        packet.put(&*self.p_data);

        packet
    }

    pub fn to_packet(raw: BytesMut) -> Packet {
        assert!(raw.len() >= 7);

        Packet {
            p_type: 0,
            p_sequence: 0,
            p_data: BytesMut::new(),
        }
    }
}
