use std::{
	convert::TryFrom,
	io,
};

use midly::{
	live::LiveEvent,
	num::u4,
	MetaMessage,
	MidiMessage,
	TrackEventKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Moment {
	Empty,
	Events(Vec<Event>),
}

impl Default for Moment {
	fn default() -> Self {
		Self::Empty
	}
}

impl Moment {
	pub fn push(&mut self, e: Event) {
		match self {
			Self::Events(events) => events.push(e),
			Self::Empty => {
				*self = Self::Events(vec![e]);
			}
		};
	}

	pub fn is_empty(&self) -> bool {
		*self == Self::Empty
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Event {
	Tempo(u32),
	TimeSignature(u8, u8, u8, u8),
	KeySignature(i8, bool),
	Midi(MidiEvent),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MidiEvent {
	channel: u4,
	pub message: MidiMessage,
}

impl MidiEvent {
	pub fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
		let msg = LiveEvent::Midi {
			channel: self.channel,
			message: self.message,
		};
		msg.write_std(w)
	}
}

impl<'a> TryFrom<TrackEventKind<'a>> for Event {
	type Error = &'static str;

	fn try_from(event: TrackEventKind<'_>) -> Result<Self, Self::Error> {
		Ok(match event {
			TrackEventKind::Midi { channel, message } => Self::Midi(MidiEvent { channel, message }),
			TrackEventKind::Meta(MetaMessage::Tempo(n)) => Self::Tempo(u32::from(n)),
			TrackEventKind::Meta(MetaMessage::TimeSignature(a, b, c, d)) => {
				Self::TimeSignature(a, b, c, d)
			}
			TrackEventKind::Meta(MetaMessage::KeySignature(a, b)) => Self::KeySignature(a, b),
			_ => return Err("not a valid Event"),
		})
	}
}
