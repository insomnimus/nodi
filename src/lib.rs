pub mod player;

pub enum Timer {
	Ticker(Ticker),
	Absolute(f64),
}

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

	pub fn change_tempo(&mut self, tempo: u32) {
		self.micros_per_tick = tempo as f64 / self.ticks_per_beat as f64;
	}
}
