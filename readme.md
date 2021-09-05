# plmidi

A library and a command for playing MIDI files.

# Installation

```shell
# to install after a git clone
git clone https://github.com/insomnimus/plmidi
cd plmidi
git checkout main
cargo install --path . --locked

# here's a one liner:
# cargo install --locked --branch main --git https://github.com/insomnimus/plmidi
```

# Command Usage

``òutput
plmidi 0.1.0

Play MIDI files.

USAGE:
    plmidi <file> [SUBCOMMAND]

ARGS:
    <file>    A MIDI file (.mid) to be played.

OPTIONS:
    -d, --device <device>    The index of the MIDI device that will be used for synthesis. [default:
                             0]
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    list    Show available MIDI devices. [aliases: ls]
```

# TODO

Library usage and documentation coming soon.
