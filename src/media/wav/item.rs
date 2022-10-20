use std::{fs::File, io::BufReader};

use bytes::BytesMut;
use hound::{WavReader, WavSpec};

use crate::constant::CHUNK_SIZE;

pub struct WavHandler {
    fpath: String,
    max_chunk_size: usize,
    spec: WavSpec,
    chunks: Vec<BytesMut>,
}

impl WavHandler {
    pub fn build(fpath: &str) -> WavHandler {
        let mut reader = hound::WavReader::open(fpath).unwrap();

        let spec = reader.spec();

        let chunks = WavHandler::get_chunks(&reader);

        WavHandler {
            fpath: String::from(fpath),
            max_chunk_size: 1024,
            spec,
            chunks,
        }
    }

    pub fn play(&self) {
        todo!();
    }

    pub fn get_metadata(&self) -> WavSpec {
        self.spec
    }

    pub fn get_chunks(reader: &WavReader<BufReader<File>>) -> Vec<BytesMut> {
        let n_channels = reader.spec().channels as usize;
        let sample_width = reader.spec().bits_per_sample as usize;
        let frame_rate = reader.spec().sample_rate as usize;

        let frame_size = n_channels * sample_width;
        let frame_count_per_chunk = CHUNK_SIZE / frame_size;

        let chunk_time = 1000 * frame_count_per_chunk / frame_rate;

        let chunks: Vec<BytesMut> = vec![];

        chunks
    }
}
