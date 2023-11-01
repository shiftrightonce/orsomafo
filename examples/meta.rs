use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventHandler};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    UserCreated::subscribe::<HandleUserCreated>().await;

    let event = UserCreated { id: 34343464 };
    event.dispatch_event();

    // The following line is use to pause the application for
    // few milliseconds. This will allow us to handle all dispatched events.
    // In a full application, this line wil not be require.
    sleep(Duration::from_millis(100)).await;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct UserCreated {
    id: u32,
}

impl Dispatchable for UserCreated {}

#[derive(Default)]
struct HandleUserCreated;

#[async_trait]
impl EventHandler for HandleUserCreated {
    async fn handle(&self, dispatched: &DispatchedEvent) {
        println!("event id: {:?}", dispatched.id());
        println!("created at: {:?}", dispatched.created_at());
    }
}
