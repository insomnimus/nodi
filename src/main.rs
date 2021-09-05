mod app;

use std::{
	error::Error,
	fs,
	process,
};

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

fn list_devices() -> Result<(), Box<dyn Error>> {
	let midi_out = MidiOutput::new("plmidi")?;

	let out_ports = midi_out.ports();

	if out_ports.is_empty() {
		println!("No active MIDI output device detected.");
	} else {
		for (i, p) in out_ports.iter().enumerate() {
			println!(
				"#{}: {}",
				i,
				midi_out
					.port_name(p)
					.as_deref()
					.unwrap_or("<no device name>")
			);
		}
	}

	Ok(())
}

fn get_midi(n: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
	let midi_out = MidiOutput::new("plmidi")?;

	let out_ports = midi_out.ports();
	if out_ports.is_empty() {
		return Err("no MIDI output device detected".into());
	}
	if n >= out_ports.len() {
		return Err(format!(
			"only {} MIDI devices detected; run `plmidi list` to see them",
			out_ports.len()
		)
		.into());
	}

	let out_port = &out_ports[n];
	let out = midi_out.connect(out_port, "cello-tabs")?;
	Ok(out)
}

fn run() -> Result<(), Box<dyn Error>> {
	let m = app::new().get_matches();
	match m.subcommand_name() {
		None => (),
		Some("list") => return list_devices(),
		Some(unknown) => panic!("unhandled subcommand case: {:?}", unknown),
	};

	let n_device = m.value_of("device").unwrap().parse::<usize>().unwrap();
	let file_name = m.value_of("file").unwrap();

	let out = get_midi(n_device)?;
	let data = fs::read(file_name)?;

	let Smf { header, tracks } = Smf::parse(&data)?;
	let ticks_per_beat = match header.timing {
		Timing::Metrical(n) => u16::from(n),
		_ => return Err("unsupported time format".into()),
	};

	let timer = Ticker::new(ticks_per_beat);
	let sheet = Sheet::parallel(tracks);
	let mut player = Player::new(out, timer);
	player.play_sheet(&sheet);
	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
		process::exit(2);
	}
}
