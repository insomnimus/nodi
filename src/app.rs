use clap::{
	crate_authors,
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("plmidi")
		.about("Play MIDI files.")
		.version(crate_version!())
		.author(crate_authors!())
		.setting(AppSettings::UnifiedHelpMessage)
		.setting(AppSettings::DisableVersionForSubcommands)
		.subcommand(
			App::new("list")
				.visible_alias("ls")
				.about("Show available MIDI devices."),
		);

	let file = Arg::new("file")
		.required(true)
		.about("A MIDI file (.mid) to be played.");

	let device = Arg::new("device")
		.short('d')
		.long("device")
		.takes_value(true)
		.default_value("0")
		.about("The index of the MIDI device that will be used for synthesis.")
		.validator(|s| {
			s.parse::<usize>()
				.map(|_| {})
				.map_err(|_| String::from("the value must be a number greater than or equal to 0"))
		});

	app.arg(device).arg(file)
}
