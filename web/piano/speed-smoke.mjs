import assert from 'node:assert/strict';
import {mkdir} from 'node:fs/promises';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}), errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  window.midiSent=[];
  const port={id:'tempo-out',name:'Tempo piano',state:'connected',async open(){return this},clear(){},send(bytes){window.midiSent.push({bytes:[...bytes],at:(window.sourceClock?.() ?? window.tangentPlatform?.clock_us?.() ?? 0)/1000,wall:performance.now()})}};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>({inputs:new Map(),outputs:new Map([[port.id,port]]),onstatechange:null})});
 });
 async function boot(){
  await ready(page,server.url);
  await page.evaluate(()=>{
   window.sourceClock=tangentPlatform.clock_us;
   tangentPlatform.clock_us=()=>window.lastSongClock=sourceClock();
  });
 }
 await boot();await click(page,'start-play');await click(page,'help');
 async function rate(percent) {
  while(true){const current=await page.evaluate(()=>tangentDiagnostics().preferences.speed_percent);if(current===percent)break;await click(page,current>percent?'speed-slower':'speed-faster');}
 }
 await rate(60);assert.equal((await page.evaluate(()=>tangentDiagnostics())).playing,false);
 await page.setViewportSize({width:390,height:844});
 await page.waitForFunction(()=>tangentDiagnostics().width===390);
 const buttons=await page.evaluate(()=>tangentDiagnostics().buttons);
 for(const b of buttons)assert(b.x>=0&&b.x+b.w<=390&&b.y>=0&&b.y+b.h<=844);
 for(let i=0;i<buttons.length;i++)for(let j=i+1;j<buttons.length;j++){
  const a=buttons[i],b=buttons[j];assert(!(Math.min(a.x+a.w,b.x+b.w)>Math.max(a.x,b.x)&&Math.min(a.y+a.h,b.y+b.h)>Math.max(a.y,b.y)),`${a.id} overlaps ${b.id}`);
 }
 await mkdir(new URL('../../dist/verification/',import.meta.url),{recursive:true});
 await page.screenshot({path:new URL('../../dist/verification/pause-speed-mobile.png',import.meta.url).pathname});
 await page.setViewportSize({width:1280,height:800});
 await click(page,'play');
 async function slope(expected){
  const read=()=>page.evaluate(()=>({position:tangentDiagnostics().position,now:window.lastSongClock}));
  const a=await read();await page.waitForTimeout(800);const b=await read();
  const actual=(b.position-a.position)/((b.now-a.now));
  assert(Math.abs(actual-expected)<0.13,`Rate ${expected}, measured ${actual}`);
 }
 await slope(.6);await page.waitForFunction(()=>tangentDiagnostics().energy>0.0001);
 await click(page,'help');const pos=await page.evaluate(()=>tangentDiagnostics().position);
 await page.waitForTimeout(200);assert.equal(await page.evaluate(()=>tangentDiagnostics().position),pos);
 await rate(70);await click(page,'play');await slope(.7);
 await click(page,'help');await rate(120);await click(page,'play');await slope(1.2);
 await click(page,'help');await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
 await page.reload();await boot();
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.speed_percent,120);
 await click(page,'start-play');await click(page,'help');await click(page,'speed-reset');
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.speed_percent,100);
 await click(page,'settings');await click(page,'midi');await click(page,'midi-output');await click(page,'midi-output-choice');
 await page.waitForFunction(()=>tangentDiagnostics().midi_output==='tempo-out');
 await click(page,'settings-back');await rate(60);await click(page,'play');
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO');
 const track=Buffer.from([0,0x90,60,100,0x9e,0,0x80,60,0,0,0x90,62,100,0x9e,0,0x80,62,0,0,0xff,0x2f,0]);
 const header=Buffer.from([77,84,104,100,0,0,0,6,0,0,0,1,1,224,77,84,114,107,0,0,0,track.length]);
 await(await chooser).setFiles({name:'Tempo check.mid',mimeType:'audio/midi',buffer:Buffer.concat([header,track])});
 await page.waitForFunction(()=>tangentDiagnostics().notes===2);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.speed_percent,60,'Import preserves speed');
 await page.evaluate(()=>window.midiSent=[]);await click(page,'play');
 await page.waitForFunction(()=>midiSent.some(e=>e.bytes[0]===0x90&&e.bytes[1]===62));
 const sent=await page.evaluate(()=>midiSent);
 const on=key=>sent.find(e=>e.bytes[0]===0x90&&e.bytes[1]===key);
 assert(Math.abs((on(62).at-on(60).at)/1000-4/.6)<.35,`Physical MIDI event spacing ${(on(62).at-on(60).at)/1000}s; events ${JSON.stringify(sent.filter(e=>(e.bytes[0]&240)===144||(e.bytes[0]&240)===128))}`);
 const off=sent.find(e=>e.bytes[0]===0x80&&e.bytes[1]===60);
 assert(off&&Math.abs(off.at-on(62).at)<100,'Note-off and next attack remain aligned');
 await click(page,'help');const stopped=await page.evaluate(()=>midiSent.length);await page.waitForTimeout(200);
 assert.equal(await page.evaluate(()=>midiSent.length),stopped,'Pause cancels output');
 assert.deepEqual(errors,[]);
 console.log('Tempo browser passed: 60/70/120%, pause, reset, saved speed, import, audio, unchanged MIDI pitches/timing and mobile controls.');
}finally{await browser.close();await server.close();}
