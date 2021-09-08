This crate provides types and functions for abstracting MIDI files.

This crate works alongside with the [midly][] and [midir][] crates.
However you can implement your own [Connection], instead of using [midir][].

# A Basic Workflow

- Use [midly][] to parse a MIDI file.
- Create a [Ticker] from the `header`.
- Create a [Sheet] from the parsed [tracks](midly::Track).
- Initialize a MIDI connection using [midir][].
- Create a [Player] from the connection and the timer.
- Play the sheet using the player.

# Examples

Check out `/examples/play_midi.rs` for a basic midi player.

For a little more complicated example please check out the source code of [plmidi][] for an implementation.


# Crate Features
No feature is enabled by default.

-  `midir`: Adds implementations of [Connection] for [midir::MidiOutputConnection].

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
