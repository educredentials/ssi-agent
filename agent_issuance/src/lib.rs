#[cfg(not(target_arch = "wasm32"))]
use async_trait::async_trait;

#[cfg(not(target_arch = "wasm32"))]
use cqrs_es::{Aggregate, EventEnvelope, Query};

#[cfg(not(target_arch = "wasm32"))]
use tracing::info;

// Aggregates (only for non-WASM)
#[cfg(not(target_arch = "wasm32"))]
pub mod credential;

#[cfg(not(target_arch = "wasm32"))]
pub mod offer;

#[cfg(not(target_arch = "wasm32"))]
pub mod server_config;

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod application;

#[cfg(not(target_arch = "wasm32"))]
pub mod services;

#[cfg(not(target_arch = "wasm32"))]
pub mod state;

// WASM bindings (only compiled for wasm32 target)
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub struct SimpleLoggingQuery {}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl<A: Aggregate> Query<A> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<A>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            info!("{}-{} - {}", aggregate_id, event.sequence, payload);
        }
    }
}
