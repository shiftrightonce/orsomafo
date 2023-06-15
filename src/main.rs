use crate::event_listener::{EventDispatcherBuilder, Subscriber};
use async_trait::async_trait;
use event::{Dispatchable, DispatchedEvent, EventHandler};

mod event;
mod event_dispatcher;
mod event_listener;

#[tokio::main]
async fn main() {
    let mut handlers = Subscriber::new();
    handlers.listen::<UserCreated>(HandleUserCreated.to_handler());

    let dispatcher = EventDispatcherBuilder::new()
        .listen::<UserCreated>(HandleUserCreated.to_handler())
        .subscribe(handlers)
        .build()
        .await;

    dispatcher.dispatch(500);
    dispatcher.dispatch(UserCreated { id: 6000 });
    println!("Hello, world!");
}

impl Dispatchable for i32 {}

#[derive(Debug, Clone)]
struct UserCreated {
    id: u32,
}

impl Dispatchable for UserCreated {}

struct HandleUserCreated;

#[async_trait]
impl EventHandler for HandleUserCreated {
    async fn handle(&self, dispatched: &DispatchedEvent) {
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("we are handling user created event: {:?}", event.id)
    }
}
