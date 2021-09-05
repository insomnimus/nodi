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

```òutput
plmidi 0.1.2

Taylan G├╢kkaya <insomnimus.dev@gmail.com>

Play MIDI files.

USAGE:
    plmidi.exe [OPTIONS] [file]

ARGS:
    <file>    A MIDI file (.mid) to be played.

OPTIONS:
    -d, --device <device>    The index of the MIDI device that will be used for synthesis. [default:
                             0]
    -h, --help               Print help information
    -l, --list               List available MIDI output devices.
    -V, --version            Print version information
```

# TODO

Library usage and documentation coming soon.
