#![allow(dead_code)]
use std::any::Any;

#[derive(Debug)]
pub struct DispatchedEvent(Box<dyn Any + Send + Sync + 'static>);

impl DispatchedEvent {
    pub(crate) fn new(inner: Box<dyn Any + Send + Sync + 'static>) -> Self {
        Self(inner)
    }

    /// Returns the actual instance of the event
    /// ```
    /// # use async_trait::async_trait;
    /// # use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
    /// # use tokio::time::{sleep, Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #  _ =  EventDispatcherBuilder::new().build().await;
    ///
    ///    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    ///    struct MyEvent;
    ///    impl Dispatchable for MyEvent {}
    ///
    ///    struct MyEventHandler;
    ///    
    ///    #[orsomafo::async_trait]
    ///    impl EventHandler for MyEventHandler {
    ///        async fn handle(&self, dispatched: &DispatchedEvent)  {
    ///           let event: MyEvent = dispatched.the_event().unwrap();
    ///           // or
    ///           // let event = dispatched.the_event::<MyEvent>().unwrap()
    ///           //...
    ///        }
    ///    }
    ///
    /// }
    /// ```
    pub fn the_event<T: Clone + 'static>(&self) -> Option<T> {
        let result: Option<&T> = self.0.downcast_ref();
        result.cloned()
    }
}
