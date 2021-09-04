use std::mem;

use midly::{
	TrackEvent,
	TrackEventKind,
};

pub type Tracks<'a> = Vec<Vec<TrackEvent<'a>>>;
type Offsets<'a> = Vec<Option<Vec<TrackEventKind<'a>>>>;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Moment<'a> {
	pub(crate) delta: u32,
	pub(crate) events: Vec<TrackEventKind<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sheet<'a>(pub(crate) Vec<Moment<'a>>);

impl<'a> Sheet<'a> {
	pub fn parallel(tracks: Tracks<'a>) -> Self {
		let offsets = tracks
			.into_iter()
			.map(|t| map_moments(&t))
			.collect::<Vec<_>>();
		Self::join_moment_offsets(offsets)
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	fn join_moment_offsets(mut offsets: Vec<Offsets<'a>>) -> Self {
		let cap = offsets.iter().map(|o| o.len()).max().unwrap();
		let mut moments = Vec::with_capacity(cap);
		let mut none_counter = 0_u32;

		for i in 0..cap {
			let mut merged_moments = Vec::new();
			for track in &mut offsets {
				if i < track.len() {
					if let Some(moment) = mem::take(&mut track[i]) {
						merged_moments.extend(moment);
					}
				}
			}

			if merged_moments.is_empty() {
				none_counter += 1;
			} else {
				moments.push(Moment {
					delta: none_counter,
					events: merged_moments,
				});
				none_counter = 0;
			}
		}

		Self(moments)
	}
}

fn map_moments<'a>(events: &[TrackEvent<'a>]) -> Offsets<'a> {
	let mut map = Vec::new();
	let mut i = 0_usize;
	while i < events.len() {
		let event = events[i];
		i += 1;
		let mut moment = vec![event.kind];
		map.extend((0..(u32::from(event.delta))).map(|_| None));

		if i >= events.len() {
			map.push(Some(moment));
			break;
		}

		moment.extend(events[i..].iter().take_while(|e| e.delta == 0).map(|e| {
			i += 1;
			e.kind
		}));

		map.push(Some(moment))
	}

	map
}
