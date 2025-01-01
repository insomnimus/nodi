use core::{
	borrow::Borrow,
	convert::TryFrom,
	iter::{FromIterator, IntoIterator},
	ops::{Deref, Index, IndexMut},
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

impl From<&[TrackEvent<'_>]> for Sheet {
	fn from(events: &[TrackEvent<'_>]) -> Self {
		let total_frames = events
			.iter()
			.map(|e| u32::from(e.delta) as usize)
			.sum::<usize>()
			+ 1;
		let mut buf = vec![Moment::default(); total_frames];

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

impl Borrow<[Moment]> for Sheet {
	fn borrow(&self) -> &[Moment] {
		&self.0[..]
	}
}

impl FromIterator<Moment> for Sheet {
	fn from_iter<I: IntoIterator<Item = Moment>>(it: I) -> Self {
		Self(Vec::from_iter(it))
	}
}

impl Deref for Sheet {
	type Target = [Moment];
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<I> Index<I> for Sheet
where
	Vec<Moment>: Index<I>,
{
	type Output = <Vec<Moment> as Index<I>>::Output;
	fn index(&self, i: I) -> &Self::Output {
		self.0.index(i)
	}
}

impl<I> IndexMut<I> for Sheet
where
	Vec<Moment>: IndexMut<I>,
{
	fn index_mut(&mut self, i: I) -> &mut Self::Output {
		self.0.index_mut(i)
	}
}
