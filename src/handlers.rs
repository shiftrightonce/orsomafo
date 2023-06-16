use crate::{dispatched_event::DispatchedEvent, event::EventHandler, UserCreated};
use async_trait::async_trait;

pub struct SendEmailOnUserCreated;

#[async_trait]
impl EventHandler for SendEmailOnUserCreated {
    async fn handle(&self, dispatched_event: &DispatchedEvent) {
        let user = dispatched_event.the_event::<UserCreated>();
        println!("sending new user created email: {:#?}", user)
    }
}
