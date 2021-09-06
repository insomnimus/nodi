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

/// A type that can play [Sheet]s.
pub struct Player<T: Timer> {
	/// An initialized MIDI output connection.
	pub con: MidiOutputConnection,
	timer: T,
}

impl<T: Timer> Player<T> {
	/// Creates a new [Player] with the given [Timer] and
	/// [MidiOutputConnection].
	pub fn new(con: MidiOutputConnection, timer: T) -> Self {
		Self { con, timer }
	}

	/// Changes `self.timer`.
	pub fn set_timer(&mut self, timer: T) {
		self.timer = timer;
	}

	/// Plays the given [Sheet].
	///
	/// # Remarks
	/// The tempo change events are handled by `self.timer`.
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
