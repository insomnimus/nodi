use midly::{
	TrackEvent,
	TrackEventKind,
};

pub type Tracks<'a> = Vec<Vec<TrackEvent<'a>>>;
type Offsets<'a> = Vec<Option<TrackEventKind<'a>>>;

#[derive(Debug, PartialEq, Clone)]
struct Moment<'a> {
	delta: u32,
	events: Vec<TrackEventKind<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sheet<'a>(Vec<Moment<'a>>);

impl<'a> Sheet<'a> {
	pub fn new(tracks: Tracks<'a>) -> Self {
		let offsets = tracks.into_iter().map(map_offsets).collect::<Vec<_>>();
		Self::join_offsets(&offsets)
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	fn join_offsets(offsets: &[Offsets<'a>]) -> Self {
		let mut moments = Vec::new();
		let total_frames = offsets.iter().map(|o| o.len()).max().unwrap();
		let mut none_counter = 0_u32;
		for i in 0..total_frames {
			let mut moment = Vec::new();
			for track in offsets {
				if let Some(Some(event)) = track.get(i) {
					moment.push(*event);
				}
			}

			if moment.is_empty() {
				none_counter += 1;
			} else {
				moments.push(Moment {
					delta: none_counter,
					events: moment,
				});
				none_counter = 0;
			}
		}

		Self(moments)
	}
}

fn map_offsets(events: Vec<TrackEvent<'_>>) -> Offsets {
	let mut map = Vec::new();
	for event in events {
		map.extend((0..u32::from(event.delta)).map(|_| None));

		map.push(Some(event.kind));
	}
	map
}
