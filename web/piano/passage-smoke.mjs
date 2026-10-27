import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  window.sent=[];
  const output={id:'loop-piano',name:'Loop piano',state:'connected',async open(){return this},clear(){},send(bytes){window.sent.push([...bytes]);}};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>({inputs:new Map(),outputs:new Map([[output.id,output]]),onstatechange:null})});
 });
 await ready(page,server.url);console.log('Passage: booted');await click(page,'start-play');
 const events=[];
 for(const key of [60,62,64,65,67,69,71,72])events.push(0,0x90,key,100,0x87,0x40,0x80,key,0);
 events.push(0,0xff,0x2f,0);
 const track=Buffer.from(events),head=Buffer.from([77,84,104,100,0,0,0,6,0,0,0,1,1,224,77,84,114,107,0,0,0,track.length]);
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO',{delay:100});
 await(await chooser).setFiles({name:'Passage test.mid',mimeType:'audio/midi',buffer:Buffer.concat([head,track])});
 await page.waitForFunction(()=>tangentDiagnostics().notes===8);
 if(!(await page.evaluate(()=>tangentDiagnostics().help)))await click(page,'help');
 await click(page,'loop-a');await click(page,'forward');await click(page,'loop-b');
 console.log('Passage: range set');let d=await page.evaluate(()=>tangentDiagnostics());assert.deepEqual(d.loop,{start_us:0,end_us:5000000});assert.equal(d.position,0);
 for(let n=100;n>70;n-=5)await click(page,'speed-slower');
 await click(page,'settings');await click(page,'settings-audio');await click(page,'midi');
 await page.waitForFunction(()=>tangentDiagnostics().midi_outputs.length===1);
 await click(page,'midi-output');await click(page,'midi-output-choice');await click(page,'settings-back');
 await page.evaluate(()=>window.sent=[]);await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().progress_history.filter(r=>r.finished&&r.context?.passage.end_us===5000000).length>=3,null,{timeout:35000});
 d=await page.evaluate(()=>tangentDiagnostics());
 assert(d.playing);assert(d.position>=0&&d.position<5000000);
 console.log('Passage: three completed loops');const complete=d.progress_history.filter(r=>r.finished&&r.context?.passage.end_us===5000000);
 assert(complete.every(r=>r.song_total===5&&r.points===0&&r.song_missed===5&&r.context.speed_percent===70),'Autoplay cannot earn points; each iteration has selected denominator/context');
 const notes=await page.evaluate(()=>sent.filter(b=>(b[0]&240)===144&&b[2]>0).map(b=>b[1]));
 assert(notes.length>=15);assert(notes.every(key=>key<=67),'No attacks beyond loop B');
 await click(page,'help');await click(page,'speed-faster');await click(page,'speed-faster');await click(page,'restart');await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().practice_run?.context.speed_percent===80);
 await click(page,'help');await click(page,'practice-finish');await click(page,'progress');
 await page.waitForFunction(()=>tangentDiagnostics().progress_history.some(r=>r.finished));
 const row=await page.evaluate(()=>{const d=tangentDiagnostics(),i=d.progress_history.findIndex(r=>r.finished);return d.buttons.filter(b=>b.id==='progress-open')[i]});
 assert(row);await page.mouse.click(row.x+20,row.y+15);await click(page,'practice-section');
 d=await page.evaluate(()=>tangentDiagnostics());assert.equal(d.playing,false);assert.equal(d.help,true);assert.equal(d.preferences.speed_percent,70);assert.deepEqual(d.loop,{start_us:0,end_us:5000000});
 await page.screenshot({path:'../../docs/screenshots/passage-practice.png'});
 await click(page,'help'); // close pause without playback
 // Drag B to four seconds; pointer editing remains paused and updates core range.
 await page.mouse.move(28+(1280-56)*5/8,778);await page.mouse.down();await page.waitForTimeout(150);await page.mouse.move(28+(1280-56)*4/8,778,{steps:12});await page.waitForTimeout(150);await page.mouse.up();await page.waitForTimeout(200);
 d=await page.evaluate(()=>tangentDiagnostics());assert(Math.abs(d.loop.end_us-4000000)<30000,JSON.stringify({loop:d.loop,help:d.help,position:d.position}));assert.equal(d.playing,false);
 await click(page,'help');await click(page,'loop-clear');assert.equal((await page.evaluate(()=>tangentDiagnostics())).loop,null);
 await page.setViewportSize({width:390,height:720});
 await page.waitForFunction(()=>tangentDiagnostics().width===390&&tangentDiagnostics().height===720);
 d=await page.evaluate(()=>tangentDiagnostics());
 for(const b of d.buttons){assert(b.x>=0&&b.y>=0&&b.x+b.w<=390&&b.y+b.h<=720,`Button outside viewport: ${b.id}`);}
 await page.screenshot({path:'../../docs/screenshots/passage-practice-narrow.png'});
 assert.deepEqual(errors,[]);
 console.log('Passage Bevy browser: 3 loops at 70%, 80% segmentation, external MIDI boundary, no autoplay points, contextual retry, draggable B and narrow pause passed.');
} finally {await browser.close();await server.close();}
