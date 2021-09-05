use log::error;
use midir::MidiOutputConnection;

use crate::{
	event::{
		Event,
		Moment,
	},
	Sheet,
	Timer,
};

pub struct Player<T: Timer> {
	pub con: MidiOutputConnection,
	timer: T,
}

impl<T: Timer> Player<T> {
	pub fn new(con: MidiOutputConnection, timer: T) -> Self {
		Self { con, timer }
	}

	pub fn set_timer(&mut self, timer: T) {
		self.timer = timer;
	}

	pub fn play_sheet(&mut self, sheet: &Sheet) {
		let mut buf = Vec::with_capacity(6);
		let mut empty_counter = 0_u32;
		for moment in &sheet.0 {
			match moment {
				Moment::Empty => empty_counter += 1,
				Moment::Events(events) => {
					self.timer.sleep(empty_counter);
					empty_counter = 0;
					for event in events {
						match event {
							Event::Tempo(val) => self.timer.change_tempo(*val),
							Event::Midi(msg) => {
								buf.clear();
								let _ = msg.write(&mut buf);
								if let Err(e) = self.con.send(&buf) {
									error!("failed to send a midi message: {:?}", e);
								}
							}
							_ => (),
						};
					}
				}
			};
		}
	}
}
