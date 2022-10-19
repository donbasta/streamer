pub mod test;

use bytes::{BufMut, BytesMut};

use crate::{constant::*, errors::PacketError};

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

    pub fn generate_head(&self) -> BytesMut {
        let mut head = BytesMut::new();
        head.put_u8(self.p_type.try_into().unwrap());
        head.put_u16(self.p_data.len().try_into().unwrap());
        head.put_u16(self.p_sequence.try_into().unwrap());

        head
    }

    pub fn generate_checksum(&self, head: &mut BytesMut) -> usize {
        let mut check_data = BytesMut::new();
        check_data.put(head);
        check_data.put(&*self.p_data);

        if check_data.len() % 2 != 0 {
            check_data.put(&b"\x00"[..]);
        }
        let mut checksum = 0usize;
        let mut iterator = check_data.iter();
        loop {
            let a = match iterator.next() {
                Some(&t) => t as usize,
                None => break,
            };
            let b = match iterator.next() {
                Some(&t) => t as usize,
                None => break,
            };
            let h = (a * 256 + b) as usize;
            checksum ^= h;
        }

        checksum
    }

    pub fn to_bytes(&self) -> BytesMut {
        let mut head = self.generate_head();
        let checksum = self.generate_checksum(&mut head);

        let mut packet = self.generate_head();

        packet.put_u16(checksum.try_into().unwrap());
        packet.put(&*self.p_data);

        packet
    }

    pub fn to_packet(raw: BytesMut) -> Result<Packet, PacketError> {
        assert!(raw.len() >= 7);

        let mut iterator = raw.iter();
        let p_type = match iterator.next() {
            Some(&t) => t as usize,
            None => 0,
        };

        let mut x = [0, 0, 0];

        for i in 0..3 {
            let a = match iterator.next() {
                Some(&t) => t as usize,
                None => 0,
            };
            let b = match iterator.next() {
                Some(&t) => t as usize,
                None => 0,
            };
            x[i] = a * 256 + b;
        }

        let [p_len, p_sequence, p_checksum] = x;
        let p_data = &raw[7..];

        if p_len != p_data.len() {
            return Err(PacketError::LengthError);
        }

        let buf = BytesMut::from(p_data);

        let packet = Packet {
            p_type,
            p_sequence,
            p_data: buf,
        };

        if p_checksum != packet.generate_checksum(&mut BytesMut::from(&raw[5..])) {
            return Err(PacketError::ChecksumError);
        }

        Ok(packet)
    }
}
