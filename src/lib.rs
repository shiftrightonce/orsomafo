//! # Soma
//! Soma is a event dispatcher
//!
//! Events are dispatchable across threads. Handlers are executed asynchronously
//!
//! ## Example
//! ```
//! # use async_trait::async_trait;
//! # use soma::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
//! # use tokio::time::{sleep, Duration};
//!
//! #[derive(Clone, Debug)] // Event must be cloneable
//! struct MyEvent;
//!
//! impl soma::Dispatchable for MyEvent {} // MyEvent is now dispatchable
//!
//!  // create a handler
//!  struct MyEventHandler;
//!    
//!  #[soma::async_trait]
//!   impl soma::EventHandler for MyEventHandler {
//!        // called when event from "MyEvent" is dispatched
//!        async fn handle(&self, dispatched: &DispatchedEvent)  {
//!           let event: MyEvent = dispatched.the_event().unwrap();  // Get the instance of "MyEvent"
//!           println!("handled my event: {:#?}",event);
//!        }
//!    }
//!
//!  #[tokio::main]
//!  async fn main() {
//!   _ =  EventDispatcherBuilder::new()
//!         .listen::<MyEvent>(MyEventHandler.to_handler()) // Register "MyEventHandler" for "MyEvent"
//!         .build().await;
//!
//!    let event = MyEvent;
//!    event.dispatch_event();
//!
//! }
mod builder;
mod dispatched_event;
mod event;
mod event_dispatcher;
mod event_listener;

pub use async_trait::async_trait;
pub use builder::EventDispatcherBuilder;
pub use dispatched_event::DispatchedEvent;
pub use event::*;
pub use event_dispatcher::event_dispatcher;
pub use event_dispatcher::EventDispatcher;
pub use event_listener::Subscriber;
