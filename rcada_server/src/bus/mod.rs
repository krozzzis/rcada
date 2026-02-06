pub mod mock;

use async_trait::async_trait;
use rcada_core::tag::TagName;
use std::time::Duration;

#[async_trait]
pub trait BusDriver: Send + Sync {
    fn default_poll_rate(&self) -> Duration;

    fn register_tag(&mut self, name: TagName, poll_rate: Option<Duration>);

    async fn start(&mut self);

    fn stop(&mut self);

    async fn poll(&self);
}
