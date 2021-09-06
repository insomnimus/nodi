use std::{
	convert::TryFrom,
	thread,
	time::Duration,
};

use midly::Timing;

/// Used for timing MIDI playback.
pub trait Timer {
	/// Must return how many microseconds should a MIDI tick last.
	fn tick_len_micros(&self) -> f64;

	/// Changes the timers tempo.
	///
	/// # Arguments
	/// - `tempo`: Represents microseconds per a beat (MIDI quarter note).
	fn change_tempo(&mut self, tempo: u32);

	/// Sleeps given number of ticks.
	/// The provided implementation will call [thread::sleep].
	fn sleep(&self, n_ticks: u32) {
		let t = self.tick_len_micros() * n_ticks as f64;

		if t > 0.0 {
			thread::sleep(Duration::from_micros(t as u64));
		}
	}
}

/// Implements a metronomical [Timer].
///
/// # Remarks
/// Use this when the MIDI file header specifies the time format as being
/// [Timing::Metrical], this is the case 99% of the time.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ticker {
	ticks_per_beat: u16,
	micros_per_tick: f64,
	/// Speed modifier, a value of `1.0` is the default and affects nothing.
	pub speed: f32,
}

impl Ticker {
	/// Creates an instance of [Self] with the given ticks-per-beat.
	/// The tempo will be infinitely rapid, meaning no sleeps will happen.
	/// However this is rarely an issue since a tempo change message will set
	/// it, and this usually happens before any non-0 offset event.
	pub fn new(ticks_per_beat: u16) -> Self {
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
	fn tick_len_micros(&self) -> f64 {
		self.micros_per_tick
	}

	fn change_tempo(&mut self, tempo: u32) {
		self.micros_per_tick = tempo as f64 / self.ticks_per_beat as f64;
	}
}

impl Timer for f64 {
	fn tick_len_micros(&self) -> f64 {
		*self
	}

	fn change_tempo(&mut self, _: u32) {}
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
