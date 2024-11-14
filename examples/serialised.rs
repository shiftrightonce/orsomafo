use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Register event handler
    UserCreated::subscribe::<HandleUserCreated>().await;

    // 2. Create event instance
    let event = UserCreated { id: 45 };

    // 3. We are pretending a serialized version of this event
    //    came "over the wire"
    let event_string = event.serialize_event();

    // - dump of the raw json
    println!("event's json dump: {:#?}", event_string);

    // 4. Build an instance of the event dispatcher
    let dispatcher = EventDispatcherBuilder::new().build().await;

    // 5. Dispatch a json version of the event
    dispatcher.dispatch_json(&event_string);
}

// 6. Event to dispatch
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct UserCreated {
    id: u32,
}

impl Dispatchable for UserCreated {}

// 7. Event handler
#[derive(Default)]
struct HandleUserCreated;

#[async_trait]
impl EventHandler for HandleUserCreated {
    async fn handle(&self, dispatched: DispatchedEvent) {
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("we are handling user created event: {:?}", event.id)
    }
}
