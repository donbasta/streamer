use crate::workers::{chunk_player_worker::ChunkPlayerWorker, downloader_worker::DownloaderWorker};

use super::config::ClientConfig;

pub struct Client {
    chunk_player_worker: ChunkPlayerWorker,
    downloader_worker: DownloaderWorker,
    config: ClientConfig,
}

impl Client {
    pub fn from(config: ClientConfig) -> Client {
        let chunk_player_worker = ChunkPlayerWorker::create();
        let downloader_worker = DownloaderWorker::create();
        Client {
            chunk_player_worker,
            downloader_worker,
            config,
        }
    }

    pub fn run() {}
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!()
    }
}
