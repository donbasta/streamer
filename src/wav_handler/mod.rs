pub struct WavHandler {
    fpath: String,
    max_chunk_size: usize,
}

impl WavHandler {
    pub fn build(fpath: &str) -> WavHandler {
        WavHandler {
            fpath: String::from(fpath),
            max_chunk_size: 1024,
        }
    }

    pub fn play(&self) {
        todo!();
    }

    pub fn get_metadata(raw: &[u8]) {
        todo!();
    }

    pub fn get_chunks(raw: &[u8]) {
        todo!();
    }
}
