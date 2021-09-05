use std::mem;

use midly::{
	TrackEvent,
	TrackEventKind,
};

pub type Tracks<'a> = [Vec<TrackEvent<'a>>];
type Offsets<'a> = Vec<Vec<TrackEventKind<'a>>>;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Moment<'a> {
	pub(crate) delta: u32,
	pub(crate) events: Vec<TrackEventKind<'a>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Sheet<'a>(pub(crate) Vec<Moment<'a>>);

impl<'a> Sheet<'a> {
	pub fn parallel(tracks: &Tracks<'a>) -> Self {
		let offsets = tracks.iter().map(|t| map_moments(t)).collect::<Vec<_>>();
		Self::merge_moments(offsets)
	}

	pub fn sequential(tracks: &Tracks<'a>) -> Self {
		let mut s = Self::default();
		for t in tracks.iter().map(|t| map_moments(t)) {
			let t = Self::merge_moments(vec![t]);
			s.append(t);
		}
		s
	}

	pub fn append(&mut self, other: Self) {
		self.0.extend(other.0);
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	fn merge_moments(mut offsets: Vec<Offsets<'a>>) -> Self {
		let cap = offsets.iter().map(|o| o.len()).max().unwrap();
		let mut moments = Vec::with_capacity(cap);
		let mut empty_counter = 0_u32;

		for i in 0..cap {
			let mut merged_moments = Vec::new();
			for track in &mut offsets {
				if i < track.len() {
					let moment = mem::take(&mut track[i]);
					if !moment.is_empty() {
						merged_moments.extend(moment);
					}
				}
			}

			if merged_moments.is_empty() {
				empty_counter += 1;
			} else {
				moments.push(Moment {
					delta: empty_counter,
					events: merged_moments,
				});
				empty_counter = 0;
			}
		}

		Self(moments)
	}
}

fn map_moments<'a>(events: &[TrackEvent<'a>]) -> Offsets<'a> {
	let total_frames: usize = events.iter().map(|e| u32::from(e.delta) as usize).sum();
	let mut map: Offsets = vec![Vec::new(); total_frames + 1];
	let mut cur_pos = 0_usize;

	for event in events {
		cur_pos += u32::from(event.delta) as usize;
		map[cur_pos].push(event.kind);
	}

	map
}
