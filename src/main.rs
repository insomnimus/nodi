use midir::{
	MidiOutput,
	MidiOutputConnection,
};
use midly::{
	Smf,
	Timing,
};
use plmidi::{
	Player,
	Sheet,
	Ticker,
};

type Error = Box<dyn std::error::Error>;

static DATA: &[u8] = include_bytes!("in_flames.mid");

fn get_midi() -> Result<MidiOutputConnection, Error> {
	let midi_out = MidiOutput::new("kb-drums output")?;

	let out_ports = midi_out.ports();
	let out_port = &out_ports[0];
	let out = midi_out.connect(out_port, "cello-tabs")?;
	Ok(out)
}

fn play_track() -> Result<(), Error> {
	let Smf { header, tracks } = Smf::parse(DATA)?;

	let ticks_per_beat = match header.timing {
		Timing::Metrical(n) => u16::from(n),
		_ => return Err("unsupported time format".into()),
	};

	let out = get_midi()?;
	let timer = Ticker::new(ticks_per_beat);
	let sheet = Sheet::parallel(tracks);
	let mut player = Player::new(out, timer);
	player.play_sheet(&sheet);
	Ok(())
}

fn main() -> Result<(), Error> {
	play_track()
}
