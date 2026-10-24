# ADR-0002: Byte-based MIDI import with explicit source timing

- Status: Accepted
- Created / decision date: 2026-10-24
- Delivery: Initial native/WASM import implemented; Bevy integration not implemented
- Requirements: [MIDI import](../spec/midi-import.md)

## Context and decision

The first requested application element is MIDI file import. Input may originate
from a browser File object, native dialog or asset loader. The importer therefore
takes bytes and returns owned data; it does not open paths, request permissions,
connect devices, play sound or depend on Bevy.

Use independently published midly 0.5.3 with `std` and `strict`, without its
parallel feature, for lazy SMF decoding. Keep Tangent's event/note models independent
of midly types. Resource limits are checked while consuming the lazy iterators.

Retain exact source ticks and metrical division as performance coordinates, plus
microsecond positions integrated across the complete tempo map. These ticks do
not claim to be a canonical written-notation model for MusicXML. Do not quantize
or infer spelling, staves, hands or authored difficulty during import.

Preserve channel, port and metadata events. Derived notes describe key-down spans;
pedal is retained separately. Pair within each source track/port/channel/key and
use FIFO with an explicit diagnostic for overlapping unisons. Unmatched releases
and notes closed at track end produce diagnostics, not silently discarded notes.
Channel-mode releases close pending keys on that track/port/channel. A future
playback adapter must interpret the full merged event stream and controller state.

Reject format 2 and SMPTE division explicitly in this slice, rather than merging
independent sequences or applying metrical tempo math to timecode. Conflicting
authored tempos at one tick are errors, including across tracks.

## Consequences and alternatives

The same importer can be tested and consumed on native and wasm. A minimal JSON
WASM boundary and browser harness provide a real target check without committing
to the full Bevy UI or a persistent public data format. JSON is currently an
inspection interface, not a versioned song package or record identity encoding.

A Bevy-specific importer or a native path-based API would couple the first
musical capability to a platform. A synthetic notation model derived from MIDI
would claim information that is not present. Both are avoided here.

See the support contract in [MIDI import](../midi-import.md) and exposure history
in [PROVENANCE.md](../../PROVENANCE.md). Accepted technical direction does not
certify a clean room process.
