use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    // 1. Create an instance EventDispatcherBuilder and use it to register
    //    listeners. Note: You can create a dispatcher as many times as you like
    let dispatcher = EventDispatcherBuilder::new()
        // 2. On the builder instance, use the `listen` method to register a handler for an event.
        //    listen::<The event you want to listen for, The handler>().
        //    All handler must implement `EventHandler`
        .listen::<UserCreated, HandleUserCreated>()
        .build()
        .await;

    // 3. When you are ready to dispatch an event, create an instance of your event
    //    and call the `dispatch_event` method on the instance
    let event = UserCreated { id: 1 };
    event.dispatch_event(); // Dispatches the event

    // 4. The returned "dispatcher" instance from the builder can be used to dispatch
    //    the event.
    let event2 = UserCreated { id: 2 };
    dispatcher.dispatch(event2);

    // The following line is use to pause the application for
    // few milliseconds. This will allow us to handle all dispatched events.
    // In a full application, this line wil not be require.
    sleep(Duration::from_millis(100)).await;
}

// 5. Create you event
//    Events must implement "Clone". A of the even is passed to each handler
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct UserCreated {
    id: u32,
}

// 6. An event must implement "soma::Dispatchable"
impl Dispatchable for UserCreated {}

// 7. Create an event handler
//    The handler must implement default
#[derive(Default)]
struct HandleUserCreated;

// 8. Event handler must implement "soma::EventHandler"
#[async_trait]
impl EventHandler for HandleUserCreated {
    async fn handle(&self, dispatched: &DispatchedEvent) {
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("we are handling user created event: {:?}", event.id)
    }
}
