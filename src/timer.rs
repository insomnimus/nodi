use std::{
	convert::TryFrom,
	thread,
	time::Duration,
};

use midly::Timing;

use crate::{
	Event,
	Moment,
};

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
	/// The provided implementation will call [thread::sleep] with the argument
	/// being `self.sleep_duration(n_ticks)`.
	///
	/// # Notes
	/// The provided implementation will not sleep if
	/// `self.sleep_duration(n_ticks).is_zero()`.
	fn sleep(&self, n_ticks: u32) {
		let t = self.sleep_duration(n_ticks);

		if !t.is_zero() {
			thread::sleep(t);
		}
	}

	/// Calculates the length of a track or a slice of [Moment]s.
	///
	/// # Notes
	/// The default implementation modifies `self` if a tempo event is found.
	fn duration(&mut self, moments: &[Moment]) -> Duration {
		let mut total = Duration::default();
		let mut empty_counter = 0_u32;
		for moment in moments {
			match moment {
				Moment::Empty => empty_counter += 1,
				Moment::Events(events) => {
					total += self.sleep_duration(empty_counter);
					empty_counter = 0;
					for event in events {
						if let Event::Tempo(val) = event {
							self.change_tempo(*val);
						}
					}
				}
			}
		}
		total
	}
}

/// Implements a Metrical [Timer].
///
/// # Remarks
/// Use this when the MIDI file header specifies the time format as being
/// [Timing::Metrical], this is the case 99% of the time.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ticker {
	ticks_per_beat: u16,
	micros_per_tick: f64,
	/// Speed modifier, a value of `1.0` is the default and affects nothing.
	///
	/// Important: Do not set to 0.0, this value is used as a denominator.
	pub speed: f32,
}

impl Ticker {
	/// Creates an instance of [Self] with the given ticks-per-beat.
	/// The tempo will be infinitely rapid, meaning no sleeps will happen.
	/// However this is rarely an issue since a tempo change message will set
	/// it, and this usually happens before any non-0 offset event.
	pub const fn new(ticks_per_beat: u16) -> Self {
		Self {
			ticks_per_beat,
			micros_per_tick: 0.0,
			speed: 1.0,
		}
	}

	/// Will create an instance of [Self] with a provided tempo.
	pub fn with_initial_tempo(ticks_per_beat: u16, tempo: u32) -> Self {
		let mut s = Self::new(ticks_per_beat);
		s.change_tempo(tempo);
		s
	}
}

impl Timer for Ticker {
	fn change_tempo(&mut self, tempo: u32) {
		self.micros_per_tick = tempo as f64 / self.ticks_per_beat as f64;
	}

	fn sleep_duration(&self, n_ticks: u32) -> Duration {
		let t = self.micros_per_tick * n_ticks as f64 / self.speed as f64;
		if t > 0.0 {
			Duration::from_micros(t as u64)
		} else {
			Duration::default()
		}
	}
}

impl TryFrom<Timing> for Ticker {
	type Error = Box<dyn std::error::Error>;

	/// Tries to create a [Ticker] from the provided [Timing].
	///
	/// # Errors
	/// Will return an error if the given [Timing] is not [Timing::Metrical].
	fn try_from(t: Timing) -> Result<Self, Self::Error> {
		match t {
			Timing::Metrical(n) => Ok(Self::new(u16::from(n))),
			_ => Err("unsupported time format".into()),
		}
	}
}
