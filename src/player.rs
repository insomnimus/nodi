use log::error;
use midir::MidiOutputConnection;

use crate::{
	event::{
		Event,
		MidiEvent,
		Moment,
	},
	Sheet,
	Timer,
};

/// A type that can play [Sheet]s.
pub struct Player<T: Timer, C: Connection> {
	/// An active midi connection.
	pub con: C,
	timer: T,
}

impl<T: Timer, C: Connection> Player<T, C> {
	/// Creates a new [Player] with the given [Timer] and
	/// [Connection].
	pub fn new(con: C, timer: T) -> Self {
		Self { con, timer }
	}

	/// Changes `self.timer`.
	pub fn set_timer(&mut self, timer: T) {
		self.timer = timer;
	}

	/// Plays the given [Sheet].
	///
	/// # Remarks
	/// The tempo change events are handled by `self.timer` and playing sound by
	/// `self.con`
	pub fn play_sheet(&mut self, sheet: &Sheet) {
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
								if let Err(e) = self.con.play(msg) {
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

/// Any type that can play sound, given a [MidiEvent].
///
/// This trait is implemented for [MidiOutputConnection].
pub trait Connection {
	/// Any error that may arise while playing a MIDI message.
	type Error: std::error::Error;

	/// Given a [MidiEvent], plays the message.
	fn play(&mut self, msg: &MidiEvent) -> Result<(), Self::Error>;
}

impl Connection for MidiOutputConnection {
	type Error = midir::SendError;

	fn play(&mut self, msg: &MidiEvent) -> Result<(), Self::Error> {
		let mut buf = Vec::with_capacity(4);
		let _ = msg.write(&mut buf);
		self.send(&buf)
	}
}
