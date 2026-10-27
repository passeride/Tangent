import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs});
const xml=`<?xml version="1.0"?><score-partwise version="3.1"><work><work-title>Sequential fixture</work-title></work><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><direction><sound tempo="120"/></direction><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>E</step><octave>4</octave></pitch><duration>1</duration><type>quarter</type></note><note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><type>quarter</type></note></measure></part></score-partwise>`;
try{
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  window.sent=[];const input={id:'guide-in',name:'P-45 simulated',state:'connected',onmidimessage:null};
  const output={id:'guide-out',name:'Guide piano',state:'connected',async open(){return this},clear(){},send(b){sent.push([...b]);}};
  window.fakeMidi={inputs:new Map([[input.id,input]]),outputs:new Map([[output.id,output]]),onstatechange:null};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>fakeMidi});
  Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
 });
 await ready(page,server.url);await click(page,'start-play');
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO',{delay:100});
 await(await chooser).setFiles({name:'Sequential.musicxml',mimeType:'application/xml',buffer:Buffer.from(xml)});
 await page.waitForFunction(()=>tangentDiagnostics().notes===5);
 if(!await page.evaluate(()=>tangentDiagnostics().help))await click(page,'help');
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.sequential,false);
 await click(page,'sequential');await page.waitForFunction(()=>tangentDiagnostics().preferences.sequential);
 await click(page,'settings');await click(page,'settings-audio');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('guide-in').onmidimessage);
 await click(page,'midi-output');await click(page,'midi-output-choice');
 await page.waitForFunction(()=>tangentDiagnostics().midi_output==='guide-out');
 await click(page,'settings-back');await page.evaluate(()=>sent.length=0);await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().guided_waiting&&tangentDiagnostics().position===0);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).playing,true,'Waiting is distinct from manual pause');
 await page.waitForTimeout(250);assert.equal((await page.evaluate(()=>tangentDiagnostics())).position,0);
 assert.deepEqual(await page.evaluate(()=>sent.filter(b=>(b[0]&240)===144&&b[2]>0)),[],'Backing cannot play the blocked first note');
 async function midi(key,down){
  await page.evaluate(({key,down})=>fakeMidi.inputs.get('guide-in').onmidimessage({data:[down?0x90:0x80,key,down?90:0]}),{key,down});
  await page.waitForFunction(({key,down})=>tangentDiagnostics().solid_keys.includes(key)===down,{key,down});
 }
 async function gate(at){await page.waitForFunction(at=>tangentDiagnostics().guided_waiting&&tangentDiagnostics().position===at,at);}
 await midi(72,true);await midi(72,false);assert.equal((await page.evaluate(()=>tangentDiagnostics())).position,0);
 await midi(60,true); // Keep this physically held through the later repeated C.
 await gate(500000);await page.waitForTimeout(300);assert.equal((await page.evaluate(()=>tangentDiagnostics())).position,500000);
 assert.deepEqual(await page.evaluate(()=>sent.filter(b=>b[0]===0x90&&b[2]>0).map(b=>b[1])),[60],'No future D attack and no ended-C reattack');
 await page.keyboard.press('Space');await page.waitForFunction(()=>!tangentDiagnostics().playing);
 await midi(62,true);await midi(62,false);assert.equal((await page.evaluate(()=>tangentDiagnostics())).guided_gate,500000,'Manual pause never accepts answers');
 await page.keyboard.press('Space');await page.waitForFunction(()=>tangentDiagnostics().playing&&tangentDiagnostics().guided_waiting);
 await midi(62,true);await midi(62,false);await gate(1000000);
 await page.keyboard.down('KeyD');await page.waitForFunction(()=>tangentDiagnostics().solid_keys.includes(64));
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).guided_gate,1000000,'Chord remains blocked after one note');
 assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).expected_glow,[67]);
 const cut=await page.evaluate(()=>sent.length);
 await page.keyboard.down('KeyG');await page.waitForFunction(()=>tangentDiagnostics().solid_keys.includes(67));
 await gate(1500000);
 assert(!(await page.evaluate(cut=>sent.slice(cut).some(b=>b[0]===0x8f&&b[1]===64||b[0]===0xbf&&[120,123].includes(b[1])),cut)),'Guided resets preserve held computer-key MIDI voices');
 await midi(60,true);await page.waitForTimeout(150);assert.equal((await page.evaluate(()=>tangentDiagnostics())).position,1500000,'Held/repeated note-on cannot satisfy a new onset');
 if(process.env.TANGENT_CAPTURE_GUIDED==='1')await page.screenshot({path:new URL('../../docs/screenshots/sequential-play.png',import.meta.url).pathname});
 await midi(60,false);await midi(60,true);
 await page.keyboard.up('KeyD');await page.keyboard.up('KeyG');await midi(60,false);
 await page.waitForFunction(()=>tangentDiagnostics().screen==='Results'&&tangentDiagnostics().progress_pending===0);
 const result=await page.evaluate(()=>tangentDiagnostics().practice_result);
 assert.equal(result.points,500);assert.equal(result.song_correct,5);assert.equal(result.errors,1);
 assert.deepEqual(result.context.guided,{early_ms:250});assert.equal(result.matched_targets.length,5);
 await click(page,'practice-again');
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).playing,false);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.sequential,true);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).guided_gate,0,'Retry clears credited gates, including at the same timestamp');
 await click(page,'settings');await click(page,'settings-practice');await click(page,'sequential-window');
 await page.waitForFunction(()=>tangentDiagnostics().preferences.sequential_early_ms===500&&tangentDiagnostics().storage.pending===0);
 for(const [width,height,name] of [[390,844,'mobile'],[1280,420,'short']]){
  await page.setViewportSize({width,height});await page.waitForFunction(({width,height})=>tangentDiagnostics().width===width&&tangentDiagnostics().height===height,{width,height});
  for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.x>=0&&b.x+b.w<=width&&b.y+b.h<=height,`${b.id} fits ${name}`);
  if(process.env.TANGENT_CAPTURE_GUIDED==='1')await page.screenshot({path:new URL(`../../docs/screenshots/sequential-settings-${name}.png`,import.meta.url).pathname});
 }
 await page.reload();await ready(page,server.url);
 await page.waitForFunction(()=>tangentDiagnostics().storage.ready);
 const p=await page.evaluate(()=>tangentDiagnostics().preferences);assert.equal(p.sequential,true);assert.equal(p.sequential_early_ms,500);
 await click(page,'progress');await page.waitForFunction(()=>tangentDiagnostics().progress_history.some(r=>r.context?.guided?.early_ms===250&&r.points===500));
 assert.deepEqual(errors,[]);
 console.log('Sequential Play: exact gates/chords/rearticulation, clock/output freeze, live MIDI preservation, scored evidence/retry, stored tolerance and responsive settings passed.');
}finally{await browser.close();await server.close();}
