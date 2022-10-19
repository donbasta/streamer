use crate::wav_handler::WavHandler;

pub struct ChunkSenderWorker<'a> {
    server_address: String,
    chunk: &'a [u8],
    fin: bool,
    meta: bool,
    wav: WavHandler,
}

impl ChunkSenderWorker<'_> {
    pub fn create() {}
}
