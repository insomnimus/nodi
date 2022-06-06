//! Contains various small types that implement [Connection] that add extra capabilities to another [Connection] by wrapping them.

use crate::{Connection, MidiEvent};
use std::sync::{
	atomic::{AtomicI8, Ordering},
	Arc,
};

/// [Connection] combinators.
///
/// This trait is implemented for all types that implement [Connection].
pub trait Compose: Connection + Sized {
	/// Returns a [Connection] that transposes its input and calls `self` to play it.
	fn transpose(self, shift: i8, transpose_ch9: bool) -> Transpose<Self> {
		Transpose {
			con: self,
			transpose_ch9,
			shift: Arc::new(AtomicI8::new(shift)),
		}
	}

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

/// A transposing [Connection], Created by calling [Compose::transpose] on an existing connection.
#[derive(Clone, Debug)]
pub struct Transpose<C: Connection> {
	/// The transposition in semi-tones.
	pub shift: Arc<AtomicI8>,
	/// If set to `true`, the drum channel (ch 9) will also be transposed.
	pub transpose_ch9: bool,
	/// The wrapped [Connection].
	pub con: C,
}

impl<C: Connection> Connection for Transpose<C> {
	#[inline]
	fn play(&mut self, event: MidiEvent) -> bool {
		match event.transposed(self.shift.load(Ordering::SeqCst), self.transpose_ch9) {
			Some(e) => self.con.play(e),
			None => true,
		}
	}
}

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
