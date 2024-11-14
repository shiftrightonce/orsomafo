#![allow(dead_code)]
use crate::{
    closure_handler_wrapper::ClosureHandlerWrapper,
    dispatched_event::DispatchedEvent,
    event::{Dispatchable, EventHandler},
    event_dispatcher::{EventDispatcher, EVENT_DISPATCHER},
    event_listener::{merge_subscribers, EventListener, Subscriber, SubscriberList, LOG_TITLE},
};
use futures::future::BoxFuture;
use std::sync::Arc;
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
    pub fn listen_fn<E: Dispatchable>(
        self,
        handler: impl Fn(DispatchedEvent) -> BoxFuture<'static, ()> + Send + Sync + 'static,
    ) -> Self {
        let wrapper = ClosureHandlerWrapper(handler);

        self.register(E::event(), wrapper.to_handler())
    }

    pub fn listen_str_fn<F>(self, event: &str, handler: F) -> Self
    where
        F: Fn(DispatchedEvent) -> BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        let wrapper = ClosureHandlerWrapper(handler);

        self.register(event.to_string(), wrapper.to_handler())
    }

    pub fn listen<E: Dispatchable, H: EventHandler + Default>(self) -> Self {
        let event = E::event();
        let the_handler = H::default().to_handler();
        self.register(event, the_handler)
    }

    pub fn listen_str<H: EventHandler + Default>(self, event: &str) -> Self {
        let the_handler = H::default().to_handler();
        self.register(event.to_string(), the_handler)
    }

    pub fn listen_with<E: Dispatchable>(self, instance: impl EventHandler) -> Self {
        let event = E::event();
        let the_handler = instance.to_handler();
        self.register(event, the_handler)
    }

    pub fn listen_str_with(self, event: &str, instance: impl EventHandler) -> Self {
        let the_handler = instance.to_handler();
        self.register(event.to_string(), the_handler)
    }

    pub fn subscribe(mut self, subscriber: Subscriber) -> Self {
        for name_and_handlers in subscriber.subscribers {
            log::trace!(
                target: LOG_TITLE,
                "subscriber event: {:?}",
                &name_and_handlers.0,
            );

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
        if let Some(dispatcher) = EVENT_DISPATCHER.get() {
            let subscribers = self.subscribers;
            merge_subscribers(subscribers).await;
            dispatcher.clone()
        } else {
            let (tx, rx) = mpsc::unbounded_channel::<DispatchedEvent>();
            let subscribers = self.subscribers;

            tokio::spawn(async move {
                let mut handler = EventListener::new(subscribers, rx).await;
                handler.receive().await;
            });

            let dispatcher = Arc::new(EventDispatcher::new(tx));

            _ = EVENT_DISPATCHER.set(dispatcher.clone());

            dispatcher
        }
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
