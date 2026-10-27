# Library artwork and metadata providers

- Owner request: a metadata-provider interface, album covers first, other useful metadata too
- Date: 2026-10-27
- Delivery: first MusicBrainz/Cover Art Archive adapter; live service verification limited by this environment
- Architecture: [ADR-0034](../adr/0034-metadata-providers-and-local-artwork.md)

## Provider choice

| Service | Fit for Tangent | Decision |
| --- | --- | --- |
| MusicBrainz + Cover Art Archive | Title/artist search, recording/release IDs and credits, album/date/country, release artwork; read access without a user API key | First provider; respect MusicBrainz's one-request-per-second limit |
| TheAudioDB | Artwork and richer descriptions; free v1 key documented, v2/supporter plan and distribution conditions | Potential later adapter, with its own configuration/terms review |
| Apple iTunes Search | Useful music/artwork search, but artwork is promotional content tied to store promotion/badges | Excluded as the default library-art source |

Primary references checked on this date:
- https://musicbrainz.org/doc/MusicBrainz_API
- https://musicbrainz.org/doc/MusicBrainz_API/Search
- https://musicbrainz.org/doc/MusicBrainz_API/Rate_Limiting
- https://musicbrainz.org/doc/Cover_Art_Archive/API
- https://musicbrainz.org/doc/Cover_Art_Archive
- https://musicbrainz.org/doc/About/Data_License
- https://www.theaudiodb.com/free_music_api
- https://www.theaudiodb.com/docs_terms_of_use.php
- https://performance-partners.apple.com/search-api

## Review flow

Library → Edit → **Find cover** searches using the displayed title and composer as
an artist hint. The provider calls it an artist search; this is not proof that a
composer performed a recording. **Title only** is useful for classical works or
arrangements where the performer differs. The title is conservatively cleaned of
file extensions, underscores and common piano/easy suffixes only for querying.
The local text is preserved. No file bytes, notes, performance history or recordings
are sent to metadata providers, and imports never start remote searches themselves.

A work can have many recordings and editions. Show album, artist, track and date
candidates; **Preview** loads a release's front-cover thumbnail. **Use cover &
metadata** saves only the reviewed annotation and image. A missing cover leaves
**Use metadata only** available. No candidate is silently applied, even on Enter.
Return to results or cancel while a request is pending; late results are ignored.

Keep the original title/composer and source hash/file bytes. Artist credits are
separate annotations, not imported as a composer. Saved title/artist/album metadata
is searchable alongside local title/composer. Reimport preserves annotations.
The main library shows thumbnails, with an original music-note fallback when none
is saved; metadata can be removed independently of the source and practice history.

## Layout and storage

Use the single Bevy canvas. Fit one to four candidate rows according to height,
with a separate larger preview. Narrow and short screens retain search, preview,
save and back controls. Cover placeholders use existing theme/music glyphs.
Downloaded covers are local SQLite BLOBs, included in database export and usable
offline. There is no remote-image hotlink on library open. Provider/recording/
release identifiers and the saved-at timestamp preserve the association's origin.

Limits: 1 MiB/download, PNG/JPEG only, 1024×1024 decoded dimensions, 8 MiB decode
allocation budget, 32 MiB total saved covers. Search/thumbnail responses use a
bounded ten-minute memory cache, timeouts, and serialized requests. Native HTTP
has its own bounded worker, separate from audio and SQLite command handling.

MusicBrainz core metadata is CC0; Cover Art Archive access does not make every
album image CC0. The app identifies the provider and release, does not bundle real
album art, and does not imply an arrangement has a definitive original album.
No auto-identification from audio or the melodic fingerprint engine is claimed.

## Verification and screenshots

`node web/piano/metadata-schema.test.mjs`, `metadata-http.test.mjs` and
`metadata-smoke.mjs` cover the adapter boundary and review workflow. Reproduce
real Bevy captures with `TANGENT_CAPTURE_METADATA=1 node web/piano/capture.mjs`:

- [Library with a saved cover](../screenshots/library-covers.png)
- [Release review](../screenshots/metadata-review.png)

The album/artist response and gradient image in tests/screenshots are original
synthetic fixtures; provider HTTP is intercepted. They prove the app integration,
not live catalogue coverage. Direct live MusicBrainz requests from this execution
environment returned a network access page, so end-user live search/CORS and
catalogue matching remain explicitly unverified here. Errors are shown without
silently substituting another provider or an incorrect cover.
