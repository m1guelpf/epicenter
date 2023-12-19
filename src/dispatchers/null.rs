use super::sync;
use crate::Event;

pub struct Dispatcher {
	dispatcher: sync::Dispatcher,
}

impl Dispatcher {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			dispatcher: sync::Dispatcher::new(),
		}
	}

	/// Register an event listener with the dispatcher.
	///
	/// # Errors
	///
	/// Returns an error if the listener lock is poisoned.
	pub fn listen<Ev: Event + 'static>(
		&mut self,
		on_event: impl FnMut(&mut Ev) + 'static,
	) -> Result<(), sync::Error> {
		self.dispatcher.listen(on_event)
	}

	/// Determine if a given event has listeners.
	///
	/// # Errors
	///
	/// Returns an error if the listener lock is poisoned.
	pub fn has_listeners<Ev: Event + 'static>(&self) -> Result<bool, sync::Error> {
		self.dispatcher.has_listeners::<Ev>()
	}

	/// Don't fire an event.
	pub fn dispatch<Ev: Event + 'static>(&self, _: &mut Ev) {
		//
	}
}
