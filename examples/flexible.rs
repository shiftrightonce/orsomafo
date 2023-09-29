use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventHandler};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Subscribe directly to the event using the event's static `subscribe` method
    UserCreated::subscribe::<HandleUserCreated>().await;

    // 2. Somewhere in your code create an instance of your event is dispatch it
    let user = UserCreated { id: 33 };
    user.dispatch_event();

    // Example of multiple instances of the same event being dispatched
    let users = vec![
        UserCreated { id: 1 },
        UserCreated { id: 2 },
        UserCreated { id: 3 },
    ];

    users.into_iter().for_each(|user| {
        user.dispatch_event();
    });
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
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("we are handling user created event: {:?}", event.id)
    }
}
