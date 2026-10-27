//! External metadata is a reviewed annotation, never an inferred source identity.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query {
    pub title: String,
    #[serde(default)]
    pub creator: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub provider: String,
    pub recording_id: String,
    pub release_id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub date: String,
    pub country: String,
    pub disambiguation: String,
    pub length_ms: Option<u64>,
    pub release_group_id: Option<String>,
    pub isrcs: Vec<String>,
    pub provider_score: u8,
    pub title_match: bool,
    pub creator_match: bool,
}
impl Candidate {
    pub fn source_url(&self) -> String {
        provider(&self.provider).map_or_else(String::new, |p| p.source_url(self))
    }
    pub fn cover_url(&self) -> String {
        provider(&self.provider).map_or_else(String::new, |p| p.cover_url(self))
    }
    pub fn validate(&self) -> bool {
        provider(&self.provider).is_some_and(|p| p.valid_identity(self))
            && self.length_ms.is_none_or(|v| v <= 86_400_000)
            && !self.title.trim().is_empty()
            && !self.album.trim().is_empty()
            && [
                &self.title,
                &self.album,
                &self.date,
                &self.country,
                &self.disambiguation,
            ]
            .into_iter()
            .chain(self.artists.iter())
            .chain(self.isrcs.iter())
            .all(|s| s.chars().count() <= 240 && !s.chars().any(char::is_control))
            && self.artists.len() <= 16
            && self.isrcs.len() <= 32
            && self.provider_score <= 100
    }
}
pub trait MetadataProvider: Sync {
    fn id(&self) -> &'static str;
    fn valid_identity(&self, c: &Candidate) -> bool;
    fn source_url(&self, c: &Candidate) -> String;
    fn cover_url(&self, c: &Candidate) -> String;
    fn name(&self) -> &'static str;
    fn search_url(&self, query: &Query) -> Result<String, String>;
    fn candidates(&self, query: &Query, response: &Value) -> Result<Vec<Candidate>, String>;
}
pub struct MusicBrainz;
static MUSICBRAINZ: MusicBrainz = MusicBrainz;
static PROVIDERS: [&dyn MetadataProvider; 1] = [&MUSICBRAINZ];
pub fn providers() -> &'static [&'static dyn MetadataProvider] {
    &PROVIDERS
}
pub fn provider(id: &str) -> Option<&'static dyn MetadataProvider> {
    providers().iter().copied().find(|p| p.id() == id)
}
pub fn mbid(s: &str) -> bool {
    s.len() == 36
        && s.bytes().enumerate().all(|(i, c)| {
            if [8, 13, 18, 23].contains(&i) {
                c == b'-'
            } else {
                c.is_ascii_hexdigit()
            }
        })
}
fn normal(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .filter(|c| c.is_alphanumeric())
        .collect()
}
pub fn search_title(s: &str) -> String {
    let mut s = s.trim().to_string();
    for suffix in [".musicxml", ".midi", ".mxl", ".xml", ".mid"] {
        if s.to_lowercase().ends_with(suffix) {
            s.truncate(s.len() - suffix.len());
            break;
        }
    }
    s = s.replace('_', " ");
    for suffix in [
        " (piano)",
        " (piano solo)",
        " [piano]",
        " (easy)",
        " (arrangement)",
    ] {
        if s.to_lowercase().ends_with(suffix) {
            s.truncate(s.len() - suffix.len());
        }
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
fn quote(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn encode(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn string(v: &Value, key: &str) -> String {
    v[key]
        .as_str()
        .unwrap_or("")
        .chars()
        .filter(|c| !c.is_control())
        .take(240)
        .collect()
}
impl MetadataProvider for MusicBrainz {
    fn valid_identity(&self, c: &Candidate) -> bool {
        mbid(&c.recording_id)
            && mbid(&c.release_id)
            && c.release_group_id.as_ref().is_none_or(|s| mbid(s))
    }
    fn source_url(&self, c: &Candidate) -> String {
        format!("https://musicbrainz.org/release/{}", c.release_id)
    }
    fn cover_url(&self, c: &Candidate) -> String {
        format!(
            "https://coverartarchive.org/release/{}/front-250",
            c.release_id
        )
    }
    fn id(&self) -> &'static str {
        "musicbrainz"
    }
    fn name(&self) -> &'static str {
        "MusicBrainz + Cover Art Archive"
    }
    fn search_url(&self, q: &Query) -> Result<String, String> {
        let title = search_title(&q.title);
        if title.is_empty()
            || title.chars().count() > 160
            || q.creator.chars().count() > 160
            || q.title
                .chars()
                .chain(q.creator.chars())
                .any(char::is_control)
        {
            return Err("Enter a title of up to 160 characters.".into());
        }
        let query = if q.creator.trim().is_empty() {
            format!("recording:{}", quote(&title))
        } else {
            format!(
                "recording:{} AND artist:{}",
                quote(&title),
                quote(q.creator.trim())
            )
        };
        Ok(format!(
            "https://musicbrainz.org/ws/2/recording/?query={}&fmt=json&limit=12&client=Tangent-0.1.0",
            encode(&query)
        ))
    }
    fn candidates(&self, q: &Query, response: &Value) -> Result<Vec<Candidate>, String> {
        let recordings = response["recordings"]
            .as_array()
            .ok_or("Provider returned an unexpected response.")?;
        let mut result = vec![];
        let mut seen = BTreeSet::new();
        for r in recordings.iter().take(12) {
            let artists: Vec<_> = r["artist-credit"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|a| a["name"].as_str().or(a["artist"]["name"].as_str()))
                .take(16)
                .map(str::to_string)
                .collect();
            for release in r["releases"].as_array().into_iter().flatten().take(20) {
                let c = Candidate {
                    provider: self.id().into(),
                    recording_id: string(r, "id"),
                    release_id: string(release, "id"),
                    title: string(r, "title"),
                    artists: artists.clone(),
                    album: string(release, "title"),
                    date: string(release, "date"),
                    country: string(release, "country"),
                    disambiguation: string(r, "disambiguation"),
                    length_ms: r["length"].as_u64(),
                    release_group_id: release["release-group"]["id"].as_str().map(str::to_string),
                    isrcs: r["isrcs"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .take(32)
                        .map(str::to_string)
                        .collect(),
                    provider_score: r["score"]
                        .as_u64()
                        .or_else(|| r["score"].as_str().and_then(|s| s.parse().ok()))
                        .unwrap_or(0)
                        .min(100) as u8,
                    title_match: normal(&string(r, "title")) == normal(&search_title(&q.title)),
                    creator_match: !q.creator.trim().is_empty()
                        && artists
                            .iter()
                            .any(|a| normal(a).contains(&normal(&q.creator))),
                };
                if c.validate() && seen.insert(c.release_id.clone()) {
                    result.push(c);
                }
            }
        }
        result.sort_by_key(|c| {
            std::cmp::Reverse((
                c.title_match,
                c.creator_match,
                c.disambiguation.is_empty(),
                c.provider_score,
            ))
        });
        result.truncate(12);
        Ok(result)
    }
}
/// Only provider-built endpoints are fetchable; files and arbitrary URLs are not requests.
pub fn allowed_url(url: &str, cover: bool) -> bool {
    if cover {
        url.strip_prefix("https://coverartarchive.org/release/")
            .and_then(|s| s.strip_suffix("/front-250"))
            .is_some_and(mbid)
    } else {
        url.starts_with("https://musicbrainz.org/ws/2/recording/?query=")
            && url.len() < 4000
            && !url.chars().any(char::is_control)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe_query_and_filename_cleanup() {
        let q = Query {
            title: "Boadicea_(piano).musicxml".into(),
            creator: "Enya\" OR *:*".into(),
        };
        let url = MusicBrainz.search_url(&q).unwrap();
        assert!(url.contains("Boadicea"));
        assert!(url.contains("%5C%22"));
        assert!(!url.contains(' '));
        assert_eq!(search_title(&q.title), "Boadicea");
        assert!(
            MusicBrainz
                .search_url(&Query {
                    title: "".into(),
                    creator: "".into()
                })
                .is_err()
        );
    }
    #[test]
    fn destinations_are_provider_owned() {
        assert!(allowed_url(
            "https://coverartarchive.org/release/11111111-1111-1111-1111-111111111111/front-250",
            true
        ));
        for u in [
            "file:///tmp/a",
            "https://localhost/a",
            "https://coverartarchive.org.evil/a",
            "https://coverartarchive.org/release/../front-250",
        ] {
            assert!(!allowed_url(u, true));
        }
        assert_eq!(providers().len(), 1);
        assert!(provider("unknown").is_none());
    }
    #[test]
    fn candidates_keep_recording_artist_separate_from_local_composer() {
        let r = serde_json::json!({"recordings":[{"id":"11111111-1111-1111-1111-111111111111","title":"Boadicea","score":100,"artist-credit":[{"name":"Enya"}],"releases":[{"id":"22222222-2222-2222-2222-222222222222","title":"The Celts","date":"1992"},{"id":"bad","title":"bad"}]}]});
        let c = MusicBrainz
            .candidates(
                &Query {
                    title: "Boadicea".into(),
                    creator: "Enya".into(),
                },
                &r,
            )
            .unwrap();
        assert_eq!(c.len(), 1);
        assert!(c[0].title_match && c[0].creator_match);
        assert_eq!(c[0].album, "The Celts");
        assert!(c[0].validate());
        assert!(
            MusicBrainz
                .candidates(
                    &Query {
                        title: "x".into(),
                        creator: "".into()
                    },
                    &serde_json::json!({"html":"unavailable"})
                )
                .is_err()
        );
    }
}
