#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("doc_lib.md")]

pub mod compose;
mod event;
mod player;
mod sheet;
pub mod timers;

use std::time::Duration;

pub use self::{compose::Compose, event::*, player::*, sheet::*};
/// Re-export of the [midly] crate.
pub use midly;
use timers::sleep;

/// Used for timing MIDI playback.
pub trait Timer {
	/// Returns the [Duration] that should be slept for.
	///
	/// # Arguments
	/// - `n_ticks`: Number of MIDI ticks to sleep for.
	fn sleep_duration(&self, n_ticks: u32) -> Duration;

	/// Changes the timers tempo.
	///
	/// # Arguments
	/// - `tempo`: Represents microseconds per a beat (MIDI quarter note).
	fn change_tempo(&mut self, tempo: u32);

	/// Sleeps given number of ticks.
	/// The provided implementation will sleep the thread  for
	/// `self.sleep_duration(n_ticks)`.
	///
	/// # Notes
	/// The provided implementation will not sleep if
	/// `self.sleep_duration(n_ticks).is_zero()`.
	fn sleep(&self, n_ticks: u32) {
		let t = self.sleep_duration(n_ticks);

		if !t.is_zero() {
			sleep(t);
		}
	}

	/// Calculates the length of a track or a slice of [Moment]s.
	///
	/// # Notes
	/// The default implementation modifies `self` if a tempo event is found.
	fn duration(&mut self, moments: &[Moment]) -> Duration {
		let mut counter = Duration::default();
		for moment in moments {
			counter += self.sleep_duration(1);
			for event in &moment.events {
				if let Event::Tempo(val) = event {
					self.change_tempo(*val);
				}
			}
		}
		counter
	}
}
