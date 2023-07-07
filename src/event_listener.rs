#![allow(dead_code)]
use crate::{
    closure_handler_wrapper::ClosureHandlerWrapper,
    dispatched_event::DispatchedEvent,
    event::{Dispatchable, EventHandler},
};
use std::{collections::HashMap, future::Future};
use tokio::sync::mpsc::UnboundedReceiver;

pub(crate) const LOG_TITLE: &str = "orsomafo";
pub(crate) type SubscriberList = HashMap<String, Vec<Box<dyn EventHandler>>>;

#[derive(Default)]
pub struct Subscriber {
    pub(crate) subscribers: SubscriberList,
}

impl Subscriber {
    pub fn new() -> Self {
        Self {
            subscribers: SubscriberList::new(),
        }
    }

    // TODO: complete implementation that will allow closure to be used as a handler
    fn listen_fn<E: Dispatchable, F, Fut>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&DispatchedEvent) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send + Sync + 'static,
    {
        let wraper = ClosureHandlerWrapper(Some(handler));
        let event = E::event();

        self.register(event, wraper.to_handler())
    }

    pub fn listen<E: Dispatchable, H: EventHandler + Default>(&mut self) -> &mut Self {
        let event = E::event();
        let handler = H::default().to_handler();

        self.register(event, handler)
    }

    fn register(&mut self, event_name: String, handler: Box<dyn EventHandler>) -> &mut Self {
        if self.subscribers.get(&event_name).is_none() {
            self.subscribers.insert(event_name.clone(), Vec::new());
        }

        if let Some(collection) = self.subscribers.get_mut(&event_name) {
            log::trace!(
                target: LOG_TITLE,
                "registered handler: {:?}, for event: {:?}",
                &event_name,
                &handler.handler_id()
            );

            collection.push(handler);
        } else {
            log::error!(
                "could not register handler: {:?}, for event: {:?}",
                &event_name,
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
