import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
   const input={id:'explorer-in',name:'P-45 simulated',state:'connected',onmidimessage:null};
   window.fakeMidi={inputs:new Map([[input.id,input]]),outputs:new Map(),onstatechange:null};
   Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>window.fakeMidi});
   Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
 });
 await ready(page,server.url);
 await click(page,'settings');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('explorer-in').onmidimessage);
 await page.keyboard.press('Escape');
 await click(page,'train');await click(page,'train-explorer');
 await page.waitForFunction(()=>tangentDiagnostics().mode==='Explorer');
 let d=await page.evaluate(()=>tangentDiagnostics());
 assert.equal(d.explorer_source,48);assert.deepEqual(d.target,[60]);assert.deepEqual(d.expected_glow,[]);
 if(process.env.TANGENT_CAPTURE_EXPLORER==='1'){await page.waitForTimeout(300);await page.screenshot({path:new URL('../../docs/screenshots/keyboard-explorer.png',import.meta.url).pathname});}
 async function midi(key,down) {
   await page.evaluate(({key,down})=>fakeMidi.inputs.get('explorer-in').onmidimessage({data:[down?0x90:0x80,key,down?90:0]}),{key,down});
   await page.waitForFunction(({key,down})=>tangentDiagnostics().solid_keys.includes(key)===down,{key,down});
 }
 async function solve() {
   const before=await page.evaluate(()=>({key:tangentDiagnostics().target[0],count:tangentDiagnostics().practice_run.trials.length}));
   await midi(before.key,true);
   await page.waitForFunction(count=>tangentDiagnostics().practice_run.trials.length===count+1,before.count);
   assert.equal((await page.evaluate(()=>tangentDiagnostics())).completed,before.count);
   await midi(before.key,false);
   await page.waitForFunction(count=>tangentDiagnostics().screen==='Results'||tangentDiagnostics().completed===count+1,before.count);
 }
 // The marked source and a second, wrong octave are both wrong answers.
 for(const key of [48,72]){await midi(key,true);await page.waitForFunction(()=>tangentDiagnostics().practice_wrong);await midi(key,false);}
 await solve();
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).practice_run.points,40);
 await click(page,'practice-hint');await page.waitForFunction(()=>tangentDiagnostics().expected_glow.includes(62));
 await click(page,'practice-hint');await solve();
 await click(page,'practice-listen');await page.waitForFunction(()=>tangentDiagnostics().reference_keys.length>0);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).practice_run.trials.length,2,'Reference never scores');
 assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).solid_keys,[]);
 await click(page,'practice-listen');await solve();
 await click(page,'practice-skip');
 await page.waitForFunction(()=>tangentDiagnostics().practice_run.trials.length===4);
 for(let i=4;i<12;i++)await solve();
 await page.waitForFunction(()=>tangentDiagnostics().screen==='Results'&&tangentDiagnostics().progress_pending===0&&tangentDiagnostics().storage.pending===0);
 let result=await page.evaluate(()=>tangentDiagnostics().practice_result);
 assert.equal(result.mode,'keyboard-explorer');assert.equal(result.points,840);assert.equal(result.finished,true);
 assert.equal(result.trials[0].skill,'explorer:1:48:60');assert.equal(result.trials[1].assisted,true);assert.equal(result.trials[2].assisted,true);assert.equal(result.trials[3].skipped,true);
 await page.reload();await ready(page,server.url);await click(page,'progress');
 await page.waitForFunction(()=>tangentDiagnostics().progress_history.some(r=>r.mode==='keyboard-explorer'&&r.points===840));
 await click(page,'progress-open');await click(page,'practice-again');
 await page.waitForFunction(()=>tangentDiagnostics().mode==='Explorer'&&tangentDiagnostics().target[0]===60);
 await click(page,'practice-level');await page.waitForFunction(()=>tangentDiagnostics().preferences.practice_level===1);
 await click(page,'practice-level');await page.waitForFunction(()=>tangentDiagnostics().target[0]===72);
 // Reloaded input access is explicit. Reconnect through the pause settings.
 await page.keyboard.press('Escape');await click(page,'settings');await click(page,'settings-audio');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('explorer-in').onmidimessage);
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);
 await page.keyboard.press('Escape');
 await page.waitForFunction(()=>!tangentDiagnostics().help);
 await solve();assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).target,[48]);
 await page.setViewportSize({width:390,height:700});await page.waitForFunction(()=>tangentDiagnostics().width===390);
 if(process.env.TANGENT_CAPTURE_EXPLORER==='1')await page.screenshot({path:new URL('../../docs/screenshots/keyboard-explorer-mobile.png',import.meta.url).pathname});
 for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.x>=0&&b.x+b.w<=390,`${b.id} fits`);
 await page.setViewportSize({width:1280,height:420});await page.waitForFunction(()=>tangentDiagnostics().height===420);
 if(process.env.TANGENT_CAPTURE_EXPLORER==='1')await page.screenshot({path:new URL('../../docs/screenshots/keyboard-explorer-short.png',import.meta.url).pathname});
 await page.keyboard.press('Escape');await click(page,'main');await click(page,'train');
 for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.y+b.h<=420,`${b.id} fits short menu`);
 assert.deepEqual(errors,[]);
 console.log('Keyboard Explorer: exact octave/release, real MIDI events, assisted/skipped scoring, durable evidence/retry, levels, mobile and short-window UI passed.');
} finally {await browser.close();await server.close();}
