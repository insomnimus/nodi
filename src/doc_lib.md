This crate provides types and functions for abstracting MIDI files.

This crate works alongside with the [midly][] and [midir][] crates.

# A Basic Workflow

- Use [midly][] to parse a MIDI file.
- Create a [Ticker] from the `header`.
- Create a [Sheet] from the parsed [tracks](midly::Track).
- Initialize a MIDI connection using [midir][].
- Create a [Player] from the connection and the timer.
- Play the sheet using the player.

# Examples

Please check out the source code of [plmidi][] for an implementation.

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
