//! Contains various small types that implement [Connection] that add extra capabilities to another [Connection] by wrapping them.

use crate::{Connection, MidiEvent};

/// [Connection] combinators.
///
/// This trait is implemented for all types that implement [Connection].
pub trait Compose: Connection + Sized {
	/// Returns a [Connection] that maps its input and calls `self` to play it.
	fn map<F: FnMut(MidiEvent) -> MidiEvent>(self, f: F) -> Map<Self, F> {
		Map { con: self, f }
	}

	/// Returns a [Connection] that conditionally calls `self` to play its input.
	///
	/// The event is played if `f(&event) == true`.
	/// If the closure returns false, the event will be skipped but the return value will still be `true`.
	fn filter<F>(self, f: F) -> Filter<Self, F>
	where
		F: for<'a> FnMut(&'a MidiEvent) -> bool,
	{
		Filter { con: self, f }
	}
}

impl<C: Connection> Compose for C {}

/// A mapping [Connection]. Created by calling [Compose::map] on an existing connection.
#[derive(Clone, Debug)]
pub struct Map<C: Connection, F> {
	f: F,
	/// The wrapped [Connection].
	pub con: C,
}

impl<C: Connection, F: FnMut(MidiEvent) -> MidiEvent> Connection for Map<C, F> {
	#[inline]
	fn play(&mut self, event: MidiEvent) -> bool {
		let e = (self.f)(event);
		self.con.play(e)
	}
}

/// A filtering [Connection]. Created by calling [Compose::filter] on an existing connection.
#[derive(Clone, Debug)]
pub struct Filter<C: Connection, F> {
	/// The wrapped [Connection].
	pub con: C,
	f: F,
}

impl<C: Connection, F> Connection for Filter<C, F>
where
	F: for<'a> FnMut(&'a MidiEvent) -> bool,
{
	#[inline]
	fn play(&mut self, event: MidiEvent) -> bool {
		if (self.f)(&event) {
			self.con.play(event)
		} else {
			true
		}
	}
}
