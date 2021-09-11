# nodi

Nodi provides types and functions for abstracting MIDI files.
 
This crate works alongside with the [midly][] and [midir][] crates. 
However you can implement your own MIDI player, instead of relying on [midir][].

# Features

-	Time-map MIDI events.
-	Join or merge multiple MIDI tracks.
-	Play MIDI files.
 
# Examples

Check out `/examples/play_midi.rs` for a basic midi player.

For a little more complicated example please check out the source code of [plmidi][].

# Crate Features
No feature is enabled by default.

-  `midir`: Adds implementations of `Connection` for `midir::MidiOutputConnection`.

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
