use crate::{
    dispatched_event::DispatchedEvent,
    event_dispatcher::event_dispatcher,
    event_listener::{merge_subscribers, SubscriberList, LOG_TITLE},
};
use async_trait::async_trait;

/// Types that are dispatchable must implement this trait
#[async_trait]
pub trait Dispatchable:
    serde::Serialize + serde::de::DeserializeOwned + Clone + Send + Sync
{
    /// By default the name of the type is used as the event name
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

    fn supports_cluster(&self) -> bool {
        true
    }

    fn serialize_event(&self) -> String {
        let event = DispatchedEvent::new(serde_json::to_string(self).unwrap(), Self::event());

        serde_json::to_string(&event).unwrap()
    }

    async fn subscribe<H: EventHandler + Default>()
    where
        Self: Sized,
    {
        crate::setup().await;

        let event = Self::event();
        let the_handler = H::default().to_handler();

        let mut subscriber = SubscriberList::new();

        log::trace!(
            target: LOG_TITLE,
            "registered handler: {:?}, for event: {:?}",
            &event,
            &the_handler.handler_id()
        );

        subscriber.insert(event, vec![the_handler]);
        merge_subscribers(subscriber).await;
    }
}

/// Event handler must implement this trait
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// The "handle" method will be called when an event is ready
    /// ```
    /// # use async_trait::async_trait;
    /// # use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
    /// # use tokio::time::{sleep, Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #  _ =  EventDispatcherBuilder::new().build().await;
    ///
    ///    struct MyEventHandler;
    ///    
    ///    #[orsomafo::async_trait]
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

    /// Executes this handler once and dequeue it if `true` is returned
    fn execute_once(&self) -> bool {
        false
    }

    /// Stops propagating the event to other handlers when `false` is returned
    fn propagate(&self) -> bool {
        true
    }
}

mod test {
    use super::*;

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct UserCreated {
        id: u32,
    }

    impl Dispatchable for UserCreated {}

    #[tokio::test]
    async fn test_event_dispatching() {
        UserCreated::subscribe::<HandleUserCreated>().await;
        event_dispatcher()
            .dispatch_sync(UserCreated { id: 200 })
            .await;
    }

    #[derive(Default)]
    struct HandleUserCreated;

    #[async_trait]
    impl EventHandler for HandleUserCreated {
        async fn handle(&self, dispatched: &DispatchedEvent) {
            let the_event = dispatched.the_event();

            assert_eq!(the_event.is_none(), false);

            let event: UserCreated = the_event.unwrap();
            assert_eq!(event.id, 200);
        }
    }
}
