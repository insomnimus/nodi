#[cfg(feature = "midir")]
use midir::{self, MidiOutputConnection};

use crate::{
	event::{Event, MidiEvent, Moment},
	Timer,
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
	/// `self.con`.
	///
	/// Stops playing if [Connection::play] returns `false`.
	/// Returns `true` if the track is played through the end, `false` otherwise.
	pub fn play(&mut self, sheet: &[Moment]) -> bool {
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
								if !self.con.play(msg) {
									return false;
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

		true
	}
}

/// Any type that can play sound, given a [MidiEvent].
///
/// This trait is implemented for midir::MidiOutputConnection, if the `midir`
/// feature is enabled.
pub trait Connection {
	/// Given a [MidiEvent], plays the message.
	///
	/// If this function returns `false`, [Player::play] will stop playing and return.
	fn play(&mut self, msg: &MidiEvent) -> bool;
}

#[cfg(feature = "midir")]
impl Connection for MidiOutputConnection {
	fn play(&mut self, msg: &MidiEvent) -> bool {
		let mut buf = Vec::with_capacity(8);
		let _ = msg.write(&mut buf);

		let _ = self.send(&buf);
		true
	}
}
