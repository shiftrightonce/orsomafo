#![allow(dead_code)]
use crate::{dispatched_event::DispatchedEvent, event::Dispatchable};
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) static EVENT_DISPATCHER: OnceLock<Arc<EventDispatcher>> = OnceLock::new();

#[derive(Debug)]
pub struct EventDispatcher {
    sender: UnboundedSender<(String, DispatchedEvent)>,
}

impl EventDispatcher {
    pub(crate) fn new(sender: UnboundedSender<(String, DispatchedEvent)>) -> Self {
        Self { sender }
    }

    /// Dispatches the event
    pub fn dispatch<T: Dispatchable + Send + Sync + 'static>(&self, event: T) {
        _ = self.sender.send((
            T::event(),
            DispatchedEvent::new(serde_json::to_string(&event).unwrap(), T::event()),
        ));
    }

    pub fn dispatch_json(&self, event: &str) {
        if let Ok(dispatched_event) = serde_json::from_str::<DispatchedEvent>(event) {
            dbg!(&dispatched_event);
            _ = self
                .sender
                .send((dispatched_event.event(), dispatched_event));
        }
    }
}

pub fn event_dispatcher() -> Arc<EventDispatcher> {
    if let Some(dispatcher) = EVENT_DISPATCHER.get() {
        return dispatcher.clone();
    }

    panic!("event dispatcher is not ready")
}
