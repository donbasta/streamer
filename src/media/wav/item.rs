use std::{fs::File, intrinsics::ceilf64, io::BufReader};

use bytes::{BufMut, BytesMut};
use hound::{WavReader, WavSpec};
use math::round::ceil;

use crate::constant::{CHUNK_SIZE, MAX_PACKET_LENGTH};

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
        let n_frames = reader.len() as usize;
        let mut itr = 0;
        let upper = ceil((n_frames as f64) / (MAX_PACKET_LENGTH as f64), 0) as usize;

        let chunks_bytes = reader.samples();

        while itr < upper {
            let chunk_size = MAX_PACKET_LENGTH;
            let start = itr * chunk_size + 44;
            let mut end: usize = 0;
            let mut chunk = BytesMut::new();

            if (itr + 1) * chunk_size + 44 > n_frames {
                end = n_frames;
            } else {
                end = (itr + 1) * chunk_size + 44
            }

            for _ in start..end {
                let a = match chunks_bytes.next() {
                    Some(x) => x,
                    None => panic!("frame size count doesn't match"),
                };
                chunk.put(a);
            }

            println!("Size of chunk {} = {}", itr, chunks[itr].len());
            itr += 1;
        }

        println!("Number of of Chunks: {}", chunks.len());
        println!("---- FINISH MAKING CHUNKS ----");

        chunks
    }
}
