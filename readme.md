![Build Status](https://github.com/insomnimus/nodi/actions/workflows/main.yml/badge.svg)

# nodi
Nodi provides types and functions for abstracting MIDI files.
 
This crate works alongside with the [midly][] and [midir][] crates. 
However you can implement your own MIDI player, instead of relying on [midir][].

# Features

-	Time-map MIDI events.
-	Join or merge multiple MIDI tracks.
-	Play MIDI files.
 -	Split a MIDI track into measures/bars.
-	Transpose a track.

# Examples

Check out `/examples/play_midi.rs` for a basic midi player.

I started developping this crate because I needed the features it now offers.
Here are some real-world examples of nodi in action:

-	[midnote][]: An accessible MIDI note viewer/ player.
-	[plmidi][]: A MIDI player for the command line.

# Debugging
Nodi uses the [log][] crate for the logging; you can use a compatible logger for consumption.

# Crate Features
No feature is enabled by default.

-  `midir`: Adds implementations of `Connection` for `midir::MidiOutputConnection`.

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
[midnote]: https://github.com/insomnimus/midnote
[log]: https://docs.rs/log/0.4.14/log/
