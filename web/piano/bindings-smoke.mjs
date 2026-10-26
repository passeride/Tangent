import assert from 'node:assert/strict';
import {mkdir} from 'node:fs/promises';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  const input={id:'bindings-in',name:'P-45',state:'connected',onmidimessage:null};
  window.fakeMidi={inputs:new Map([[input.id,input]]),outputs:new Map(),onstatechange:null};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>fakeMidi});
  Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
 });
 await ready(page,server.url);
 await click(page,'settings');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('bindings-in').onmidimessage);
 await click(page,'settings-accessibility');await click(page,'keybindings');
 async function nth(id,index) {
  const b=await page.evaluate(({id,index})=>tangentDiagnostics().buttons.filter(b=>b.id===id)[index],{id,index});
  assert(b,`Missing ${id} ${index}`);await page.mouse.click(b.x+b.w/2,b.y+b.h/2);
  const f=await page.evaluate(()=>tangentDiagnostics().frame);await page.waitForFunction(f=>tangentDiagnostics().frame>=f+2,f);
 }
 async function cell(action,piano=false) {
  while(true){
   const d=await page.evaluate(()=>tangentDiagnostics());
   const index=d.preferences.bindings.rows.findIndex(r=>r.action===action),count=d.buttons.filter(b=>b.id==='binding-keyboard').length;
   const target=Math.floor(index/count);
   if(d.bindings_page!==target){await click(page,d.bindings_page<target?'binding-next':'binding-prev');continue;}
   // Piano cells omit direct live-note rows; these tests use command rows only.
   await nth(piano?'binding-piano':'binding-keyboard',index%count);return;
  }
 }
 async function midi(key,down=true){await page.evaluate(({key,down})=>fakeMidi.inputs.get('bindings-in').onmidimessage({data:[down?0x90:0x80,key,down?90:0]}),{key,down});}
 async function tap(key){await midi(key);await page.waitForTimeout(80);await midi(key,false);await page.waitForTimeout(80);}
 await cell('play');await page.keyboard.press('KeyZ');
 await page.waitForFunction(()=>tangentDiagnostics().preferences.bindings.rows.find(r=>r.action==='play').keyboard.key==='KeyZ');
 await cell('faster');await page.keyboard.press('KeyZ');
 await page.waitForFunction(()=>tangentDiagnostics().bindings_status.includes('Already assigned'));
 assert.equal(await page.evaluate(()=>tangentDiagnostics().preferences.bindings.rows.find(r=>r.action==='faster').keyboard.key),'ArrowUp');
 await page.keyboard.press('Escape');
 await cell('faster',true);await tap(60);
 await cell('play',true);await tap(72);
 await cell('skip',true);await tap(74);
 await nth('binding-control',0);await tap(23);
 await page.waitForFunction(()=>tangentDiagnostics().preferences.bindings.control===23);
 await click(page,'binding-toggle');
 while(await page.evaluate(()=>tangentDiagnostics().bindings_page>0))await click(page,'binding-prev');
 await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
 await mkdir(new URL('../../dist/verification/',import.meta.url),{recursive:true});
 await page.screenshot({path:new URL('../../dist/verification/keybindings.png',import.meta.url).pathname});
 // Layout bounds on a low window and a phone; no overlapping control cells.
 for(const viewport of [{width:640,height:360},{width:390,height:844}]) {
  await page.setViewportSize(viewport);await page.waitForFunction(w=>tangentDiagnostics().width===w,viewport.width);
  const buttons=await page.evaluate(()=>tangentDiagnostics().buttons);
  for(const b of buttons)assert(b.x>=0&&b.y>=0&&b.x+b.w<=viewport.width+.1&&b.y+b.h<=viewport.height+.1,`Out of bounds ${b.id}`);
  for(let i=0;i<buttons.length;i++)for(let j=i+1;j<buttons.length;j++){
   const a=buttons[i],b=buttons[j];assert(!(Math.min(a.x+a.w,b.x+b.w)>Math.max(a.x,b.x)&&Math.min(a.y+a.h,b.y+b.h)>Math.max(a.y,b.y)),`${a.id} overlaps ${b.id}`);
  }
 }
 await page.setViewportSize({width:1280,height:800});
 await click(page,'settings-back');await click(page,'settings-back');await click(page,'start-play');
 await page.keyboard.press('KeyZ');await page.waitForFunction(()=>tangentDiagnostics().playing);
 await page.keyboard.press('KeyZ');await page.waitForFunction(()=>!tangentDiagnostics().playing);
 // Same poll, opposite message order: simultaneous keys still form a command.
 await page.evaluate(()=>{const p=fakeMidi.inputs.get('bindings-in');p.onmidimessage({data:[0x90,60,90]});p.onmidimessage({data:[0x90,23,90]});});
 await page.waitForFunction(()=>tangentDiagnostics().preferences.speed_percent===105);
 await midi(60,false);await midi(23,false);
 await page.waitForFunction(()=>tangentDiagnostics().solid_keys.length===0);
 await tap(23);await tap(60);
 await page.waitForFunction(()=>tangentDiagnostics().preferences.speed_percent===110);
 await midi(60);await page.waitForFunction(()=>tangentDiagnostics().solid_keys.includes(60));await midi(60,false);
 await page.waitForFunction(()=>tangentDiagnostics().solid_keys.length===0);
 await page.keyboard.press('Home');await page.waitForFunction(()=>tangentDiagnostics().position===0);
 await page.keyboard.press('KeyZ');await page.waitForFunction(()=>tangentDiagnostics().playing && tangentDiagnostics().practice_run?.mode==='play');
 const runId=await page.evaluate(()=>tangentDiagnostics().practice_run.id);
 await tap(23);await tap(74);
 assert.equal(await page.evaluate(()=>tangentDiagnostics().practice_run?.id),runId,'Exercise shortcut preserves the song attempt');
 assert.equal(await page.evaluate(()=>tangentDiagnostics().practice_run?.trials.length??0),0,'Exercise commands cannot mutate a song attempt');
 assert.equal(await page.evaluate(()=>tangentDiagnostics().practice_run?.song_correct??0),0);
 await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
 await page.reload();await page.waitForFunction(()=>window.tangentDiagnostics?.()?.storage.ready);
 await page.waitForFunction(()=>fakeMidi.inputs.get('bindings-in').onmidimessage);
 const cfg=await page.evaluate(()=>tangentDiagnostics().preferences.bindings);
 assert.equal(cfg.control,23);assert(cfg.piano_enabled);
 assert.equal(cfg.rows.find(r=>r.action==='play').keyboard.key,'KeyZ');
 assert.equal(cfg.rows.find(r=>r.action==='faster').piano,60);
 assert.equal(cfg.rows.find(r=>r.action==='play').piano,72);
 assert.deepEqual(errors,[]);
 console.log('Keybindings: capture, conflict rejection, MIDI modifier/chord/sequence isolation, PC remap, compact matrix and persistent reload passed.');
} finally {await browser.close();await server.close();}
