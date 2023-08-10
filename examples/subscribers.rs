use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler, Subscriber};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // 1. Subscriber allows you to register a list of event listeners.
    let handlers = Subscriber::new()
        .listen::<UserCreated, SendWelcomeEmail>()
        .listen::<UserCreated, HandleUserCreated>();

    let _ = EventDispatcherBuilder::new()
        // 2. Use the "subscribe" method on the builder to subscribe to the list of events
        .subscribe(handlers)
        .build()
        .await;

    // 3. Another way to register subscribers
    //    A crate can use this method to register it's specific
    //    event handlers
    Subscriber::new()
        .listen::<UserCreated, SendWelcomeEmail>()
        .listen::<UserCreated, HandleUserCreated>()
        .build() // Same as "subscribe" on the builder
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

#[derive(Default)]
struct HandleUserCreated;

#[async_trait]
impl EventHandler for HandleUserCreated {
    async fn handle(&self, dispatched: &DispatchedEvent) {
        let event: UserCreated = dispatched.the_event().unwrap();
        println!("we are handling user created event: {:?}", event.id)
    }
}

struct SendWelcomeEmail(String);

impl Default for SendWelcomeEmail {
    fn default() -> Self {
        Self("noreplay@example.com".into())
    }
}

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
