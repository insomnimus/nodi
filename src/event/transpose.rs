use std::mem;

use midly::MidiMessage;

use super::{Event, MidiEvent, Moment};

impl Moment {
	/// Transposes every note contained in this moment, returns `true` if
	/// anything has changed.
	///
	/// # Arguments
	/// - `shift`: Amount of half notes the notes will be transposed with.
	/// - `transpose_ch9`:  If set to `true`, the notes in MIDI channel 9
	///   (percussion by default) are also transposed.
	///
	/// # Notes
	/// Only `NoteOn`, `NoteOff` and `Aftertouch` messages will be transposed.
	/// The notes exceeding the MIDI treshold (0..=127) are dropped, meaning
	/// this function is lossy. If after transposition, all the notes are
	/// dropped, `self` will be set to `Moment::Empty`.
	pub fn transpose(&mut self, shift: i8, transpose_ch9: bool) -> bool {
		use midly::num::u7;

		let shift = shift as i32;
		let tp = move |n: u7| -> Option<u7> {
			let n = shift + n.as_int() as i32;
			if !(0..128).contains(&n) {
				None
			} else {
				Some(u7::new(n as u8))
			}
		};

		match self {
			Self::Empty => false,
			_ if shift == 0 => false,
			Self::Events(events) => {
				let mut changed = false;
				let buf = mem::take(events);
				*events = buf
					.into_iter()
					.filter_map(|e| match e {
						Event::Midi(m) if transpose_ch9 || m.channel != 9 => {
							let channel = m.channel;
							match m.message {
								MidiMessage::NoteOn { key, vel } => tp(key).map(|k| {
									changed = true;
									MidiMessage::NoteOn { key: k, vel }
								}),
								MidiMessage::NoteOff { key, vel } => tp(key).map(|k| {
									changed = true;
									MidiMessage::NoteOff { key: k, vel }
								}),
								MidiMessage::Aftertouch { key, vel } => tp(key).map(|k| {
									changed = true;
									MidiMessage::Aftertouch { key: k, vel }
								}),
								other => Some(other),
							}
							.map(|m| {
								Event::Midi(MidiEvent {
									channel,
									message: m,
								})
							})
						}
						other => Some(other),
					})
					.collect();
				if events.is_empty() && changed {
					*self = Self::Empty;
				}
				changed
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn transpose() {
		fn new_moment(range: std::ops::RangeInclusive<i32>) -> Moment {
			Moment::Events(
				range
					.map(|n| {
						Event::Midi(MidiEvent {
							channel: 0.into(),
							message: MidiMessage::NoteOn {
								key: (n as u8).into(),
								vel: 50.into(),
							},
						})
					})
					.collect(),
			)
		}

		let tests = vec![
			(12, 12..=127),
			(-12, 0..=115),
			(126, 126..=127),
			(-126, 0..=1),
		];

		let full = new_moment(0..=127);

		for (shift, range) in tests {
			let mut m = full.clone();
			m.transpose(shift, false);
			let expected = new_moment(range);
			assert_eq!(m, expected);
		}
	}
}
