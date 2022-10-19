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
}
