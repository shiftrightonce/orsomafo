use crate::{DispatchedEvent, EventHandler};
use async_trait::async_trait;
use futures::future::BoxFuture;

#[derive(Default)]
pub(crate) struct ClosureHandlerWrapper<F>(pub(crate) F)
where
    F: Fn(DispatchedEvent) -> BoxFuture<'static, ()> + Send + Sync + 'static;

#[async_trait]
impl<F> EventHandler for ClosureHandlerWrapper<F>
where
    F: Fn(DispatchedEvent) -> BoxFuture<'static, ()> + Send + Sync + 'static,
{
    async fn handle(&self, event: DispatchedEvent) {
        (self.0)(event).await;
    }
}
