#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! A simple sync/async event dispatcher.
//!
//! # Usage
//!
//! ```rust
//! use epicenter::{Event, AsyncDispatcher};
//!
//! # #[derive(Debug, Clone)]
//! struct OrderShipped {
//!     order_id: u64
//! }
//! impl Event for OrderShipped {}
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut dispatcher = AsyncDispatcher::new();
//!
//! dispatcher.listen(|event: OrderShipped| async move {
//!     assert_eq!(event.order_id, 123);
//! }).await;
//!
//! dispatcher.dispatch(&mut OrderShipped {
//!     order_id: 123
//! }).await?;
//! # Ok(())
//! # }
//! ```

pub mod dispatchers;

#[cfg(feature = "async")]
pub use dispatchers::r#async::Dispatcher as AsyncDispatcher;
pub use dispatchers::sync::Dispatcher as SyncDispatcher;

pub trait Event {}
