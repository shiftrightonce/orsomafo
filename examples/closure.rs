use orsomafo::{Dispatchable, EventDispatcherBuilder};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Use a closure to subscribe directly to the event
    HeartBeatLogged::subscribe_fn(|event| {
        Box::pin(async move { println!("'subscribe_with' handler: {:?}", event.data()) })
    })
    .await;

    // 2. Using closures on the builder
    EventDispatcherBuilder::new()
        .listen_fn::<HeartBeatLogged>(|event| {
            Box::pin(async move { println!("'listen_fn' handler: {:?}", event.data()) })
        })
        .listen_str_fn(&HeartBeatLogged::event(), |event| {
            Box::pin(async move { println!("'listen_str_fn' handler: {:?}", event.data()) })
        })
        .build()
        .await;

    // 3. Dispatch event
    HeartBeatLogged(10_000).dispatch_event();

    // The following line is use to pause the application for
    // few milliseconds. This will allow us to handle all dispatched events.
    // In a full application, this line wil not be require.
    sleep(Duration::from_millis(100)).await;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct HeartBeatLogged(u64);

impl Dispatchable for HeartBeatLogged {}
