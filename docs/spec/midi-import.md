# MIDI import requirements — first slice

Date: 2026-10-24. Implementation authorized by the owner directly on main.
This requirement set was prepared for this repository from the requested MIDI
import capability and the independent references below. Prior source exposure
is recorded in PROVENANCE.md; this is not a certification of a strict clean room
handoff or an attempt to erase that history.

| ID | Requirement |
| --- | --- |
| MIDI-01 | Accept bytes, not filesystem paths or browser objects. Return owned data usable on native and wasm. |
| MIDI-02 | Support standard MIDI file formats 0 and 1 with positive metrical division. Reject independent format-2 sequences and SMPTE timing explicitly until implemented. |
| MIDI-03 | Keep tracks, channels, ports, source order, names, tempo, meter, key and text. Preserve channel/control and opaque system/metadata events; do not execute them. |
| MIDI-04 | Derive note key-down spans from NoteOn/NoteOff, including zero-velocity NoteOn as release. Pair by source track, port, channel and key; diagnose orphan releases, missing releases and ambiguous overlapping same-key attacks. FIFO is the documented overlap convention. |
| MIDI-05 | Compute absolute ticks and microseconds from the complete tempo map. Use default 500,000 microseconds/quarter before an authored tempo. Accumulate fractional time before rounding, never independently truncate each segment. |
| MIDI-06 | Reject conflicting simultaneous tempo declarations and invalid tempo zero. Keep time/key signatures as source metadata, without inventing a written score or hand assignment. |
| MIDI-07 | Bound input bytes, tracks, events and duration; stop lazy parsing at the limits before collecting an unbounded event vector. Report structured errors with source location where available. |
| MIDI-08 | Preserve trailing silence and source note references. Sustain remains a controller event; key-down spans do not claim to be audible or written durations. |
| MIDI-09 | Provide an inspectable native example and a browser file-import harness. Neither is playback, live device input, a synth or the Bevy application. |

Acceptance includes independently generated fixtures for tempo transitions,
channels/ports, repeated notes, sustain preservation, timing fractions, empty
tracks, malformed input, resource bounds and both target builds. New tests and
fixtures are authored here; none are imported from Neothesia or its forks.

References checked 2026-10-24:

- https://midi.org/standard-midi-files
- https://docs.rs/midly/0.5.3/midly/
- https://docs.rs/midly/0.5.3/midly/fn.parse.html
- https://docs.rs/midly/0.5.3/midly/enum.MetaMessage.html
