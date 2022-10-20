use bytes::BytesMut;

pub trait Metadata {}

pub trait Media {
    fn open(&self);
    fn get_metadata(&self) -> Box<dyn Metadata>;
    fn get_chunks(&self) -> Vec<BytesMut>;
}
