mod transpose;

use std::{convert::TryFrom, io};

use midly::{live::LiveEvent, num::u4, MetaMessage, MidiMessage, TrackEventKind};

/// Represents a single moment (tick) in a MIDI track.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Moment {
	/// There are no MIDI events at this moment.
	Empty,
	/// There is at least one MIDI event at this moment.
	Events(Vec<Event>),
}

impl Default for Moment {
	fn default() -> Self {
		Self::Empty
	}
}

impl Moment {
	/// Adds an [Event] to `self`.
	/// Will change `self` to [Self::Events] if `self` is [Self::Empty].
	pub fn push(&mut self, e: Event) {
		match self {
			Self::Events(events) => events.push(e),
			Self::Empty => {
				*self = Self::Events(vec![e]);
			}
		};
	}

	/// Returns `true` if `self` is [Self::Empty].
	pub fn is_empty(&self) -> bool {
		*self == Self::Empty
	}
}

/// Represents a single MIDI event.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Event {
	/// Represents a tempo change message.
	/// The wrapped `u32` represents microseconds per beat.
	Tempo(u32),
	/// In order of the MIDI specification, numerator, denominator, MIDI clocks
	/// per click, 32nd notes per quarter.
	TimeSignature(u8, u8, u8, u8),
	/// As in the MIDI specification, negative numbers indicate number of flats
	/// and positive numbers indicate number of sharps. false indicates a major
	/// scale, true indicates a minor scale.
	KeySignature(i8, bool),
	/// Represents a MIDI event.
	Midi(MidiEvent),
}

/// Represents a MIDI message.
/// An instance of this type can sometimes be converted from a [TrackEventKind]
/// with the [TryFrom] trait.
/// This type can be fed to a synthesizer.
///
/// # Examples
/// ```ignore
/// // An instance of this type can be sent to a synthesizer like this.
/// let msg: MidiEvent = /* ... */;
/// let mut buf = Vec::new();
/// msg.write(&mut buf)?;
/// // Now `bf` contains a valid MIDI message, send it to a MIDI api like `midir::MidiOutputConnection`.
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MidiEvent {
	/// The channel this event is to be sent to.
	pub channel: u4,
	/// The message body.
	pub message: MidiMessage,
}

impl MidiEvent {
	/// Writes a valid MIDI message to the given [io::Write].
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

	/// Tries to create [Self] from a [TrackEventKind].
	///
	/// # Errors
	/// Will return an error if the given [TrackEventKind] is not compatible.
	fn try_from(event: TrackEventKind<'_>) -> Result<Self, Self::Error> {
		Ok(match event {
			TrackEventKind::Midi { channel, message } => Self::Midi(MidiEvent { channel, message }),
			TrackEventKind::Meta(MetaMessage::Tempo(n)) => Self::Tempo(u32::from(n)),
			TrackEventKind::Meta(MetaMessage::TimeSignature(a, b, c, d)) => {
				Self::TimeSignature(a, b, c, d)
			}
			TrackEventKind::Meta(MetaMessage::KeySignature(a, b)) => Self::KeySignature(a, b),
			_ => return Err("not a valid event"),
		})
	}
}
