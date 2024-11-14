#![allow(dead_code)]
use crate::{
    dispatched_event::DispatchedEvent, event::Dispatchable, event_listener::call_event_handlers,
    EventDispatcherBuilder,
};
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) static EVENT_DISPATCHER: OnceLock<Arc<EventDispatcher>> = OnceLock::new();

#[derive(Debug)]
pub struct EventDispatcher {
    sender: UnboundedSender<DispatchedEvent>,
}

impl EventDispatcher {
    pub(crate) fn new(sender: UnboundedSender<DispatchedEvent>) -> Self {
        Self { sender }
    }

    /// Dispatches the event
    pub fn dispatch<T: Dispatchable>(&self, event: T) {
        let event = DispatchedEvent::new(serde_json::to_string(&event).unwrap(), T::event());
        _ = self.sender.send(event);
    }

    pub fn dispatch_str(&self, name: &str, event: impl Dispatchable) {
        let event = DispatchedEvent::new(serde_json::to_string(&event).unwrap(), name.to_string());
        _ = self.sender.send(event);
    }

    pub fn dispatch_json(&self, event: &str) {
        if let Ok(dispatched_event) = serde_json::from_str::<DispatchedEvent>(event) {
            _ = self.sender.send(dispatched_event);
        }
    }

    /// Dispatches the event in the current thread
    pub async fn dispatch_sync<T: Dispatchable + Send + Sync + 'static>(&self, event: T) {
        let event = DispatchedEvent::new(serde_json::to_string(&event).unwrap(), T::event());
        call_event_handlers(event).await;
    }
}

pub fn event_dispatcher() -> Arc<EventDispatcher> {
    if let Some(dispatcher) = EVENT_DISPATCHER.get() {
        dispatcher.clone()
    } else {
        futures::executor::block_on(EventDispatcherBuilder::new().build());
        event_dispatcher()
    }
}
