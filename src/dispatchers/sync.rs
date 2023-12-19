use std::{
	any::{Any, TypeId},
	sync::RwLock,
};

use crate::Event;

#[allow(clippy::type_complexity)]
struct Listener {
	event: TypeId,
	handler: Box<dyn (FnMut(&mut dyn Any) -> Result<(), Error>)>,
}

pub struct Dispatcher {
	listeners: RwLock<Vec<Listener>>,
}

impl Dispatcher {
	/// Create a new event dispatcher instance.
	#[must_use]
	pub const fn new() -> Self {
		Self {
			listeners: RwLock::new(Vec::new()),
		}
	}

	/// Register an event listener with the dispatcher.
	///
	/// # Errors
	///
	/// Returns an error if the listener lock is poisoned.
	pub fn listen<Ev: Event + 'static>(
		&mut self,
		mut on_event: impl FnMut(&mut Ev) + 'static,
	) -> Result<(), Error> {
		let mut listeners = self.listeners.write().map_err(|_| Error::LockPoisoned)?;

		listeners.push(Listener {
			event: TypeId::of::<Ev>(),
			handler: Box::new(move |ev: &mut dyn Any| -> Result<(), Error> {
				(on_event)(ev.downcast_mut().ok_or(Error::UnregisteredEvent)?);

				Ok(())
			}),
		});

		drop(listeners);

		Ok(())
	}

	/// Determine if a given event has listeners.
	///
	/// # Errors
	///
	/// Returns an error if the listener lock is poisoned.
	pub fn has_listeners<Ev: Event + 'static>(&self) -> Result<bool, Error> {
		let listeners = self.listeners.read().map_err(|_| Error::LockPoisoned)?;

		Ok(listeners.iter().any(|l| l.event == TypeId::of::<Ev>()))
	}

	/// Fire an event and call the listeners.
	///
	/// # Errors
	///
	/// Returns an error if the event type is not registered with the dispatcher.
	#[allow(clippy::significant_drop_in_scrutinee)]
	pub fn dispatch<Ev: Event + 'static>(&self, event: &mut Ev) -> Result<(), Error> {
		for listener in self
			.listeners
			.write()
			.map_err(|_| Error::LockPoisoned)?
			.iter_mut()
			.filter(|listener| listener.event == TypeId::of::<Ev>())
		{
			(listener.handler)(event)?;
		}

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

	/// The listener lock is poisoned.
	#[error("The listener lock is poisoned")]
	LockPoisoned,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, PartialEq)]
	struct OrderShipped {
		order_id: u64,
	}
	impl Event for OrderShipped {}

	#[test]
	fn test_sync_dispatcher() {
		let mut dispatcher = Dispatcher::new();

		dispatcher
			.listen(|event: &mut OrderShipped| {
				assert_eq!(event.order_id, 123);
			})
			.unwrap();

		dispatcher
			.dispatch(&mut OrderShipped { order_id: 123 })
			.unwrap();
	}
}
