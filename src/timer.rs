use std::{convert::TryFrom, fmt, thread, time::Duration};

use log::{debug, info, trace};
use midly::Timing;

use crate::{Event, Moment};

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
	///
	/// Using the [log] crate, if the log level is set to `debug`,
	/// the sleep duration will be logged before any sleep happens.
	/// If the log level is set to `trace`, the times when the returned duration
	/// is 0 (does not cause [thread::sleep]), will also be logged.
	fn sleep(&self, n_ticks: u32) {
		let t = self.sleep_duration(n_ticks);

		if !t.is_zero() {
			debug!(target: "Timer", "sleeping the thread for {:?}", &t);
			thread::sleep(t);
		} else {
			trace!(target: "Timer", "timer returned 0 duration, not sleeping")
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

/// An error that might arise while converting [Timing] to a [Ticker] or
/// [FixedTempo].
pub struct TimeFormatError;

impl std::error::Error for TimeFormatError {}

impl fmt::Debug for TimeFormatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("unsupported time format")
	}
}

impl fmt::Display for TimeFormatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("unsupported time format")
	}
}

/// Implements a Metrical [Timer].
///
/// # Notes
/// Use this when the MIDI file header specifies the time format as being
/// [Timing::Metrical], this is the case 99% of the time.
///
/// Set the log level to `info` (using the [log] crate) for logging the tempo change events.
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
		let micros_per_tick = tempo as f64 / self.ticks_per_beat as f64;
		info! {
			target: "Ticker",
			"tempo change: {} (microseconds per tick: {} -> {})",
			tempo,
			self.micros_per_tick,
			micros_per_tick,
		};
		self.micros_per_tick = micros_per_tick;
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
	type Error = TimeFormatError;

	/// Tries to create a [Ticker] from the provided [Timing].
	///
	/// # Errors
	/// Will return an error if the given [Timing] is not [Timing::Metrical].
	fn try_from(t: Timing) -> Result<Self, Self::Error> {
		match t {
			Timing::Metrical(n) => Ok(Self::new(u16::from(n))),
			_ => Err(TimeFormatError),
		}
	}
}

/// A [Timer] with a fixed tempo.
///
/// The value wrapped corresponds to the length of a tick, in microseconds.
///
/// # Notes
/// This type corresponds to [Timing::Timecode] and can be converted using
/// [TryFrom::from]. Try to avoid using this timer because it's not tested (it's
/// very rare to get [Timing::Timecode] in real life).
pub struct FixedTempo(pub u64);

impl TryFrom<Timing> for FixedTempo {
	type Error = TimeFormatError;

	fn try_from(t: Timing) -> Result<Self, Self::Error> {
		if let Timing::Timecode(fps, frame) = t {
			let micros = 1_000_000.0 / fps.as_f32() / frame as f32;
			Ok(Self(micros as u64))
		} else {
			Err(TimeFormatError)
		}
	}
}

impl Timer for FixedTempo {
	fn sleep_duration(&self, n_ticks: u32) -> Duration {
		Duration::from_millis(self.0 * n_ticks as u64)
	}

	/// This function does nothing.
	fn change_tempo(&mut self, _: u32) {}
}

/// Returns an appropriate [Timer] for the given [Timing].
///
/// # Notes
/// You will rarely need this, most MIDI files use [Timing::Metrical]; use that
/// with [Ticker].
/// Because this function returns a trait object.
pub fn timing_to_timer(t: Timing) -> Box<dyn Timer> {
	match &t {
		Timing::Metrical(..) => Box::new(Ticker::try_from(t).unwrap()),
		Timing::Timecode(..) => Box::new(FixedTempo::try_from(t).unwrap()),
	}
}
