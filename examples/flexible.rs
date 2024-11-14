use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventHandler};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Subscribe directly to the event using the event's static `subscribe` method
    UserCreated::subscribe::<HandleUserCreated>().await;

    // 1.b You can subscribe with an instance of your handler
    UserCreated::subscribe_with(HandleUserCreated).await;

    // 2. Somewhere in your code create an instance of your event is dispatch it
    let event = UserCreated { id: 33 };
    event.dispatch_event();

    // Example of multiple instances of the same event being dispatched
    let events = vec![
        UserCreated { id: 1 },
        UserCreated { id: 2 },
        UserCreated { id: 3 },
    ];

    events.into_iter().for_each(|an_event| {
        an_event.dispatch_event();
    });
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
        println!("we are handling user created event: {:?}", event.id)
    }
}
