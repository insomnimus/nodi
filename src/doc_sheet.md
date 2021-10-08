Holds every moment in a MIDI track, each moment representing a MIDI tick.

This type is used for time-mapping a MIDI track.

A [Sheet] is a thin wrapper over `Vec<Moment>`.
Every item ([Moment]) contained will last one MIDI tick long.
The length of a tick depends on the header in the file and the tempo change events contained in each track.
Therefore this type makes no assumptions about the actual duration but works with MIDI ticks, which are the smallest time units in a MIDI file.

A [Sheet] can be iterated over by using [`.iter()`](Sheet::iter), 
using `&sheet[..]` or directly calling [`.into_iter()`](Sheet::into_iter).

# Examples

```no_run
use midly::{Format, Smf};
use nodi::Sheet;
// Assume `data` contains the bytes of some MIDI file (.mid).
let data = Vec::new();
// We can parse the file using the midly crate:
let Smf{header, tracks} = Smf::parse(&data)?;

// Since a Sheet is header independant, we can construct it from `tracks`.
// However MIDI files may specify how the tracks are to be played, so we still read the header
// for an appropriate representation.
let sheet = match header.format {
    // Here we use `sequential`  to be fail-proof, because a file
    // can specify the format as single and still have multiple tracks in it.
    Format::SingleTrack => Sheet::sequential(&tracks),
    Format::Sequential=> Sheet::sequential(&tracks), // This concatenates each track into one.
    Format::Parallel => Sheet::parallel(&tracks), // This merges every track into one.
};

// Do stuff with the sheet.
# Ok::<(), Box<dyn std::error::Error>>(())
```