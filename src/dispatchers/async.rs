use std::{
	any::{Any, TypeId},
	future::Future,
	pin::Pin,
	sync::Arc,
};
use tokio::sync::RwLock;

use crate::Event;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

pub trait EventHandler<Ev: Event + 'static>: Send + Sync {
	fn handle(&self, event: Ev) -> BoxFuture<'_, ()>;
}

impl<Ev: Event + Send + Sync + 'static, F, Fut> EventHandler<Ev> for F
where
	F: Fn(Ev) -> Fut + Send + Sync,
	Fut: Future<Output = ()> + Send + Sync + 'static,
{
	fn handle(&self, event: Ev) -> BoxFuture<'_, ()> {
		Box::pin(async move {
			(self)(event).await;
		})
	}
}

#[allow(clippy::type_complexity)]
struct AsyncListener {
	event: TypeId,
	handler: Box<dyn (Fn(&dyn Any) -> BoxFuture<'_, ()>) + Send + Sync>,
}

pub struct Dispatcher {
	listeners: RwLock<Vec<AsyncListener>>,
}

impl Dispatcher {
	/// Create a new dispatcher.
	#[must_use]
	pub fn new() -> Self {
		Self {
			listeners: RwLock::new(Vec::new()),
		}
	}

	/// Register an event listener with the dispatcher.
	///
	/// # Panics
	///
	/// This function will panic if the event type does not match the dispatcher's event type.
	pub async fn listen<
		Ev: Event + Clone + Send + Sync + 'static,
		Handler: EventHandler<Ev> + 'static,
	>(
		&mut self,
		on_event: Handler,
	) {
		let on_event = Arc::new(on_event);
		let mut listeners = self.listeners.write().await;

		listeners.push(AsyncListener {
			event: TypeId::of::<Ev>(),
			handler: Box::new(move |ev: &dyn Any| {
				let ev = ev
					.downcast_ref::<Ev>()
					.expect("Event type mismatch in dispatcher")
					.clone();
				let on_event = on_event.clone();

				Box::pin(async move { on_event.handle(ev).await })
			}),
		});
	}

	/// Determine if a given event has listeners.
	pub async fn has_listeners<Ev: Event + 'static>(&self) -> bool {
		let listeners = self.listeners.read().await;

		listeners.iter().any(|l| l.event == TypeId::of::<Ev>())
	}

	/// Fire an event and call the listeners.
	///
	/// # Errors
	///
	/// Returns an error if the event type is not registered with the dispatcher.
	#[allow(clippy::significant_drop_in_scrutinee)]
	pub async fn dispatch<Ev: Event + Send + Sync + 'static>(
		&self,
		event: &Ev,
	) -> Result<(), Error> {
		let listeners = self.listeners.read().await;

		let futures = listeners
			.iter()
			.filter(|listener| listener.event == TypeId::of::<Ev>())
			.map(|listener| (listener.handler)(event));

		futures::future::join_all(futures).await;
		drop(listeners);

		Ok(())
	}
}

impl Default for Dispatcher {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// The event type is not registered with the dispatcher.
	#[error("Event type is not registered with the dispatcher")]
	UnregisteredEvent,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, PartialEq)]
	struct OrderShipped {
		order_id: u64,
	}
	impl Event for OrderShipped {}

	#[tokio::test]
	async fn test_async_dispatcher() {
		let mut dispatcher = Dispatcher::new();

		dispatcher
			.listen(|event: OrderShipped| async move { assert_eq!(event.order_id, 123) })
			.await;

		dispatcher
			.dispatch(&OrderShipped { order_id: 123 })
			.await
			.unwrap();
	}
}
