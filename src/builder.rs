#![allow(dead_code)]
use crate::{
    closure_handler_wrapper::ClosureHandlerWrapper,
    dispatched_event::DispatchedEvent,
    event::{Dispatchable, EventHandler},
    event_dispatcher::{EventDispatcher, EVENT_DISPATCHER},
    event_listener::{EventListener, Subscriber, SubscriberList, LOG_TITLE},
};
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc::{self};

#[derive(Default)]
pub struct EventDispatcherBuilder {
    subscribers: SubscriberList,
}

impl EventDispatcherBuilder {
    pub fn new() -> Self {
        Self {
            subscribers: SubscriberList::new(),
        }
    }

    // TODO: complete implementation that will allow closure to be used as a handler
    fn listen_fn<F, Fut>(self, event: String, handler: F) -> Self
    where
        F: Fn(&DispatchedEvent) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send + Sync + 'static,
    {
        let wraper = ClosureHandlerWrapper(Some(handler));

        self.register(event, wraper.to_handler())
    }

    pub fn listen<E: Dispatchable, H: EventHandler + Default>(self) -> Self {
        let event = E::event();
        let the_handler = H::default().to_handler();
        self.register(event, the_handler)
    }

    pub fn subscribe(mut self, subscriber: Subscriber) -> Self {
        for name_and_handlers in subscriber.subscribers {
            println!("subscriber event: {:#}", &name_and_handlers.0);
            if let Some(collection) = self.subscribers.get_mut(&name_and_handlers.0) {
                collection.extend(name_and_handlers.1);
            } else {
                self.subscribers
                    .insert(name_and_handlers.0, name_and_handlers.1);
            }
        }
        self
    }

    pub async fn build(self) -> Arc<EventDispatcher> {
        let (tx, rx) = mpsc::unbounded_channel::<(String, DispatchedEvent)>();
        let subscribers = self.subscribers;

        tokio::spawn(async move {
            let mut handler = EventListener::new(subscribers, rx);
            handler.receive().await;
        });

        let dispatcher = Arc::new(EventDispatcher::new(tx));

        _ = EVENT_DISPATCHER.set(dispatcher.clone());

        dispatcher
    }

    fn register(mut self, event: String, handler: Box<dyn EventHandler>) -> Self {
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
