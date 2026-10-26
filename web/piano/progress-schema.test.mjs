import assert from 'node:assert/strict';
import initSqlJs from 'sql.js';
import {readFileSync} from 'node:fs';
import {migrate,schemaVersion} from './storage-schema.js';
import {writeProgress,readProgress} from './progress-schema.js';
const SQL=await initSqlJs();const db=new SQL.Database();migrate(db);
assert.equal(schemaVersion,5);
const run={id:'test-session',revision:2,mode:'notes',label:'Treble / Stepwise',points:100,errors:0,elapsed_ms:1200,finished:false,trials:[{skill:'note:0:0:60',clean:true,assisted:false,skipped:false,response_ms:1200}]};
writeProgress(db,run);writeProgress(db,run);
assert.equal(readProgress(db).length,1);
writeProgress(db,{...run,revision:1,points:0,trials:[]});
assert.equal(readProgress(db)[0].points,100);
assert.equal(db.exec('SELECT count(*) FROM scores')[0].values[0][0],5);
assert.throws(()=>writeProgress(db,{...run,points:200}));
assert.equal(readProgress(db)[0].points,100);
const reopened=new SQL.Database(db.export());migrate(reopened);assert.equal(readProgress(reopened)[0].points,100);
assert.deepEqual(reopened.exec('PRAGMA foreign_key_check'),[]);
const old=new SQL.Database();migrate(old);old.run('DROP TABLE practice_details;PRAGMA user_version=4;');migrate(old);assert.deepEqual(readProgress(old),[]);
console.log('Practice SQLite: v4 migration, idempotent checkpoints, stale-write rejection, validation and export/reopen passed.');

// Compare the actual native migration with the browser schema, not a test copy.
const nativeSource=readFileSync(new URL('../../crates/tangent-player/src/scene/platform/native/progress.rs',import.meta.url),'utf8');
const nativeMigration=nativeSource.match(/pub const MIGRATION: &str = "([\s\S]*?)";/)[1];
const native=new SQL.Database();native.run('PRAGMA foreign_keys=ON; CREATE TABLE practice_sessions(id TEXT PRIMARY KEY);');native.run(nativeMigration);
for(const pragma of ['table_info(practice_details)','foreign_key_list(practice_details)','index_list(practice_details)'])assert.deepEqual(native.exec('PRAGMA '+pragma),db.exec('PRAGMA '+pragma));
console.log('Native/web progress schema parity passed.');

const context={passage:{start_us:1000000,end_us:11000000},duration_us:30000000,speed_percent:70,part:0,evaluator:1,song_audio:false};
const song={...run,id:'song-context-test',mode:'play',trials:[],song_total:3,song_correct:1,song_missed:2,finished:true,context,matched_targets:[[1000000,60]],piece_id:'a'.repeat(64)};
writeProgress(db,song);
const contextDb=new SQL.Database(db.export());migrate(contextDb);
assert.deepEqual(readProgress(contextDb).find(r=>r.id===song.id).context,context);
assert.deepEqual(readProgress(contextDb).find(r=>r.id===song.id).matched_targets,[[1000000,60]]);
for(const bad of [{...context,speed_percent:0},{...context,passage:{start_us:3,end_us:2}},{...context,evaluator:2},{...context,song_audio:null}])assert.throws(()=>writeProgress(db,{...song,context:bad}));
assert.throws(()=>writeProgress(db,{...song,matched_targets:[[11000000,60]]}));
assert.throws(()=>writeProgress(db,{...song,song_correct:2,song_missed:1,points:200,matched_targets:[[1000000,60],[1000000,60]]}));
assert.equal(readProgress(db).find(r=>r.id==='test-session').context,undefined,'Legacy context stays unknown');
console.log('Passage context: legacy absence, exact context/evidence round trip and malformed checkpoints passed.');

const explorer={...run,id:'keyboard-explorer-session',mode:'keyboard-explorer',label:'Keyboard Explorer / C / D / E',trials:[{...run.trials[0],skill:'explorer:1:48:60'}]};
writeProgress(db,explorer);
const explorerDb=new SQL.Database(db.export());migrate(explorerDb);
assert.equal(readProgress(explorerDb).find(r=>r.id===explorer.id).trials[0].skill,'explorer:1:48:60');
assert.throws(()=>writeProgress(db,{...explorer,trials:[{...explorer.trials[0],skill:'explorer:1:48:72'}]}));
assert.throws(()=>writeProgress(db,{...explorer,trials:[{...explorer.trials[0],skill:'note:0:0:60'}]}));
console.log('Keyboard Explorer checkpoint: exact octave evidence, export/reopen and invalid task rejection passed.');
