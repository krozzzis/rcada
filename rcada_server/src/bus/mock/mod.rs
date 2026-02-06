use async_trait::async_trait;
use chrono::Utc;
use rcada_core::tag::TagName;
use rcada_core::value::DataType;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::broadcast;

use crate::bus::BusDriver;
use crate::container::Container;
use crate::tag_storage::TagRepository;

pub struct MockBusDriver<R: TagRepository> {
    di: Container<R>,
    default_rate: Duration,
    tags: HashMap<TagName, Duration>,
    stop_tx: Option<broadcast::Sender<()>>,
    min: f64,
    max: f64,
}

impl<R: TagRepository> MockBusDriver<R> {
    pub fn new(di: Container<R>, min: f64, max: f64, default_rate: Duration) -> Self {
        Self {
            di,
            default_rate,
            tags: HashMap::new(),
            stop_tx: None,
            min,
            max,
        }
    }
}

#[async_trait]
impl<R: TagRepository> BusDriver for MockBusDriver<R> {
    fn default_poll_rate(&self) -> Duration {
        self.default_rate
    }

    fn register_tag(&mut self, name: TagName, poll_rate: Option<Duration>) {
        let rate = poll_rate.unwrap_or(self.default_rate);
        self.tags.insert(name, rate);
    }

    async fn start(&mut self) {
        if self.tags.is_empty() {
            return;
        }

        let (stop_tx, stop_rx) = broadcast::channel(1);
        self.stop_tx = Some(stop_tx.clone());

        let tags: HashMap<TagName, Duration> = self.tags.clone();
        let min = self.min;
        let max = self.max;
        let di = self.di.clone();

        for (name, rate) in tags {
            let name = name.clone();
            let mut stop_rx = stop_rx.resubscribe();
            let di = di.clone();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(rate);
                let start_time = Utc::now();
                let di = di;

                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let elapsed = (Utc::now() - start_time).num_milliseconds() as f64 / 1000.0;
                            let sin_val = elapsed * 2.0;
                            let value = min + sin_val.sin() * (max - min) / 2.0 + (max - min) / 2.0;

                            let tag_value = rcada_core::tag::TagValue {
                                value: rcada_core::value::Value::Float32(value as f32),
                                timestamp: Some(Utc::now()),
                            };

                            let _ = di
                                .create_update_value_command(name.clone(), tag_value)
                                .execute();
                        }
                        _ = stop_rx.recv() => {
                            break;
                        }
                    }
                }
            });
        }
    }

    fn stop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
    }

    async fn poll(&self) {
        let start_time = Utc::now();
        let min = self.min;
        let max = self.max;

        for name in self.tags.keys() {
            let elapsed = (Utc::now() - start_time).num_milliseconds() as f64 / 1000.0;
            let sin_val = elapsed * 2.0;
            let value = min + sin_val.sin() * (max - min) / 2.0 + (max - min) / 2.0;

            let tag_value = rcada_core::tag::TagValue {
                value: rcada_core::value::Value::Float32(value as f32),
                timestamp: Some(Utc::now()),
            };

            let _ = self
                .di
                .create_update_value_command(name.clone(), tag_value)
                .execute();
        }
    }
}

impl<R: TagRepository> MockBusDriver<R> {
    pub fn create_tag(&self, name: TagName, unit: rcada_core::unit::Unit) {
        let _ = self
            .di
            .create_create_tag_command(name.clone(), unit, DataType::Float32)
            .execute();
    }
}
