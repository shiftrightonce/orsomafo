use crate::{dispatched_event::DispatchedEvent, event_dispatcher::event_dispatcher};
use async_trait::async_trait;

/// Types that are dispatchable must implement this trait
pub trait Dispatchable: Send + Sync {
    /// By default name of the type is used as the event name
    /// It is recommended to leave this as it is if you don't
    /// have any good reason to change it.
    fn event() -> String
    where
        Self: Sized,
    {
        std::any::type_name::<Self>().to_string()
    }

    /// Call this method when you are ready to dispatch the event
    /// ```
    /// # use async_trait::async_trait;
    /// # use soma::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
    /// # use tokio::time::{sleep, Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #  _ =  EventDispatcherBuilder::new().build().await;
    ///
    ///    #[derive(Clone)]
    ///    struct MyEvent;
    ///    impl Dispatchable for MyEvent {}
    ///
    ///    let e = MyEvent;
    ///    e.dispatch_event();
    /// }
    /// ```
    ///
    fn dispatch_event(self)
    where
        Self: Sized + 'static,
    {
        event_dispatcher().dispatch(self);
    }
}

/// Event handler must implement this trait
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// The "handle" method will be called when an event is ready
    /// ```
    /// # use async_trait::async_trait;
    /// # use soma::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
    /// # use tokio::time::{sleep, Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #  _ =  EventDispatcherBuilder::new().build().await;
    ///
    ///    struct MyEventHandler;
    ///    
    ///    #[soma::async_trait]
    ///    impl EventHandler for MyEventHandler {
    ///        async fn handle(&self, event: &DispatchedEvent)  {
    ///           //...
    ///        }
    ///    }
    ///
    /// }
    /// ```
    async fn handle(&self, event: &DispatchedEvent);

    fn to_handler(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    /// The identification of this handler
    /// It is recommended to leave this as it is
    fn handler_id(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }
}
