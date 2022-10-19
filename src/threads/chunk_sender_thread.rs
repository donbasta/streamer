struct ChunkSenderThread {
    server_address: String,
    chunk: Vec<[u8]>,
    fin: bool,
    meta: bool,
    wav: WavHandler,
}
