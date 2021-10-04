use log::error;
#[cfg(any(feature = "midir", doc, test))]
use midir::{self, MidiOutputConnection};

use crate::{
	event::{Event, MidiEvent, Moment},
	Sheet, Timer,
};

#[doc = include_str!("doc_player.md")]
pub struct Player<T: Timer, C: Connection> {
	/// An active midi connection.
	pub con: C,
	timer: T,
}

impl<T: Timer, C: Connection> Player<T, C> {
	/// Creates a new [Player] with the given [Timer] and
	/// [Connection].
	pub fn new(timer: T, con: C) -> Self {
		Self { con, timer }
	}

	/// Changes `self.timer`.
	pub fn set_timer(&mut self, timer: T) {
		self.timer = timer;
	}

	/// Plays the given [Moment] slice.
	///
	/// # Notes
	/// The tempo change events are handled by `self.timer` and playing sound by
	/// `self.con`
	pub fn play_moments(&mut self, sheet: &[Moment]) {
		let mut counter = 0_u32;

		for moment in sheet {
			match moment {
				Moment::Events(events) if !events.is_empty() => {
					self.timer.sleep(counter);
					counter = 0;

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
				_ => (),
			};
			counter += 1;
		}
	}

	/// Plays the given [Sheet].
	///
	/// Equivalent to `.play_moments(&sheet[..])`.
	/// See also [Player::play_moments].
	pub fn play_sheet(&mut self, sheet: &Sheet) {
		self.play_moments(&sheet[..])
	}
}

/// Any type that can play sound, given a [MidiEvent].
///
/// This trait is implemented for [midir::MidiOutputConnection], if the `midir`
/// feature is set.
pub trait Connection {
	/// Any error that may arise while playing a MIDI message.
	type Error: std::error::Error;

	/// Given a [MidiEvent], plays the message.
	fn play(&mut self, msg: &MidiEvent) -> Result<(), Self::Error>;
}

#[cfg(any(feature = "midir", doc, test))]
impl Connection for MidiOutputConnection {
	type Error = midir::SendError;

	fn play(&mut self, msg: &MidiEvent) -> Result<(), Self::Error> {
		let mut buf = Vec::with_capacity(4);
		let _ = msg.write(&mut buf);
		self.send(&buf)
	}
}
