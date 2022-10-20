use crate::workers::{chunk_player_worker::ChunkPlayerWorker, downloader_worker::DownloaderWorker};

struct Client {
    chunk_player_worker: ChunkPlayerWorker,
    downloader_worker: DownloaderWorker,
}

impl Client {
    pub fn create() -> Client {
        let chunk_player_worker = ChunkPlayerWorker::create();
        let downloader_worker = DownloaderWorker::create();
        Client {
            chunk_player_worker,
            downloader_worker,
        }
    }

    pub fn run() {}
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!()
    }
}
