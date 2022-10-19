use bytes::{BufMut, BytesMut};

use crate::packet::Packet;

#[test]
fn test_head() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet {
        p_type: 0usize,
        p_sequence: 2usize,
        p_data: bytes_mut,
    };

    assert_eq!(&b"\x00\x00\x04\x00\x02"[..], packet.generate_head());
}

#[test]
fn test_checksum() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet {
        p_type: 0usize,
        p_sequence: 2usize,
        p_data: bytes_mut,
    };

    assert_eq!(2, packet.generate_checksum(&mut packet.generate_head()));
}

#[test]
fn test_to_bytes() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet {
        p_type: 0usize,
        p_sequence: 2usize,
        p_data: bytes_mut,
    };

    assert_eq!(
        BytesMut::from(&b"\x00\x00\x04\x00\x02\x00\x02abcd"[..]),
        packet.to_bytes()
    );
}

#[test]
fn test_to_packet() {
    let raw = BytesMut::from(&b"\x00\x00\x04\x00\x02\x00\x02abcd"[..]);
    let packet = Packet::to_packet(raw).unwrap();

    assert_eq!(0, packet.p_type);
    assert_eq!(2, packet.p_sequence);
    assert_eq!(&b"abcd"[..], packet.p_data);
}
