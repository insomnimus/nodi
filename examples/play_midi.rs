use std::{convert::TryFrom, error::Error, fs};

use clap::{App, Arg};
use midir::{MidiOutput, MidiOutputConnection};
use midly::{Format, Smf};
use nodi::{Player, Sheet, Ticker};

struct Args {
    file: String,
    device_no: usize,
    list: bool,
}

impl Args {
    fn from_args() -> Self {
        let m = App::new("play_midi")
            .about("An example midi player.")
            .arg(
                Arg::new("device")
                    .about("The midi device that will be used for synthesis.")
                    .default_value("0")
                    .short('d')
                    .long("device")
                    .validator(|s| {
                        s.parse::<usize>()
                            .map(|_| {})
                            .map_err(|_| String::from("the value must be a non-negative integer"))
                    }),
            )
            .arg(
                Arg::new("list")
                    .about("List available midi devices.")
                    .short('l')
                    .long("list"),
            )
            .arg(
                Arg::new("file")
                    .about("A .mid file to play.")
                    .required_unless_present("list"),
            )
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

        let mut player = Player::new(con, timer);

        println!("starting playback");
        player.play_sheet(&sheet);
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
