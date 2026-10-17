use std::sync::Arc;
use std::time::Duration;

use iced::futures::{SinkExt, Stream};
use iced::stream;
use tokio::time::MissedTickBehavior;

use crate::metrics::{Collector, Snapshot};

#[derive(Debug, Clone)]
pub enum Event {
    Snapshot(Arc<Snapshot>),
    Failed(String)
}

pub fn run(interval_ms: &u64) -> impl Stream<Item = Event> + use<> {
    let min = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
    let period = Duration::from_millis(*interval_ms).max(min);

    stream::channel(4, async move |mut output| {
        let mut collector = match tokio::task::spawn_blocking(Collector::new).await {
            Ok(collector) => collector,
            Err(error) => {
                let _ = output.send(Event::Failed(error.to_string())).await;
                return;
            }
        };

        let mut timer = tokio::time::interval_at(tokio::time::Instant::now() + period, period);
        timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            timer.tick().await;

            let result = tokio::task::spawn_blocking(move || {
                let snapshot = collector.collect();
                (collector, snapshot)
            }).await;

            match result {
                Ok((back, snapshot)) => {
                    collector = back;
                    if output.send(Event::Snapshot(Arc::new(snapshot))).await.is_err() {
                        break;
                    }
                }
                Err(error) => {
                    let _ = output.send(Event::Failed(error.to_string())).await;
                    break;
                }
            }
        }
    })
}