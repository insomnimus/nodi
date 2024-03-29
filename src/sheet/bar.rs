use std::collections::VecDeque;

use crate::{Event, Moment, Sheet};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct TimeSignature {
	// Beats per bar.
	numerator: u8,
	// Note of a beat. A negative power of 2.
	denominator: u8,
}

impl TimeSignature {
	fn bar_32s(&self) -> f32 {
		let note_as_32s = 2_f32.powi(5_i32 - self.denominator as i32);
		// let note_as_32s = 32.0 / self.denominator as f32;
		self.numerator as f32 * note_as_32s
	}
}

/// An iterator over bars in a MIDI [Sheet].
#[derive(Debug, Clone, PartialEq)]
pub struct Bars {
	time_sig: TimeSignature,
	// beat_32s: u8,
	tpb: f32,
	buf: VecDeque<Moment>,
}

impl Iterator for Bars {
	type Item = Vec<Moment>;

	fn next(&mut self) -> Option<Self::Item> {
		// Check if start of the bar has time signature.
		let first = self.buf.pop_front()?;
		if let Some(time_sig) = find_time_sig(&first) {
			self.time_sig = time_sig;
		}
		if self.buf.is_empty() {
			return Some(vec![first]);
		}

		let len_32nd = self.tpb / 8.0;
		let chunk_len = (self.time_sig.bar_32s() * len_32nd) as usize;
		let mut temp = Vec::with_capacity(chunk_len);
		temp.push(first);

		for _ in 1..chunk_len {
			if let Some(moment) = self.buf.pop_front() {
				let time_sig = find_time_sig(&moment);
				temp.push(moment);
				if let Some(time_sig) = time_sig {
					if time_sig != self.time_sig {
						self.time_sig = time_sig;
						break;
					}
				}
			} else {
				break;
			}
		}

		Some(temp)
	}
}

impl Sheet {
	/// Returns an iterator that yields measures (bars) from this sheet.
	///
	/// # Arguments
	/// - `ticks_per_beat`: Obtained from a [Header](midly::Header), same value
	///   used for constructing a [Ticker](crate::timers::Ticker).
	pub fn into_bars(self, ticks_per_beat: u16) -> Bars {
		Bars {
			tpb: ticks_per_beat as f32,
			time_sig: TimeSignature {
				numerator: 4,
				denominator: 4,
			},
			// beat_32s: 24,
			buf: self.0.into(),
		}
	}
}

fn find_time_sig(m: &Moment) -> Option<TimeSignature> {
	for e in &m.events {
		if let Event::TimeSignature(n, d, ..) = e {
			return Some(TimeSignature {
				numerator: *n,
				denominator: *d,
			});
		}
	}
	None
}
