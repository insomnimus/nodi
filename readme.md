# Nodi

[![Build Status](https://github.com/insomnimus/nodi/actions/workflows/main.yml/badge.svg)](https://github.com/insomnimus/nodi/actions)
 [![crates.io](https://img.shields.io/crates/v/nodi.svg)](https://crates.io/crates/nodi)
 [![docs.rs](https://docs.rs/nodi/badge.svg)](https://docs.rs/nodi/)

Nodi provides types and functions for playback and abstraction of MIDI files.
 
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

# Crate Features
Features enabled by default:

- `hybrid-sleep`: A more accurate sleep, mixing regular sleep with spin locking efficiently. With this feature enabled the default implementations of timers in this crate will use this. Highly recommended for Windows users but it may also increase timing on other platforms.

Optional features:

- `midir`: Adds implementations of `Connection` for `midir::MidiOutputConnection`.
- `midir-jack`: Same with `midir` but uses the Jack backend.
- `midir-winrt`: Same with `midir` but uses the WinRT backend.

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
[midnote]: https://github.com/insomnimus/midnote
