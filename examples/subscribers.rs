use async_trait::async_trait;
use soma::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler, Subscriber};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // 1. Subscriber allows you to register a list of event listeners and .
    let mut handlers = Subscriber::new();
    handlers
        .listen::<UserCreated>(SendWelcomeEmail("noreplay@example.com".into()).to_handler())
        .listen::<UserCreated>(HandleUserCreated.to_handler());

    let _ = EventDispatcherBuilder::new()
        // 2. Use the "subscribe" method on the builder to subscribe to the list of events
        .subscribe(handlers)
        .build()
        .await;

    let user = UserCreated { id: 1 };
    user.dispatch_event();

    // pause for a bit
    sleep(Duration::from_millis(100)).await;
}

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

struct SendWelcomeEmail(String);

#[async_trait]
impl EventHandler for SendWelcomeEmail {
    async fn handle(&self, event: &DispatchedEvent) {
        let user = event.the_event::<UserCreated>().unwrap();
        println!(
            "Sending welcoming email to new user {:?} from {:?}",
            user.id, &self.0
        );
    }
}
