#![allow(dead_code)]
use crate::{
    dispatched_event::DispatchedEvent,
    event::{Dispatchable, EventHandler},
};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;

pub(crate) const LOG_TITLE: &str = "orsomafo";
pub(crate) type SubscriberList = HashMap<String, Vec<Box<dyn EventHandler>>>;

pub struct Subscriber {
    pub(crate) subscribers: SubscriberList,
}

impl Subscriber {
    pub fn new() -> Self {
        Self {
            subscribers: SubscriberList::new(),
        }
    }

    pub fn listen<E: Dispatchable>(&mut self, handler: Box<dyn EventHandler>) -> &mut Self {
        let event = E::event();

        if self.subscribers.get(&event).is_none() {
            self.subscribers.insert(event.clone(), Vec::new());
        }

        if let Some(collection) = self.subscribers.get_mut(&event) {
            log::trace!(
                target: LOG_TITLE,
                "registered handler: {:?}, for event: {:?}",
                &event,
                &handler.handler_id()
            );

            collection.push(handler);
        } else {
            log::error!(
                "could not register handler: {:?}, for event: {:?}",
                &event,
                &handler.handler_id()
            );
        }

        self
    }
}

pub(crate) struct EventListener {
    chan_rev: UnboundedReceiver<(String, DispatchedEvent)>,
    subscribers: SubscriberList,
}

impl EventListener {
    pub fn new(
        subscribers: SubscriberList,
        receiver: UnboundedReceiver<(String, DispatchedEvent)>,
    ) -> Self {
        Self {
            chan_rev: receiver,
            subscribers,
        }
    }

    pub async fn receive(&mut self) {
        while let Some(event) = self.chan_rev.recv().await {
            log::trace!(
                target: LOG_TITLE,
                "received dispatched event: {:?}",
                &event.0
            );

            if let Some(subscribers) = self.subscribers.get(&event.0) {
                for a_subscriber in subscribers {
                    log::trace!(
                        target: LOG_TITLE,
                        "calling handler: {:?}, for event: {:?}",
                        &a_subscriber.handler_id(),
                        &event.0
                    );

                    a_subscriber.handle(&event.1).await
                }
            }
        }
    }
}
