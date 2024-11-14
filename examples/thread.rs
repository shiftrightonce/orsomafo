use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler, Subscriber};
use std::thread;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // 1. Subscriber allows you to register a list of event listeners and .
    let handlers = Subscriber::new()
        .listen::<UserCreated, SendWelcomeEmail>()
        .listen::<UserCreated, HandleUserCreated>();

    let _ = EventDispatcherBuilder::new()
        // 2. Use the "subscribe" method on the builder to subscribe to the list of events
        .subscribe(handlers)
        .build()
        .await;

    // 3. From another thread, 1000 users are created
    let handle = thread::spawn(move || {
        for id in 1..=1000 {
            let event = UserCreated { id };
            event.dispatch_event();
        }
    });

    handle.join().unwrap();

    // pause for a bit
    sleep(Duration::from_millis(100)).await;
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
        println!("HandleUserCreated: user {:?} was created", event.id)
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
    async fn handle(&self, event: DispatchedEvent) {
        let user = event.the_event::<UserCreated>().unwrap();
        println!(
            "SendWelcomeEmail: Sending welcoming email to new user {:?} from {:?}",
            user.id, &self.0
        );
    }
}
