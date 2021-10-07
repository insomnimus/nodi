Various [Timer] implementations.

# Picking The Right Timer
This depends on the MIDI file you want to play, specifically the type of
[Timing] you get from parsing a file (using [midly]).

Most of the time what you want is a [Ticker]; it comes in two flavours:
- [Ticker]: Provides a metrical [Timer], but you can't control playback.
- [ControlTicker]: Same with [Ticker], but provides a channel for you to play/pause the playback by sending messages to it.

These timers are appropriate when the MIDI file header specifies the timing as being metrical ([Timing::Metrical]).

In the rare case that the timing is not metrical but [Timing::Timecode], you can use [FixedTempo].

# Obtaining a Timer
[Ticker] and [FixedTempo] implement [TryFrom]\<[Timing]\>.

## Examples
Obtaining a timer:

```ignore
use std::convert::TryFrom;
use nodi::{Timer, timers::{Ticker, FixedTempo}};
use midly::{Smf, Timing};

// Assume `data` contains the bytes of our MIDI file (.smf).
let data = Vec::new();
// Parse the data.
let Smf{tracks, header} = Smf::parse(&data)?;

// This works 99% of the time since the metrical timing is used most often.
let ticker = Ticker::try_from(header.timing)?;

// However we could be safe and do it like the following.
// Notice that we have to Box the value this time, because the return types are different.
let timer: Box<dyn Timer> = match header.timing {
  Timing::Metrical(_) => Box::new(Ticker::try_from(header.timing)?),
  Timing::Timecode(..) => Box::new(FixedTempo::try_from(header.timing)?),
};

// Use the timer
```