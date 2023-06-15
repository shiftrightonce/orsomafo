use std::{any::Any, fmt::Debug};

use async_trait::async_trait;

pub trait Dispatchable: Send + Sync + Debug {
    fn event() -> String
    where
        Self: Sized,
    {
        std::any::type_name::<Self>().to_string()
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    // async fn handle(&self, event: &Box<dyn Dispatchable>);
    async fn handle(&self, event: &DispatchedEvent);

    fn to_handler(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[derive(Debug)]
pub struct DispatchedEvent(Box<dyn Any + Send + Sync + 'static>);

impl DispatchedEvent {
    pub fn new(inner: Box<dyn Any + Send + Sync + 'static>) -> Self {
        Self(inner)
    }

    pub fn the_event<T: Clone + 'static>(&self) -> Option<T> {
        let result: Option<&T> = self.0.downcast_ref();
        result.cloned()
    }
}
