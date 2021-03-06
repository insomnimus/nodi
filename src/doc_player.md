A type that orchestrates playing of MIDI tracks.

`Player` is a struct with two requirements:

-  A [Timer] for properly timing a sleep.
-  A [Connection] to send the MIDI events when they are ready to play.

So, [Player] is the glue that binds timing and playback.
> This type is more of a convenience struct; it cannot possibly satisfy all use cases.

# Implementation Details
In this section, the `"track"` refers to either a [Sheet][crate::Sheet] or a slice of [Moment]s.

This type orchestrates playback of tracks.
There are some things that are assumed:

1.  Every moment in the given sheet is offset by exactly 1 MIDI tick.
2.  The provided [Timer] is assumed to be aware of #1 above.

The implementation of [Player::play] is roughly as follows:

1. Initialize a counter that increments by 1 every tick and resets to 0 wwhenever there is a non-empty [Moment].
2. Start iterating over the provided track, incrementing the counter every iteration (tick).
3. Whenever the iterated value is a non-empty [Moment], check to see if there are any tempo change events.
4. If the event is a tempo change, call [Timer::change_tempo], if it's a MIDI event, call [Connection::play].
5. Repeat until the iteration is complete.
