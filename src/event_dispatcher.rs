use tokio::sync::mpsc::UnboundedSender;

use crate::event::{Dispatchable, DispatchedEvent};

#[derive(Debug)]
pub struct EventDispatcher {
    sender: UnboundedSender<(String, DispatchedEvent)>,
}

impl EventDispatcher {
    pub fn new(sender: UnboundedSender<(String, DispatchedEvent)>) -> Self {
        Self { sender }
    }

    pub fn dispatch<T: Dispatchable + Send + Sync + 'static>(&self, event: T) {
        _ = self
            .sender
            .send((T::event(), DispatchedEvent::new(Box::new(event))));
    }
}
