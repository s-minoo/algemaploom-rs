use std::sync::Arc;

use tokio::sync::mpsc::{self};

#[derive(Debug)]
pub struct Channel<T> {
    pub sender:   mpsc::Sender<T>,
    pub receiver: mpsc::Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new_arc() -> Arc<Channel<T>> {
        Channel::<T>::new().to_arc()
    }

    pub fn new() -> Channel<T> {
        let (sender, receiver) = mpsc::channel(256);
        Channel { receiver, sender }
    }

    pub fn to_arc(self) -> Arc<Channel<T>> {
        Arc::new(self)
    }
}
