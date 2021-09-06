use std::{
	convert::TryFrom,
	ops::{
		Index,
		IndexMut,
		Range,
	},
};

use midly::TrackEvent;

use crate::event::{
	Event,
	Moment,
};

/// Holds every moment in a MIDI track, each moment representing a MIDI tick.
///
/// This type is used for time-mapping a MIDI track.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sheet(pub(crate) Vec<Moment>);

impl Index<usize> for Sheet {
	type Output = Moment;

	fn index(&self, n: usize) -> &Self::Output {
		&self.0[n]
	}
}

impl IndexMut<usize> for Sheet {
	fn index_mut(&mut self, n: usize) -> &mut Self::Output {
		&mut self.0[n]
	}
}

impl Index<Range<usize>> for Sheet {
	type Output = [Moment];

	fn index(&self, r: Range<usize>) -> &Self::Output {
		&self.0[r]
	}
}

impl<'a> From<&[TrackEvent<'a>]> for Sheet {
	fn from(events: &[TrackEvent<'_>]) -> Self {
		let total_frames = events
			.iter()
			.map(|e| u32::from(e.delta) as usize)
			.sum::<usize>()
			+ 1;
		let mut buf = vec![Moment::Empty; total_frames];

		let mut cur_pos = 0_usize;

		for event in events {
			cur_pos += u32::from(event.delta) as usize;
			if let Ok(e) = Event::try_from(event.kind) {
				buf[cur_pos].push(e);
			}
		}

		Self(buf)
	}
}

impl Sheet {
	/// Creates a [Sheet] from a slice of [TrackEvent]s.
	///
	/// # Remarks
	/// Use this when the MIDI file header specifies the format to be 0, meaning
	/// `single`.
	pub fn single(events: &[TrackEvent<'_>]) -> Self {
		events.into()
	}

	/// Creates a [Sheet] from many tracks, merging all of them into one.
	///
	/// # Remarks
	/// Use this when a MIDI file header specifies the format to be of 1,
	/// meaning parallel.
	pub fn parallel(tracks: &[Vec<TrackEvent<'_>>]) -> Self {
		if tracks.is_empty() {
			return Self::default();
		}

		let mut first = Self::from(tracks[0].as_slice());

		for track in &tracks[1..] {
			let sh = Self::from(track.as_slice());
			first.merge_with(sh);
		}
		first
	}

	/// Creates a [Sheet] from every track, appending them end to
	/// end.
	///
	/// # Remarks
	/// Use this when a MIDI file header specifies the type as 2, meaning
	/// `sequential`.
	pub fn sequential(tracks: &[Vec<TrackEvent<'_>>]) -> Self {
		if tracks.is_empty() {
			return Self::default();
		}

		let mut first = Self::from(tracks[0].as_slice());

		for track in &tracks[1..] {
			let sh = Self::from(track.as_slice());
			first.append(sh);
		}
		first
	}

	/// Creates an instance of [Self] from a [Vec] of [Moments](Moment).
	pub fn new(moments: Vec<Moment>) -> Self {
		Self(moments)
	}

	/// Returns the wrapped `Vec<Moment>`, destroying `self`.
	pub fn into_inner(self) -> Vec<Moment> {
		self.0
	}

	/// Returns how many MIDI ticks (or [Moment]s) this [Sheet] has.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `true` if `Self::len() == 0`.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Appends another [Sheet] to `self`, destroying the other.
	///
	/// # Remarks
	/// This method will join two tracks end to end. If you want to merge them
	/// instead, see [Sheet::merge_with].
	pub fn append(&mut self, other: Self) {
		self.0.extend(other.0);
	}

	/// Merges `self` with another [Sheet], destroying the other.
	///
	/// # Remarks
	/// This method will combine every moment in both [Sheet]s into one. If you
	/// want to join them end to end instead, see [Sheet::append].
	pub fn merge_with(&mut self, other: Self) {
		if other.len() > self.len() {
			let n_new = other.len() - self.len();
			self.0.extend((0..n_new).map(|_| Moment::Empty))
		}

		for (i, moment) in other.0.into_iter().enumerate() {
			if let Moment::Events(events) = moment {
				for e in events {
					self[i].push(e);
				}
			}
		}
	}
}
