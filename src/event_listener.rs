#![allow(dead_code)]
use crate::{
    closure_handler_wrapper::ClosureHandlerWrapper,
    dispatched_event::DispatchedEvent,
    event::{Dispatchable, EventHandler},
};
use std::{collections::HashMap, future::Future, sync::OnceLock};
use tokio::sync::{mpsc::UnboundedReceiver, RwLock};

pub(crate) const LOG_TITLE: &str = "orsomafo";
pub(crate) type SubscriberList = HashMap<String, Vec<Box<dyn EventHandler>>>;

// List of registered subscribers/listeners
static REGISTERED_SUBSCRIBERS: OnceLock<RwLock<SubscriberList>> = OnceLock::new();

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
    fn listen_fn<E: Dispatchable, F, Fut>(self, handler: F) -> Self
    where
        F: Fn(&DispatchedEvent) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send + Sync + 'static,
    {
        let wrapper = ClosureHandlerWrapper(Some(handler));
        let event = E::event();

        self.register(event, wrapper.to_handler())
    }

    pub fn listen<E: Dispatchable, H: EventHandler + Default>(self) -> Self {
        let event = E::event();
        let handler = H::default().to_handler();

        self.register(event, handler)
    }

    fn register(mut self, event_name: String, handler: Box<dyn EventHandler>) -> Self {
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

    /// Apply listeners to the event listeners queue
    pub async fn build(self) {
        crate::setup().await;
        merge_subscribers(self.subscribers).await;
    }
}

pub(crate) struct EventListener {
    chan_rev: UnboundedReceiver<(String, DispatchedEvent)>,
}

impl EventListener {
    pub async fn new(
        subscribers: SubscriberList,
        receiver: UnboundedReceiver<(String, DispatchedEvent)>,
    ) -> Self {
        merge_subscribers(subscribers).await;
        Self { chan_rev: receiver }
    }

    pub async fn receive(&mut self) {
        while let Some(event) = self.chan_rev.recv().await {
            log::trace!(
                target: LOG_TITLE,
                "received dispatched event: {:?}",
                &event.0
            );

            if let Some(lock) = REGISTERED_SUBSCRIBERS.get() {
                let mut list = lock.write().await;
                if let Some(subscribers) = list.get_mut(&event.0) {
                    let mut to_remove = Vec::new();
                    for a_subscriber in subscribers.iter().enumerate() {
                        log::trace!(
                            target: LOG_TITLE,
                            "calling handler: {:?}, for event: {:?}",
                            &a_subscriber.1.handler_id(),
                            &event.0
                        );

                        a_subscriber.1.handle(&event.1).await;
                        if a_subscriber.1.execute_once() {
                            to_remove.push(a_subscriber.0);
                        }

                        if a_subscriber.1.propagate() == false {
                            break;
                        }
                    }

                    if to_remove.is_empty() == false {
                        for index in to_remove.into_iter() {
                            subscribers.remove(index);
                        }
                    }
                }
            }
        }
    }
}

pub(crate) async fn merge_subscribers(subscribers: SubscriberList) {
    let lock = REGISTERED_SUBSCRIBERS.get_or_init(|| RwLock::new(SubscriberList::new()));
    let mut list = lock.write().await;
    for entry in subscribers {
        if !list.contains_key(&entry.0) {
            list.insert(entry.0.clone(), Vec::new());
        }

        list.get_mut(&entry.0).unwrap().extend(entry.1);
    }
}

mod test {
    use async_trait::async_trait;

    use super::*;

    #[tokio::test]
    async fn test_subscribers_merging() {
        Subscriber::new()
            .listen::<UserCreated, HandleUserCreated>()
            .build()
            .await;

        Subscriber::new()
            .listen::<UserCreated, HandleUserCreated>()
            .build()
            .await;

        let subscribers = REGISTERED_SUBSCRIBERS.get();
        assert_eq!(subscribers.is_some(), true);
    }

    #[derive(Clone)]
    struct UserCreated {
        id: u32,
    }

    impl Dispatchable for UserCreated {}

    #[derive(Default)]
    struct HandleUserCreated;

    #[async_trait]
    impl EventHandler for HandleUserCreated {
        async fn handle(&self, dispatched: &DispatchedEvent) {
            let the_event = dispatched.the_event();

            assert_eq!(the_event.is_none(), true);

            let event: UserCreated = the_event.unwrap();
            assert_eq!(event.id, 200);
        }
    }
}
