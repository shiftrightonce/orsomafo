use std::collections::HashMap;

use crate::{
    event::{Dispatchable, DispatchedEvent, EventHandler},
    event_dispatcher::EventDispatcher,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

type SubscriberList = HashMap<String, Vec<Box<dyn EventHandler>>>;

pub struct Subscriber {
    subscribers: SubscriberList,
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
            collection.push(handler);
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

    // pub fn listen<E: Dispatchable>(&mut self, handler: Box<dyn EventHandler>) -> &mut Self {
    //     let event = E::event();

    //     if self.subscribers.get(&event).is_none() {
    //         self.subscribers.insert(event.clone(), Vec::new());
    //     }

    //     if let Some(collection) = self.subscribers.get_mut(&event) {
    //         collection.push(handler);
    //     }

    //     self
    // }

    pub async fn receive(&mut self) {
        while let Some(event) = self.chan_rev.recv().await {
            println!("event: \"{:?}\"", &event.1,);
            if let Some(subscribers) = self.subscribers.get(&event.0) {
                for a_subscriber in subscribers {
                    a_subscriber.handle(&event.1).await
                }
            }
        }
    }
}

pub struct EventDispatcherBuilder {
    subscribers: SubscriberList,
}

impl EventDispatcherBuilder {
    pub fn new() -> Self {
        Self {
            subscribers: SubscriberList::new(),
        }
    }

    pub fn listen<E: Dispatchable>(mut self, handler: Box<dyn EventHandler>) -> Self {
        let event = E::event();

        if self.subscribers.get(&event).is_none() {
            self.subscribers.insert(event.clone(), Vec::new());
        }

        if let Some(collection) = self.subscribers.get_mut(&event) {
            collection.push(handler);
        }
        self
    }

    pub fn subscribe(mut self, subscriber: Subscriber) -> Self {
        for name_and_handlers in subscriber.subscribers {
            if let Some(collection) = self.subscribers.get_mut(&name_and_handlers.0) {
                collection.extend(name_and_handlers.1);
            }
        }
        self
    }

    pub async fn build(self) -> EventDispatcher {
        let (tx, rx) = mpsc::unbounded_channel::<(String, DispatchedEvent)>();
        let subscribers = self.subscribers;

        tokio::spawn(async move {
            let mut handler = EventListener::new(subscribers, rx);
            handler.receive().await;
        });

        EventDispatcher::new(tx)
    }
}
