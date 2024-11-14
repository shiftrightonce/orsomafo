use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventHandler};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    UserCreated::subscribe::<HandleUserCreated>().await;
    UserCreated::subscribe::<HandleUserCreated2>().await; // this handler will never get called

    let event = UserCreated { id: 8701 };
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
    async fn handle(&self, dispatched: DispatchedEvent) {
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("User with ID: {} created", event.id);
    }

    // Prevent event propagating
    fn propagate(&self) -> bool {
        false // true by default
    }
}

#[derive(Default)]
struct HandleUserCreated2;

#[async_trait]
impl EventHandler for HandleUserCreated2 {
    async fn handle(&self, dispatched: DispatchedEvent) {
        println!("Handling event with ID: {}", dispatched.id());
    }
}
