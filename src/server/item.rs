use crate::server::config::ServerConfig;
use crate::workers::{
    streamer_worker::StreamerWorker, subscriber_listener_worker::SubscriberListenerWorker,
};

pub struct Server {
    streamer_worker: StreamerWorker,
    subscriber_listener_worker: SubscriberListenerWorker,
    config: ServerConfig,
}

impl Server {
    pub fn from(config: ServerConfig) -> Server {
        let streamer_worker = StreamerWorker::create();
        let subscriber_listener_worker = SubscriberListenerWorker::create();
        Server {
            streamer_worker,
            subscriber_listener_worker,
            config,
        }
    }

    pub fn load_source() {
        todo!()
    }

    pub fn run() {
        todo!()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        todo!()
    }
}
