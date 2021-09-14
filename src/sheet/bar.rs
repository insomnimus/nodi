use std::collections::VecDeque;

use crate::{Event, Moment, Sheet};

struct TimeSignature {
    // Beats per bar.
    nominator: u8,
    // Note of a beat. A negative power of 2.
    denominator: u8,
}

impl TimeSignature {
    fn bar_32s(&self) -> f32 {
        let note_as_32s = 2_f32.powi(5_i32 - self.denominator as i32);
        // let note_as_32s = 32.0 / self.denominator as f32;
        self.nominator as f32 * note_as_32s
    }
}

pub struct Bars {
    time_sig: TimeSignature,
    beat_32s: u8,
    tpb: f32,
    buf: VecDeque<Moment>,
}

impl Iterator for Bars {
    type Item = Vec<Moment>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }
        let len_32nd = self.tpb / self.beat_32s as f32;
        let chunk_len = (self.time_sig.bar_32s() * len_32nd as f32) as usize;
        let mut temp = Vec::with_capacity(chunk_len);

        for _ in 0..chunk_len {
            if let Some(moment) = self.buf.pop_front() {
                match &moment {
                    Moment::Empty => temp.push(moment),
                    Moment::Events(events) => {
                        if let Some((&nominator, &denominator, &beat_32s)) = events
                            .iter()
                            .flat_map(|e| match e {
                                Event::TimeSignature(nom, denom, _, beat_32s) => {
                                    Some((nom, denom, beat_32s))
                                }
                                _ => None,
                            })
                            .next()
                        {
                            self.time_sig = TimeSignature {
                                nominator,
                                denominator,
                            };
                            self.beat_32s = beat_32s;
                            temp.push(moment);
                            break;
                        } else {
                            temp.push(moment);
                        }
                    }
                }
            } else {
                break;
            }
        }

        Some(temp)
    }
}

impl Sheet {
    /// Returns an iterator that yields measures (bars) from this sheet.
    ///
    /// # Arguments
    /// - `ticks_per_beat`: Obtained from a [Header](midly::Header), same value
    ///   used for constructing a [Ticker](crate::Ticker).
    pub fn into_bars(self, ticks_per_beat: u16) -> Bars {
        Bars {
            tpb: ticks_per_beat as f32,
            time_sig: TimeSignature {
                nominator: 4,
                denominator: 4,
            },
            beat_32s: 24,
            buf: self.0.into(),
        }
    }
}
