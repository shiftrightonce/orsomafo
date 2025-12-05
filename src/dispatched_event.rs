#![allow(dead_code)]
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

use crate::Dispatchable;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DispatchedEvent {
    id: Uuid,
    created_at: i64,
    data: String,
    name: String,
}

impl DispatchedEvent {
    pub(crate) fn new(data: String, name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: chrono::Utc::now().timestamp(),
            data,
            name,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn id_ref(&self) -> &Uuid {
        &self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at, 0)
            .single()
            .expect("could not parse event timestamp")
    }

    /// Returns the timestamp
    pub fn created_at_ts(&self) -> i64 {
        self.created_at
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn name_ref(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn data_ref(&self) -> &str {
        &self.data
    }

    /// Returns the actual instance of the event
    /// ```
    /// # use async_trait::async_trait;
    /// # use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
    /// # use tokio::time::{sleep, Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #  _ =  EventDispatcherBuilder::new().build().await;
    ///
    ///    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    ///    struct MyEvent;
    ///    impl Dispatchable for MyEvent {}
    ///
    ///    struct MyEventHandler;
    ///    
    ///    #[orsomafo::async_trait]
    ///    impl EventHandler for MyEventHandler {
    ///        async fn handle(&self, dispatched: DispatchedEvent)  {
    ///           let event: MyEvent = dispatched.the_event().unwrap();
    ///           // or
    ///           // let event = dispatched.the_event::<MyEvent>().unwrap()
    ///           //...
    ///        }
    ///    }
    ///
    /// }
    /// ```
    pub fn the_event<T: Dispatchable>(&self) -> Option<T> {
        serde_json::from_str(self.data_ref()).ok()
    }
}
