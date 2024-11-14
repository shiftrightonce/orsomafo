use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // For logging purpose only.

    let generic_event = "workflow_state_changed";

    let dispatcher = EventDispatcherBuilder::new()
        // 1. Register a closure
        .listen_str_fn(generic_event, |event| {
            Box::pin(async move {
                println!("handled by 'listen_str_fn' >> {:}", event.data());
            })
        })
        // 2. Register a handler that implements `Default`
        .listen_str::<HandleWorkflowEvent>(generic_event)
        // 3. Register a instance of a handler
        .listen_str_with(generic_event, HandleWorkflowEvent)
        .build()
        .await;

    // 4. Dispatch the events using the event dispatcher
    dispatcher.dispatch_str(generic_event, WorkflowState::Started);
    dispatcher.dispatch_str(generic_event, WorkflowState::Stalled);
    dispatcher.dispatch_str(generic_event, WorkflowState::Completed);

    // 5. Dispatch the event from the event instance
    WorkflowState::Started.dispatch_event_as(generic_event);
    WorkflowState::Completed.dispatch_event_as(generic_event);

    // The following line is use to pause the application for
    // few milliseconds. This will allow us to handle all dispatched events.
    // In a full application, this line wil not be require.
    sleep(Duration::from_millis(100)).await;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
enum WorkflowState {
    Started,
    Stalled,
    Completed,
}
impl Dispatchable for WorkflowState {}

#[derive(Debug, Default)]
struct HandleWorkflowEvent;

#[async_trait]
impl EventHandler for HandleWorkflowEvent {
    async fn handle(&self, event: DispatchedEvent) {
        println!(
            "handled by 'HandleWorkflowEvent' struct >> {:}",
            event.data_ref()
        )
    }
}
