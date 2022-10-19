use bytes::{BufMut, BytesMut};

use crate::packet::item::Packet;

#[test]
fn test_head() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet::new(0usize, 2usize, &bytes_mut);

    assert_eq!(&b"\x00\x00\x04\x00\x02"[..], packet.generate_head());
}

#[test]
fn test_checksum() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet::new(0usize, 2usize, &bytes_mut);

    assert_eq!(2, packet.generate_checksum(&mut packet.generate_head()));
}

#[test]
fn test_to_bytes() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put(&b"abcd"[..]);
    let packet = Packet::new(0usize, 2usize, &bytes_mut);

    assert_eq!(
        BytesMut::from(&b"\x00\x00\x04\x00\x02\x00\x02abcd"[..]),
        packet.to_bytes()
    );
}

#[test]
fn test_to_packet() {
    let raw = BytesMut::from(&b"\x00\x00\x04\x00\x02\x00\x02abcd"[..]);
    let packet = Packet::to_packet(raw).unwrap();

    assert_eq!(0, packet.get_type());
    assert_eq!(2, packet.get_sequence());
    assert_eq!(&b"abcd"[..], packet.get_data());
}
