use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Subscribe to the `NumberAdded` event
    NumberAdded::subscribe::<HandleNumberAddedEvent>().await;

    // 2. Somewhere down the line, we unsubscribe
    NumberAdded::unsubscribe::<HandleNumberAddedEvent>().await;

    // 3. An instance of the `NumberAdded` event is dispatched
    // This event will not be handled
    NumberAdded {
        number_one: 55,
        number_two: 100,
    }
    .dispatch_event();

    // The following line is use to pause the application for
    // few milliseconds. This will allow us to handle all dispatched events.
    // In a full application, this line wil not be require.
    sleep(Duration::from_millis(100)).await;
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone)]
struct NumberAdded {
    number_one: u32,
    number_two: u32,
}

#[async_trait]
impl orsomafo::Dispatchable for NumberAdded {}

#[derive(Debug, Default)]
struct HandleNumberAddedEvent;

#[async_trait]
impl orsomafo::EventHandler for HandleNumberAddedEvent {
    async fn handle(&self, event: DispatchedEvent) {
        eprintln!(
            "-----> You shouldn't see this message for event: {}",
            event.name()
        );
    }
}
