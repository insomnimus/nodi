use std::{
	convert::TryFrom,
	thread,
	time::Duration,
};

use midly::Timing;

pub trait Timer {
	fn tick_len_micros(&self) -> f64;
	fn change_tempo(&mut self, tempo: u32);

	fn sleep(&self, n_ticks: u32) {
		let t = self.tick_len_micros() * n_ticks as f64;

		if t > 0.0 {
			thread::sleep(Duration::from_micros(t as u64));
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ticker {
	ticks_per_beat: u16,
	micros_per_tick: f64,
}

impl Ticker {
	pub fn new(ticks_per_beat: u16) -> Self {
		Self {
			ticks_per_beat,
			micros_per_tick: 0.0,
		}
	}

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

	fn try_from(t: Timing) -> Result<Self, Self::Error> {
		match t {
			Timing::Metrical(n) => Ok(Self::new(u16::from(n))),
			_ => Err("unsupported time format".into()),
		}
	}
}
