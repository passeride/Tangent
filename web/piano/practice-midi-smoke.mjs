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
    window.midiSent=[];
    const port={id:'p45-test',name:'Yamaha P-45',state:'connected',connection:'closed',
      async open(){this.connection='open';return this;},clear(){window.midiSent.push(['clear']);},
      send(bytes){window.midiSent.push([...bytes]);}};
    const input={id:'p45-in',state:'connected',onmidimessage:null};
    const outputs = sessionStorage.getItem('test-missing-output')==='yes' ? [] : [[port.id,port]];
    if(sessionStorage.getItem('test-duplicate-output')==='yes') {
      outputs.push(['p45-duplicate',{...port,id:'p45-duplicate'}]);
    }
    window.fakeMidi={outputs:new Map(outputs),inputs:new Map([[input.id,input]]),onstatechange:null};
    Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>window.fakeMidi});
    Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
  });
  await ready(page,server.url);
  await click(page,'settings');await click(page,'midi');
  await page.waitForFunction(()=>tangentDiagnostics().status.includes('MIDI input'));
  await click(page,'midi-output');
  await page.waitForFunction(()=>tangentDiagnostics().midi_dropdown_open);
  assert((await page.evaluate(()=>tangentDiagnostics().buttons)).some(b=>b.id==='midi-output-computer'));
  await click(page,'midi-output-choice');
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='p45-test'
    && tangentDiagnostics().preferences.midi_output_name==='Yamaha P-45');
  await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);

  // Reload the real Bevy app: the saved name resolves against a new port list.
  await page.reload(); await ready(page,server.url);
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='p45-test');
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.midi_output_name,'Yamaha P-45');

  // An unavailable port keeps the preference but never selects a random port.
  await page.evaluate(()=>sessionStorage.setItem('test-missing-output','yes'));
  await page.reload(); await ready(page,server.url);
  await page.waitForFunction(()=>tangentDiagnostics().midi_outputs.length===0);
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).midi_output,'');
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.midi_output_name,'Yamaha P-45');
  await page.evaluate(()=>{sessionStorage.removeItem('test-missing-output');sessionStorage.setItem('test-duplicate-output','yes');});
  await page.reload(); await ready(page,server.url);
  await page.waitForFunction(()=>tangentDiagnostics().midi_outputs.length===2);
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).midi_output,'','Duplicate device names must not auto-route playback');
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.midi_output_name,'Yamaha P-45');
  await page.evaluate(()=>sessionStorage.removeItem('test-duplicate-output'));
  await page.reload(); await ready(page,server.url);
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='p45-test');

  // Explicit computer output persists too (not a stale Yamaha selection).
  await click(page,'settings');
  await click(page,'midi-output');
  await click(page,'midi-output-computer');
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='' && tangentDiagnostics().preferences.midi_output_name===null);
  await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
  await page.reload(); await ready(page,server.url);
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).midi_output,'');
  await click(page,'settings');await click(page,'midi');
  await page.waitForFunction(()=>tangentDiagnostics().midi_outputs.length===1);
  await click(page,'midi-output');await click(page,'midi-output-choice');
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='p45-test');
  await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
  await page.keyboard.press('Escape');await click(page,'freeform');
  await page.evaluate(()=>window.midiSent=[]);
  await page.evaluate(()=>fakeMidi.inputs.get('p45-in').onmidimessage({data:[0x90,60,100]}));
  await page.waitForFunction(()=>tangentDiagnostics().solid_keys.includes(60));
  assert.deepEqual(await page.evaluate(()=>midiSent.filter(b=>(b[0]&0xf0)===0x90)),[],'No hardware MIDI echo');
  await page.evaluate(()=>fakeMidi.inputs.get('p45-in').onmidimessage({data:[0x80,60,0]}));
  await page.keyboard.down('KeyS');
  await page.waitForFunction(()=>midiSent.some(b=>b[0]===0x9f&&b[1]===62));
  await page.keyboard.up('KeyS');
  await page.waitForFunction(()=>midiSent.some(b=>b[0]===0x8f&&b[1]===62));
  await page.keyboard.press('Escape');await click(page,'main');await click(page,'train');await click(page,'train-notes');
  const first=await page.evaluate(()=>tangentDiagnostics());
  assert.equal(first.practice_hint,false);assert.deepEqual(first.expected_glow,[]);assert.equal(first.music_clefs,2);
  await page.evaluate(()=>window.midiSent=[]);
  await click(page,'practice-listen');
  await page.waitForFunction(()=>midiSent.some(b=>b[0]===0x9f&&b[1]===60));
  await page.waitForFunction(()=>midiSent.some(b=>b[0]===0x8f&&b[1]===60));
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).completed,0,'Reference does not answer the exercise');
  assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).solid_keys,[],'Reference is not player input');
  // Real MIDI, an octave below: show exact staff position and distance, no echo.
  await page.evaluate(()=>fakeMidi.inputs.get('p45-in').onmidimessage({data:[0x90,48,100]}));
  await page.waitForFunction(()=>tangentDiagnostics().practice_played_label?.includes('12 semitones below'));
  await mkdir(new URL('../../dist/verification/',import.meta.url),{recursive:true});
  await page.screenshot({path:new URL('../../dist/verification/train-live-pointer.png',import.meta.url).pathname});
  await page.evaluate(()=>fakeMidi.inputs.get('p45-in').onmidimessage({data:[0x80,48,0]}));
  await page.keyboard.down('KeyS');await page.waitForFunction(()=>tangentDiagnostics().practice_wrong);
  await page.waitForFunction(()=>tangentDiagnostics().practice_played_label?.includes('2 semitones above'));
  // Recover by lifting the wrong finger while the correct note stays down.
  await page.keyboard.down('KeyA');
  await page.keyboard.up('KeyS');
  await page.waitForFunction(()=>tangentDiagnostics().practice_waiting_release);
  assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).target,[60]);
  await page.keyboard.up('KeyA');await page.waitForFunction(()=>tangentDiagnostics().completed===1);
  assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).target,[62]);
  await click(page,'practice-clef');await page.waitForFunction(()=>tangentDiagnostics().practice_clef==='Bass');
  assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).target,[48]);
  await click(page,'practice-hint');await page.waitForFunction(()=>tangentDiagnostics().expected_glow.includes(48));
  await mkdir(new URL('../../dist/verification/',import.meta.url),{recursive:true});
  await page.screenshot({path:new URL('../../dist/verification/train-bass.png',import.meta.url).pathname});
  await click(page,'practice-level');await click(page,'practice-level');
  await page.waitForFunction(()=>tangentDiagnostics().practice_level==='Chromatic');
  await page.setViewportSize({width:390,height:844});
  await page.screenshot({path:new URL('../../dist/verification/train-mobile.png',import.meta.url).pathname});
  const small=await page.evaluate(()=>tangentDiagnostics());
  for(const b of small.buttons)assert.ok(b.x>=0&&b.x+b.w<=391,`Control ${b.id} fits`);
  await page.setViewportSize({width:1280,height:800});
  await page.keyboard.press('Escape');await click(page,'main');await click(page,'start-play');
  await page.keyboard.press('Space');
  await page.waitForFunction(()=>tangentDiagnostics().expected_glow.length>0);
  assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).solid_keys,[],'Song never fills played keys');
  await page.waitForFunction(()=>midiSent.some(b=>(b[0]&0xf0)===0x90&&b[0]!==0x9f));
  await page.keyboard.down('KeyA');await page.waitForFunction(()=>tangentDiagnostics().solid_keys.includes(60));
  await page.screenshot({path:new URL('../../dist/verification/song-glow-pressed.png',import.meta.url).pathname});
  await page.keyboard.up('KeyA');
  await page.keyboard.press('Escape');
  await page.waitForFunction(()=>!tangentDiagnostics().playing);
  await page.waitForTimeout(150);const stopped=await page.evaluate(()=>midiSent.length);
  await page.waitForTimeout(250);assert.equal(await page.evaluate(()=>midiSent.length),stopped,'No scheduled notes after pause');
  // Disconnect is reported, clears output selection and leaves playback paused.
  await page.evaluate(()=>{const p=fakeMidi.outputs.get('p45-test');p.state='disconnected';fakeMidi.onstatechange({port:p});});
  await page.waitForFunction(()=>tangentDiagnostics().midi_output==='');
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).preferences.midi_output_name,'Yamaha P-45');
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).playing,false);
  assert.deepEqual(errors,[]);
  console.log('Real Bevy practice, MIDI output/no echo, glow/solid, pause/disconnect and 390px layout passed.');
} finally {await browser.close();await server.close();}
