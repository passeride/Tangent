import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
 await ready(page,server.url);console.log('Goal: booted');await click(page,'start-play');await click(page,'help');
 await click(page,'loop-snap');
 let d=await page.evaluate(()=>tangentDiagnostics());assert.deepEqual(d.loop,{start_us:d.measure_ranges[0].start_us,end_us:d.measure_ranges[0].end_us});
 console.log('Goal: snapped');assert(d.measure_ranges.some(v=>v.pass>1),'Study actually contains repeat visits');
 await page.screenshot({path:'../../docs/screenshots/measure-practice.png'});
 const track=Buffer.from([0,0x90,60,100,0x87,0x40,0x80,60,0,0,0xff,0x2f,0]);
 const head=Buffer.from([77,84,104,100,0,0,0,6,0,0,0,1,1,224,77,84,114,107,0,0,0,track.length]);
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO');await(await chooser).setFiles({name:'One-note goal.mid',mimeType:'audio/midi',buffer:Buffer.concat([head,track])});
 await page.waitForFunction(()=>tangentDiagnostics().notes===1);console.log('Goal: imported from pause');
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).goal,null,'Source change clears session goal');
 await click(page,'loop-a');await click(page,'forward');await click(page,'loop-b');await click(page,'repetition-goal');
 d=await page.evaluate(()=>tangentDiagnostics());assert.deepEqual(d.goal,{completed:0,required:3,reached:false});
 assert(!d.buttons.some(b=>b.id==='loop-snap'),'MIDI has no invented bar snap');
 console.log('Goal: enabled');await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().progress_history.some(r=>r.finished&&r.song_total===1));
 console.log('Goal: autoplay excluded');assert.equal((await page.evaluate(()=>tangentDiagnostics())).goal.completed,0,'Autoplay earns no goal progress');
 for(let i=1;i<=3;i++) {
  if(i===1)await page.evaluate(()=>{const c=document.querySelector('#tangent');for(const type of ['keydown','keyup'])c.dispatchEvent(new KeyboardEvent(type,{key:'a',code:'KeyA',bubbles:true}));});
  else await page.keyboard.down('KeyA');
  await page.waitForFunction(()=>tangentDiagnostics().practice_run?.song_correct===1);
  if(i!==1)await page.keyboard.up('KeyA');
  await page.waitForFunction(()=>tangentDiagnostics().solid_keys.length===0);
  await page.waitForFunction(i=>tangentDiagnostics().goal.completed===i,i,{timeout:10000});console.log('Goal: pass',i);
 }
 d=await page.evaluate(()=>tangentDiagnostics());assert.equal(d.goal.reached,true);assert(d.hud_labels.some(s=>s.includes('Goal reached')));
 await click(page,'help');await page.screenshot({path:'../../docs/screenshots/repetition-goal.png'});
 await click(page,'speed-faster');await page.waitForFunction(()=>tangentDiagnostics().goal.completed===0&&!tangentDiagnostics().goal.reached);
 await click(page,'loop-clear');assert.equal((await page.evaluate(()=>tangentDiagnostics())).goal,null);
 await page.setViewportSize({width:640,height:360});await page.waitForFunction(()=>tangentDiagnostics().width===640&&tangentDiagnostics().height===360);await click(page,'settings');await click(page,'settings-practice');
 d=await page.evaluate(()=>tangentDiagnostics());for(const b of d.buttons)assert(b.x>=0&&b.y>=0&&b.x+b.w<=640&&b.y+b.h<=360,`Clipped ${b.id}`);
 assert.deepEqual(errors,[]);
 console.log('Goal browser: real MusicXML measure visits, MIDI fallback, optional goal, autoplay rejection, three actual-input passes, tempo reset and short-window controls passed.');
}finally{await browser.close();await server.close();}
