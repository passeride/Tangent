// Storage validation and atomic, idempotent checkpoints; scoring lives in Rust.
export const progressMigration = `
CREATE TABLE practice_details (
 session_id TEXT PRIMARY KEY NOT NULL REFERENCES practice_sessions(id) ON DELETE CASCADE,
 revision INTEGER NOT NULL, payload TEXT NOT NULL, updated_at INTEGER NOT NULL
);
CREATE INDEX practice_recent ON practice_details(updated_at DESC);
PRAGMA user_version = 5;`;
function valid(r) {
 const int=(n,max)=>Number.isSafeInteger(n)&&n>=0&&n<=max;
 return r && (r.version??1)===1 && /^[a-zA-Z0-9-]{8,100}$/.test(r.id) && ['notes','chords','play'].includes(r.mode)
 && typeof r.label==='string'&&r.label.length<=100&&typeof r.finished==='boolean'
 && int(r.revision,4294967295)&&int(r.errors,100000)&&int(r.elapsed_ms,604800000)
 && Array.isArray(r.trials)&&r.trials.length<=12 && r.trials.every(t=>typeof t.skill==='string'&&t.skill.length<=100
 && ['clean','assisted','skipped'].every(k=>typeof t[k]==='boolean')&&int(t.response_ms,604800000))
 && int(r.song_total??0,1000000)&&int(r.song_correct??0,r.song_total??0)&&int(r.song_missed??0,r.song_total??0)
 && (!r.finished||(r.mode==='play'?(r.song_correct??0)+(r.song_missed??0)===(r.song_total??0):r.trials.length===12))
 && r.points===(r.song_correct??0)*100+r.trials.reduce((n,t)=>n+(t.skipped||t.assisted?0:t.clean?100:40),0);
}
export function writeProgress(db,r) {
 if(!valid(r))throw Error('Invalid practice checkpoint');
 db.run('SAVEPOINT practice_save');
 try {
  const old=db.exec('SELECT revision FROM practice_details WHERE session_id=?',[r.id]);
  if(old.length&&old[0].values[0][0]>r.revision){db.run('RELEASE practice_save');return;}
  const at=new Date().toISOString();
  db.run('INSERT INTO practice_sessions(id,mode,started_at,ended_at) VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET ended_at=excluded.ended_at',[r.id,r.mode,at,r.finished?at:null]);
  db.run('INSERT INTO practice_details VALUES(?,?,?,?) ON CONFLICT(session_id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload,updated_at=excluded.updated_at',[r.id,r.revision,JSON.stringify(r),Date.now()]);
  db.run('DELETE FROM scores WHERE session_id=?',[r.id]);
  for(const [metric,value] of Object.entries({points:r.points,solved:r.mode==='play'?r.song_correct:r.trials.filter(t=>!t.skipped).length,clean:r.mode==='play'?r.song_correct:r.trials.filter(t=>t.clean&&!t.assisted&&!t.skipped).length,errors:r.errors,elapsed_ms:r.elapsed_ms}))
   db.run('INSERT INTO scores VALUES(?,?,?,?,?)',[`${r.id}:${metric}`,r.id,metric,value,at]);
  db.run('RELEASE practice_save');
 }catch(e){db.run('ROLLBACK TO practice_save');db.run('RELEASE practice_save');throw e;}
}
export function readProgress(db) {
 return db.exec('SELECT payload FROM practice_details ORDER BY updated_at DESC,session_id DESC LIMIT 200')[0]?.values.map(r=>JSON.parse(r[0]))??[];
}
