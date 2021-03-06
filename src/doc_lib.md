This crate provides types and functions for abstracting MIDI files.

This crate works alongside with the [midly][] and [midir][] crates.
However you can implement your own [Connection], instead of using [midir][].

# A Basic Workflow

- Use [midly][] to parse a MIDI file. (midly is re-exported).
- Create a [Timer] from the `header`.
- Create a [Sheet] from the parsed [tracks](midly::Track).
- Initialize a MIDI connection using [midir][]. You could also implement Connection on your custom type if you don't want to use midir.
- Create a [Player] from the connection and the timer.
- Play the sheet using the player.

# Examples
Check out `/examples/play_midi.rs` for a basic midi player.

I started developping this crate because I needed the features it now offers.
Here are some real-world examples of nodi in action:

- [midnote][]: An accessible MIDI note viewer/ player.
- [plmidi][]: A MIDI player for the command line.

# Crate Features
Features enabled by default:

- `hybrid-sleep`: A more accurate sleep, mixing regular sleep with spin locking efficiently. With this feature enabled the default implementations of [Timer]s in this crate will use this. Highly recommended for Windows users but it may also increase precision on other platforms.

Optional features:

- `midir`: Adds implementations of `Connection` for `midir::MidiOutputConnection`.
- `midir-jack`: Same with `midir` but uses the Jack backend.
- `midir-winrt`: Same with `midir` but uses the WinRT backend.

[midir]: https://crates.io/crates/midir
[midly]: https://crates.io/crates/midly
[plmidi]: https://github.com/insomnimus/plmidi
[midnote]: https://github.com/insomnimus/midnote
