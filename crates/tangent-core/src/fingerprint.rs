//! Symbolic, transposition-invariant melodic fingerprinting.
//!
//! Audio transcription and polyphonic alignment are intentionally out of scope.
//! The database adapter persists these portable models; all matching stays in Rust.
use crate::music::Song;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Four chromatic intervals (five notes) per inverted-index entry.
pub const INTERVALS_PER_FINGERPRINT: usize = 4;
pub const MIN_QUERY_NOTES: usize = INTERVALS_PER_FINGERPRINT + 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fingerprint {
    /// Hex-encoded biased signed semitone intervals, independent of absolute pitch.
    pub signature: String,
    /// Zero-based position of the first note within this melody lane.
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MelodyIndex {
    pub part: usize,
    pub keys: Vec<u8>,
    pub onsets_us: Vec<u64>,
    pub fingerprints: Vec<Fingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintedSong {
    pub title: String,
    pub melodies: Vec<MelodyIndex>,
}

/// SQLite returns candidate lanes, not complete MusicXML documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub song_id: String,
    pub title: String,
    pub part: usize,
    pub keys: Vec<u8>,
    pub onsets_us: Vec<u64>,
    pub hits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub song_id: String,
    pub title: String,
    pub part: usize,
    /// Start of the matched passage within the melody lane.
    pub note_index: usize,
    pub onset_us: u64,
    /// Normalized interval-edit similarity, NOT a calibrated probability.
    pub similarity: f32,
    pub fingerprint_hits: usize,
}

/// Extract one reproducible melodic lane per non-percussion part. On simultaneous
/// pitches, the highest wins. This is a deliberate monophonic MVP heuristic;
/// it is not a full polyphonic/voice-separation algorithm.
pub fn index_song(song: &Song) -> FingerprintedSong {
    let mut lanes: BTreeMap<usize, BTreeMap<u64, u8>> = BTreeMap::new();
    for note in &song.notes {
        if song.parts.get(note.part).is_none_or(|part| part.percussion) || note.key > 127 {
            continue;
        }
        lanes
            .entry(note.part)
            .or_default()
            .entry(note.start_us)
            .and_modify(|key| *key = (*key).max(note.key))
            .or_insert(note.key);
    }
    let melodies = lanes
        .into_iter()
        .filter_map(|(part, notes)| {
            if notes.len() < MIN_QUERY_NOTES {
                return None;
            }
            let (onsets_us, keys): (Vec<_>, Vec<_>) = notes.into_iter().unzip();
            let fingerprints = fingerprints(&keys);
            Some(MelodyIndex {
                part,
                keys,
                onsets_us,
                fingerprints,
            })
        })
        .collect();
    FingerprintedSong {
        title: song.title.clone(),
        melodies,
    }
}

/// Every overlapping five-note window gets one portable index signature.
/// Interval values [-127,127] are biased to [0,254] before hex encoding.
pub fn fingerprints(keys: &[u8]) -> Vec<Fingerprint> {
    keys.windows(MIN_QUERY_NOTES)
        .enumerate()
        .map(|(position, window)| {
            let signature = window
                .windows(2)
                .map(|pair| format!("{:02x}", (pair[1] as i16 - pair[0] as i16 + 127) as u8))
                .collect::<String>();
            Fingerprint {
                signature,
                position,
            }
        })
        .collect()
}

/// Deduplicated lookup keys for the SQL inverted index.
pub fn signatures(keys: &[u8]) -> Vec<String> {
    fingerprints(keys)
        .into_iter()
        .map(|fp| fp.signature)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

const GAP: f32 = 0.85;

/// Align a query to any substring of a candidate's *interval* sequence.
/// Substitutions and missed/extra notes are allowed. No absolute pitch is used.
/// The returned index addresses the first matched note in the candidate.
fn align_intervals(query: &[u8], candidate: &[u8]) -> Option<(usize, f32)> {
    if query.len() < MIN_QUERY_NOTES || candidate.len() < MIN_QUERY_NOTES {
        return None;
    }
    let a: Vec<i16> = query
        .windows(2)
        .map(|w| w[1] as i16 - w[0] as i16)
        .collect();
    let b: Vec<i16> = candidate
        .windows(2)
        .map(|w| w[1] as i16 - w[0] as i16)
        .collect();
    let mut prev_cost = vec![0.0f32; b.len() + 1];
    let mut prev_start: Vec<usize> = (0..=b.len()).collect();
    for (i, &interval) in a.iter().enumerate() {
        let mut cost = vec![0.0f32; b.len() + 1];
        let mut start = vec![0usize; b.len() + 1];
        cost[0] = (i + 1) as f32 * GAP;
        for (j, &other) in b.iter().enumerate() {
            let delta = (interval - other).abs();
            let substitution = if delta == 0 {
                0.0
            } else if delta <= 2 {
                0.55
            } else {
                1.0
            };
            let diagonal = (prev_cost[j] + substitution, prev_start[j]);
            let deletion = (prev_cost[j + 1] + GAP, prev_start[j + 1]);
            let insertion = (cost[j] + GAP, start[j]);
            let best = [diagonal, deletion, insertion]
                .into_iter()
                .min_by(|x, y| x.0.total_cmp(&y.0))
                .unwrap();
            cost[j + 1] = best.0;
            start[j + 1] = best.1;
        }
        prev_cost = cost;
        prev_start = start;
    }
    let (end, distance) = prev_cost
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))?;
    let begin = prev_start[end].min(candidate.len() - 1);
    let similarity = (1.0 - *distance / a.len() as f32).clamp(0.0, 1.0);
    Some((begin, similarity))
}

/// Re-rank SQL candidates using fuzzy subsequence alignment.
/// Score is a similarity heuristic, not a recognition confidence/probability.
pub fn rank_matches(query: &[u8], candidates: &[Candidate], limit: usize) -> Vec<Match> {
    if query.len() < MIN_QUERY_NOTES || limit == 0 {
        return vec![];
    }
    let mut results: Vec<Match> = candidates
        .iter()
        .filter_map(|candidate| {
            let (note_index, similarity) = align_intervals(query, &candidate.keys)?;
            if similarity < 0.58 {
                return None;
            }
            Some(Match {
                song_id: candidate.song_id.clone(),
                title: candidate.title.clone(),
                part: candidate.part,
                note_index,
                onset_us: candidate.onsets_us.get(note_index).copied().unwrap_or(0),
                similarity,
                fingerprint_hits: candidate.hits,
            })
        })
        .collect();
    results.sort_by(|a, b| {
        b.similarity
            .total_cmp(&a.similarity)
            .then_with(|| b.fingerprint_hits.cmp(&a.fingerprint_hits))
            .then_with(|| a.song_id.cmp(&b.song_id))
            .then_with(|| a.part.cmp(&b.part))
    });
    results.truncate(limit);
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::{Part, PerformanceNote, Song};

    fn song(lanes: &[(&[u8], bool)]) -> Song {
        let mut notes = vec![];
        let mut parts = vec![];
        for (part, (keys, percussion)) in lanes.iter().enumerate() {
            parts.push(Part {
                name: format!("Part {part}"),
                percussion: *percussion,
            });
            for (index, key) in keys.iter().enumerate() {
                notes.push(PerformanceNote {
                    id: notes.len(),
                    part,
                    key: *key,
                    velocity: 90,
                    start_us: index as u64 * 250_000,
                    end_us: (index as u64 + 1) * 250_000,
                    written: None,
                });
            }
        }
        Song {
            title: "Fixture".into(),
            duration_us: 3_000_000,
            parts,
            notes,
            notation: None,
            warnings: vec![],
            events: vec![],
        }
    }

    fn candidate(keys: &[u8]) -> Candidate {
        Candidate {
            song_id: "fixture".into(),
            title: "Fixture".into(),
            part: 0,
            keys: keys.to_vec(),
            onsets_us: (0..keys.len()).map(|i| i as u64 * 250_000).collect(),
            hits: 1,
        }
    }

    #[test]
    fn octave_and_key_transposition_produce_identical_signatures() {
        let original = [76, 75, 76, 75, 76, 71, 74, 72, 69];
        let lower = [64, 63, 64, 63, 64, 59, 62, 60, 57];
        let shifted = [78, 77, 78, 77, 78, 73, 76, 74, 71];
        assert_eq!(fingerprints(&original), fingerprints(&lower));
        assert_eq!(fingerprints(&original), fingerprints(&shifted));
        assert_eq!(
            rank_matches(&lower, &[candidate(&original)], 1)[0].similarity,
            1.
        );
    }

    #[test]
    fn matches_mid_song_with_wrong_note_and_missed_note() {
        let whole = [50, 52, 55, 57, 60, 59, 60, 59, 60, 55, 58, 57, 53, 55, 57];
        let wrong = [72, 71, 72, 71, 73, 67, 70, 69, 65]; // +12 except one error
        let hits = rank_matches(&wrong, &[candidate(&whole)], 3);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].note_index, 4);
        assert!(hits[0].similarity > 0.65);
        let missing = [72, 71, 72, 67, 70, 69, 65];
        assert!(!rank_matches(&missing, &[candidate(&whole)], 1).is_empty());
    }

    #[test]
    fn percussion_excluded_and_chords_choose_top_voice() {
        let mut s = song(&[
            (&[60, 62, 64, 65, 67, 69], false),
            (&[35, 38, 42, 36, 42, 38], true),
        ]);
        s.notes.push(PerformanceNote {
            id: 12,
            part: 0,
            key: 48,
            velocity: 80,
            start_us: 0,
            end_us: 200_000,
            written: None,
        });
        let idx = index_song(&s);
        assert_eq!(idx.melodies.len(), 1);
        assert_eq!(idx.melodies[0].keys[0], 60);
        assert_eq!(idx.melodies[0].fingerprints.len(), 2);
    }

    #[test]
    fn short_queries_and_unrelated_patterns_are_rejected() {
        assert!(signatures(&[60, 62, 64]).is_empty());
        assert!(rank_matches(&[60, 62, 64], &[candidate(&[60, 62, 64, 65, 67])], 1).is_empty());
        assert!(
            rank_matches(
                &[60, 60, 60, 60, 60, 60],
                &[candidate(&[60, 72, 48, 72, 48, 72])],
                1
            )
            .is_empty()
        );
    }

    #[test]
    fn stable_order_and_empty_catalog() {
        assert!(rank_matches(&[60, 61, 62, 63, 64], &[], 5).is_empty());
        let c = candidate(&[60, 61, 62, 63, 64]);
        assert_eq!(rank_matches(&[48, 49, 50, 51, 52], &[c], 1).len(), 1);
    }
}
