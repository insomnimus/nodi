use midly::num::u7;
use std::mem;

use midly::MidiMessage;

use super::{Event, MidiEvent, Moment};

impl Moment {
	/// Transposes every note contained in this `moment`.
	///
	/// Calls [MidiEvent::transposed] for every [Event::Midi].
	/// # Arguments
	/// - `shift`: Amount of half notes the notes will be transposed with.
	/// - `transpose_ch9`:  If set to `true`, the notes in MIDI channel 9
	///   (percussion by default) are also transposed.
	///
	/// # Notes
	/// Only `NoteOn`, `NoteOff` and `Aftertouch` messages will be transposed.
	/// The notes exceeding the MIDI treshhold (0..=127) are dropped, meaning
	/// this function is lossy.
	pub fn transpose(&mut self, shift: i8, transpose_ch9: bool) {
		if shift == 0 {
			return;
		}

		self.events = mem::take(&mut self.events)
			.into_iter()
			.filter_map(|e| match e {
				Event::Midi(e) => e.transposed(shift, transpose_ch9).map(Event::Midi),
				x => Some(x),
			})
			.collect();
	}
}

impl MidiEvent {
	/// Transposes `self` `shift` amount of half-steps.
	/// Only MIDI events with keys are transposed, others are left as is.
	/// #### Notes
	/// Normally MIDI channel 10 (index 9) is reserved for drums and MIDI keys on that channel mean not pitch but instrument/timber.
	/// So transposing them is often not desirable.
	/// If you still want to transpose the channel 9, set `transpose_ch9` to `true`.
	/// #### Return Value
	/// This function is lossy: MIDI notes can't exceed 127 or go below 0 so when that happens, this function returns `None`, otherwise it returns the transposed event.
	pub fn transposed(self, shift: i8, transpose_ch9: bool) -> Option<Self> {
		if shift == 0 || (!transpose_ch9 && self.channel.as_int() == 9) {
			return Some(self);
		}
		let shift = shift as i32;
		let tp = move |n: u7| -> Option<u7> {
			let n = shift + n.as_int() as i32;
			if !(0..128).contains(&n) {
				None
			} else {
				Some(u7::new(n as u8))
			}
		};

		let message = match self.message {
			MidiMessage::NoteOn { key, vel } => {
				tp(key).map(|key| MidiMessage::NoteOn { key, vel })?
			}
			MidiMessage::NoteOff { key, vel } => {
				tp(key).map(|key| MidiMessage::NoteOff { key, vel })?
			}
			MidiMessage::Aftertouch { key, vel } => {
				tp(key).map(|key| MidiMessage::Aftertouch { key, vel })?
			}
			other => other,
		};
		Some(Self {
			message,
			channel: self.channel,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn transpose() {
		fn new_moment(range: std::ops::RangeInclusive<i32>) -> Moment {
			Moment {
				events: range
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
			}
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
