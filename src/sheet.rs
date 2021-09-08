use midly::TrackEvent;

use crate::event::Moment;

mod impls;

#[doc = include_str!("doc_sheet.md")]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sheet(pub(crate) Vec<Moment>);

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
	///
	/// Note that multiplying this value with the length of a tick may not
	/// always give you the correct total duration. The reason for this is a
	/// MIDI file can change tempo mid-track, however it is still trivial to
	/// calculate the duration since every tempo-change event will be contained
	/// in `self`.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `Self::len() == 0`.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Appends another [Sheet] to `self`, destroying the other.
	///
	/// # Remarks
	/// This method will join two tracks end to end. If you want to merge them
	/// instead, see [merge_with](Sheet::merge_with).
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

	/// Returns an iterator over every moment in `self`.
	pub fn iter(&self) -> std::slice::Iter<'_, Moment> {
		self.0.iter()
	}
}
