use crate::{DispatchedEvent, EventHandler};
use async_trait::async_trait;
use std::future::Future;

#[derive(Default)]
pub(crate) struct ClosureHandlerWrapper<F, Fut>(pub(crate) Option<F>)
where
    F: Fn(&DispatchedEvent) -> Fut + Clone + 'static,
    Fut: Future;

#[async_trait]
impl<F, Fut> EventHandler for ClosureHandlerWrapper<F, Fut>
where
    F: Fn(&DispatchedEvent) -> Fut + Clone + Sync + Send + 'static,
    Fut: Future + Send + 'static,
{
    async fn handle(&self, event: &DispatchedEvent) {
        if let Some(func) = &self.0 {
            (func)(event).await;
        }
    }
}
