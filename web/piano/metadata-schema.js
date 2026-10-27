// Reviewed provider annotations are separate from local title/composer and source bytes.
export const metadataMigration=`CREATE TABLE library_enrichment (
 song_id TEXT PRIMARY KEY NOT NULL REFERENCES library_songs(id) ON DELETE CASCADE,
 payload TEXT NOT NULL, cover BLOB NOT NULL, search_key TEXT NOT NULL, updated_at TEXT NOT NULL);
 PRAGMA user_version=6;`;
const mbid=s=>typeof s==='string'&&/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);
export function validCandidate(c){return !!c&&c.provider==='musicbrainz'&&mbid(c.recording_id)&&mbid(c.release_id)&&(c.release_group_id==null||mbid(c.release_group_id))&&typeof c.title==='string'&&!!c.title&&typeof c.album==='string'&&!!c.album&&Array.isArray(c.artists)&&c.artists.length<=16&&Array.isArray(c.isrcs)&&c.isrcs.length<=32&&[c.title,c.album,c.date,c.country,c.disambiguation,...c.artists,...c.isrcs].every(s=>typeof s==='string'&&[...s].length<=240&&!/[\x00-\x1f\x7f-\x9f]/.test(s))&&Number.isInteger(c.provider_score)&&c.provider_score>=0&&c.provider_score<=100&&(c.length_ms==null||(Number.isSafeInteger(c.length_ms)&&c.length_ms>=0&&c.length_ms<=86400000))&&typeof c.title_match==='boolean'&&typeof c.creator_match==='boolean';}
export function writeEnrichment(db,id,candidate,cover){
 if(!validCandidate(candidate)||!(cover instanceof Uint8Array)||cover.length>1024*1024)throw Error('Invalid external metadata or cover.');
 if(cover.length&&!((cover[0]===255&&cover[1]===216)||(cover[0]===137&&cover[1]===80&&cover[2]===78&&cover[3]===71)))throw Error('Cover must be PNG or JPEG.');
 const q=db.prepare('SELECT COUNT(*) FROM library_songs WHERE id=?');let found;try{q.bind([id]);q.step();found=q.get()[0];}finally{q.free();}if(!found)throw Error('Piece is no longer in the library.');
 const usage=db.prepare('SELECT COALESCE(SUM(length(cover)),0) FROM library_enrichment WHERE song_id<>?');let size;try{usage.bind([id]);usage.step();size=usage.get()[0];}finally{usage.free();}if(size+cover.length>32*1024*1024)throw Error('Cover storage is full. Remove an old cover first.');
 db.run(`INSERT INTO library_enrichment(song_id,payload,cover,search_key,updated_at) VALUES(?,?,?,?,?) ON CONFLICT(song_id) DO UPDATE SET payload=excluded.payload,cover=excluded.cover,search_key=excluded.search_key,updated_at=excluded.updated_at`,[id,JSON.stringify(candidate),cover,[candidate.title,candidate.album,...candidate.artists].join('\n').toLowerCase(),new Date().toISOString()]);
}
export function readCover(db,id){const s=db.prepare('SELECT cover FROM library_enrichment WHERE song_id=?');try{s.bind([id]);return s.step()?new Uint8Array(s.get()[0]):new Uint8Array();}finally{s.free();}}
export function clearEnrichment(db,id){db.run('DELETE FROM library_enrichment WHERE song_id=?',[id]);}
