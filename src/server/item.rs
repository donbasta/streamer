use crate::workers::{
    streamer_worker::StreamerWorker, subscriber_listener_worker::SubscriberListenerWorker,
};

pub struct Server {
    streamer_worker: StreamerWorker,
    subscriber_listener_worker: SubscriberListenerWorker,
}

impl Server {
    pub fn create() -> Server {
        let streamer_worker = StreamerWorker::create();
        let subscriber_listener_worker = SubscriberListenerWorker::create();
        Server {
            streamer_worker,
            subscriber_listener_worker,
        }
    }

    pub fn run() {}
}

impl Drop for Server {
    fn drop(&mut self) {
        todo!()
    }
}
