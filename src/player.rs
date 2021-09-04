use log::error;
use midir::MidiOutputConnection;
use midly::{
	MetaMessage,
	TrackEventKind,
};

use crate::{
	Sheet,
	Timer,
};

pub struct Player<T: Timer> {
	pub con: MidiOutputConnection,
	timer: T,
}

impl<T: Timer> Player<T> {
	pub fn new(con: MidiOutputConnection, timer: T) -> Self {
		Self { con, timer }
	}

	pub fn set_timer(&mut self, timer: T) {
		self.timer = timer;
	}

	pub fn play_sheet(&mut self, sheet: &Sheet) {
		for moment in &sheet.0 {
			self.timer.sleep(moment.delta);
			for event in &moment.events {
				match event {
					TrackEventKind::Meta(MetaMessage::Tempo(val)) => {
						self.timer.change_tempo(u32::from(*val))
					}
					_ => {
						if let Some(msg) = event.as_live_event() {
							let mut buf = Vec::new();
							let _ = msg.write_std(&mut buf);
							if let Err(e) = self.con.send(&buf) {
								error!("failed to send a midi message: {:?}", e);
							}
						}
					}
				}
			}
		}
	}
}
