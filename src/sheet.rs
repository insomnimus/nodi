use midly::TrackEvent;

use crate::event::Moment;

mod bar;
mod impls;

pub use bar::Bars;

#[doc = include_str!("doc_sheet.md")]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sheet(pub(crate) Vec<Moment>);

impl Sheet {
	/// Creates a [Sheet] from a slice of [TrackEvent]s.
	///
	/// # Notes
	/// Use this when the MIDI file header specifies the format to be 0, meaning
	/// `single`.
	pub fn single(events: &[TrackEvent<'_>]) -> Self {
		events.into()
	}

	/// Creates a [Sheet] from many tracks, merging all of them into one.
	///
	/// # Notes
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
	/// # Notes
	/// Use this when a MIDI file header specifies the type as 2, meaning
	/// `sequential`.
	pub fn sequential(tracks: &[Vec<TrackEvent<'_>>]) -> Self {
		if tracks.is_empty() {
			return Self::default();
		}

		let mut first = Self::from(tracks[0].as_slice());

		for track in &tracks[1..] {
			let sh = Self::from(track.as_slice());
			first.extend(sh);
		}
		first
	}

	/// Creates a new, blank [Sheet].
	pub fn new() -> Self {
		Self(Vec::new())
	}

	/// Creates a new, blank [Sheet] with the given initial capacity.
	pub fn with_capacity(cap: usize) -> Self {
		Self(Vec::with_capacity(cap))
	}

	/// Destroys `self` yielding the underlying [Vec].
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

	/// Merges `self` with another [Sheet], destroying the other.
	///
	/// # Notes
	/// This method will combine every moment in both [Sheet]s into one. If you
	/// want to join them end to end instead, use [Sheet::extend].
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

	/// Returns an iterator over mutable references to the [Moment]s contained
	/// in `self`.
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Moment> {
		self.0.iter_mut()
	}

	/// Add a single [Moment] at the end of this sheet.
	pub fn push(&mut self, m: Moment) {
		self.0.push(m);
	}

	/// Transposes every note in this sheet.
	///
	/// Applies [Moment::transpose] on every item in `self`. see its
	/// documentation for more info.
	pub fn transpose(&mut self, shift: i8, transpose_ch9: bool) {
		for m in &mut self.0 {
			m.transpose(shift, transpose_ch9);
		}
	}

	/// Returns a slice of every [Moment] in `self`. Equivalent to `&sheet[..]`.
	pub fn as_moments(&self) -> &[Moment] {
		&self.0[..]
	}
}
