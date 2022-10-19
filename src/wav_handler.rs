struct WavHandler {
    fpath: String,
    max_chunk_size: usize,
}

impl WavHandler {
    pub fn build(fpath: &str) -> WavHandler {
        let mut inp_file = File::open(Path::new(path))?;
        let (header, data) = wav::read(&mut inp_file)?;
    }
}
