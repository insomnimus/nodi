use std::{convert::TryFrom, error::Error, fs};

use clap::{arg, Command};
use midir::{MidiOutput, MidiOutputConnection};
use nodi::{
	midly::{Format, Smf},
	timers::Ticker,
	Player, Sheet,
};

struct Args {
	file: String,
	device_no: usize,
	list: bool,
}

impl Args {
	fn from_args() -> Self {
		let m = Command::new("play_midi")
			.about("An example midi player.")
			.args(&[
				arg!(-d --device [DEVICE] "Index of the MIDI device to use.")
					.default_value("0")
					.validator(|s| {
						s.parse::<usize>()
							.map(|_| {})
							.map_err(|_| String::from("the value must be a positive integer or 0"))
					}),
				arg!(-l --list "List available MIDI devices."),
				arg!(file: [FILE] "A MIDI file to play.").required_unless_present("list"),
			])
			.get_matches();

		let list = m.is_present("list");
		let device_no = m.value_of("device").unwrap().parse::<usize>().unwrap();
		let file = m.value_of("file").map(String::from).unwrap_or_default();

		Self {
			file,
			device_no,
			list,
		}
	}

	fn run(&self) -> Result<(), Box<dyn Error>> {
		if self.list {
			return list_devices();
		}

		let data = fs::read(&self.file)?;
		let Smf { header, tracks } = Smf::parse(&data)?;
		let timer = Ticker::try_from(header.timing)?;

		let con = get_connection(self.device_no)?;

		let sheet = match header.format {
			Format::SingleTrack | Format::Sequential => Sheet::sequential(&tracks),
			Format::Parallel => Sheet::parallel(&tracks),
		};

		let mut player = Player::new(timer, con);

		println!("starting playback");
		player.play(&sheet);
		Ok(())
	}
}

fn get_connection(n: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
	let midi_out = MidiOutput::new("play_midi")?;

	let out_ports = midi_out.ports();
	if out_ports.is_empty() {
		return Err("no MIDI output device detected".into());
	}
	if n >= out_ports.len() {
		return Err(format!(
			"only {} MIDI devices detected; run with --list  to see them",
			out_ports.len()
		)
		.into());
	}

	let out_port = &out_ports[n];
	let out = midi_out.connect(out_port, "cello-tabs")?;
	Ok(out)
}

fn list_devices() -> Result<(), Box<dyn Error>> {
	let midi_out = MidiOutput::new("play_midi")?;

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

fn main() -> Result<(), Box<dyn Error>> {
	Args::from_args().run()
}
