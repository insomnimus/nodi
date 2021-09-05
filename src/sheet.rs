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

#[derive(Default, Clone, Debug)]
pub struct Sheet(pub(crate) Vec<Moment>);

impl Sheet {
	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

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
	pub fn from_track_events(events: &[TrackEvent<'_>]) -> Self {
		events.into()
	}

	pub fn append(&mut self, other: Self) {
		self.0.extend(other.0);
	}

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
}
