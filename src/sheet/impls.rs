use core::{
	borrow::Borrow,
	convert::TryFrom,
	iter::IntoIterator,
	ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo},
};

use midly::TrackEvent;

use crate::{Event, Moment, Sheet};

impl Extend<Moment> for Sheet {
	fn extend<T: IntoIterator<Item = Moment>>(&mut self, moments: T) {
		self.0.extend(moments);
	}
}

impl IntoIterator for Sheet {
	type IntoIter = std::vec::IntoIter<Self::Item>;
	type Item = Moment;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
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

impl IndexMut<Range<usize>> for Sheet {
	fn index_mut(&mut self, r: Range<usize>) -> &mut Self::Output {
		&mut self.0[r]
	}
}

impl IndexMut<RangeFrom<usize>> for Sheet {
	fn index_mut(&mut self, r: RangeFrom<usize>) -> &mut Self::Output {
		&mut self.0[r]
	}
}

impl IndexMut<RangeTo<usize>> for Sheet {
	fn index_mut(&mut self, r: RangeTo<usize>) -> &mut Self::Output {
		&mut self.0[r]
	}
}

impl IndexMut<RangeFull> for Sheet {
	fn index_mut(&mut self, r: RangeFull) -> &mut Self::Output {
		&mut self.0[r]
	}
}

impl Index<Range<usize>> for Sheet {
	type Output = [Moment];

	fn index(&self, r: Range<usize>) -> &Self::Output {
		&self.0[r]
	}
}

impl Index<RangeFrom<usize>> for Sheet {
	type Output = [Moment];

	fn index(&self, r: RangeFrom<usize>) -> &Self::Output {
		&self.0[r]
	}
}

impl Index<RangeTo<usize>> for Sheet {
	type Output = [Moment];

	fn index(&self, r: RangeTo<usize>) -> &Self::Output {
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

impl Index<RangeFull> for Sheet {
	type Output = [Moment];

	fn index(&self, r: RangeFull) -> &Self::Output {
		&self.0[r]
	}
}

impl Borrow<[Moment]> for Sheet {
	fn borrow(&self) -> &[Moment] {
		&self.0[..]
	}
}
