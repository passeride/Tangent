import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  const input={id:'flow-in',name:'P-45 simulated',state:'connected',onmidimessage:null};
  window.fakeMidi={inputs:new Map([[input.id,input]]),outputs:new Map(),onstatechange:null};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>window.fakeMidi});
  Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
 });
 await ready(page,server.url);await click(page,'settings');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('flow-in').onmidimessage);
 await page.keyboard.press('Escape');await click(page,'train');await click(page,'train-five-finger');
 await page.waitForFunction(()=>tangentDiagnostics().mode==='FiveFinger');
 assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).flow_notes,[60,62,64,65,67,65,64,62,60]);
 async function midi(key,down){
  await page.evaluate(({key,down})=>fakeMidi.inputs.get('flow-in').onmidimessage({data:[down?0x90:0x80,key,down?90:0]}),{key,down});
  await page.waitForFunction(({key,down})=>tangentDiagnostics().solid_keys.includes(key)===down,{key,down});
 }
 async function step(){
  const before=await page.evaluate(()=>({key:tangentDiagnostics().target[0],step:tangentDiagnostics().flow_step,trials:tangentDiagnostics().practice_run.trials.length}));
  await midi(before.key,true);
  const held=await page.evaluate(()=>tangentDiagnostics());
  assert.equal(held.practice_waiting_release,true);assert.equal(held.flow_step,before.step);
  assert.equal(held.practice_run.trials.length,before.trials+Number(before.step===8),'Only whole patterns earn trials');
  await midi(before.key,true);
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).flow_step,before.step,'Repeated held input does not advance');
  await midi(before.key,false);
  await page.waitForFunction(old=>tangentDiagnostics().screen==='Results'||tangentDiagnostics().flow_step===(old===8?0:old+1),before.step);
 }
 async function remainder(){let count=await page.evaluate(()=>9-tangentDiagnostics().flow_step);for(let i=0;i<count;i++)await step();}
 // Wrong octave and future-step input do not advance, even after correction.
 for(const key of [72,64]){await midi(key,true);await page.waitForFunction(()=>tangentDiagnostics().practice_wrong);await midi(key,false);}
 await remainder();assert.equal((await page.evaluate(()=>tangentDiagnostics())).practice_run.points,40);
 for(let i=0;i<3;i++)await step();
 await click(page,'practice-hint');await page.waitForFunction(()=>tangentDiagnostics().expected_glow.includes(65));
 await click(page,'practice-hint');await remainder();
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).practice_run.points,40,'Help remains assisted through later steps');
 for(let i=0;i<3;i++)await step();
 await click(page,'practice-listen');await page.waitForFunction(()=>tangentDiagnostics().reference_keys.length>0);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).practice_run.trials.length,2);
 assert.deepEqual((await page.evaluate(()=>tangentDiagnostics())).solid_keys,[],'Reference playback never becomes player input');
 await click(page,'practice-listen');await remainder();
 await step();await step();await click(page,'practice-skip');
 await page.waitForFunction(()=>tangentDiagnostics().flow_step===0&&tangentDiagnostics().practice_run.trials.length===4);
 for(let i=4;i<12;i++)await remainder();
 await page.waitForFunction(()=>tangentDiagnostics().screen==='Results'&&tangentDiagnostics().progress_pending===0&&tangentDiagnostics().storage.pending===0);
 const result=await page.evaluate(()=>tangentDiagnostics().practice_result);
 assert.equal(result.mode,'five-finger-flow');assert.equal(result.points,840);assert.equal(result.trials.length,12);
 assert.equal(result.trials[0].skill,'flow:1:R:60-62-64-65-67-65-64-62-60');
 assert.equal(result.trials[1].assisted,true);assert.equal(result.trials[2].assisted,true);assert.equal(result.trials[3].skipped,true);
 await page.reload();await ready(page,server.url);await click(page,'progress');
 await page.waitForFunction(()=>tangentDiagnostics().progress_history.some(r=>r.mode==='five-finger-flow'&&r.points===840));
 await click(page,'progress-open');await click(page,'practice-again');
 await page.waitForFunction(()=>tangentDiagnostics().mode==='FiveFinger'&&tangentDiagnostics().flow_step===0);
 await click(page,'practice-level');await page.waitForFunction(()=>tangentDiagnostics().flow_left&&tangentDiagnostics().target[0]===48);
 await click(page,'practice-level');await page.waitForFunction(()=>tangentDiagnostics().preferences.practice_level===2);
 await page.keyboard.press('Escape');await click(page,'settings');await click(page,'settings-audio');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('flow-in').onmidimessage);
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().help);
 await remainder();assert.equal((await page.evaluate(()=>tangentDiagnostics())).target[0],55);
 await step();await step();await click(page,'practice-skip');
 await page.waitForFunction(()=>tangentDiagnostics().target[0]===65&&tangentDiagnostics().flow_step===0);
 if(process.env.TANGENT_CAPTURE_FLOW==='1')await page.screenshot({path:new URL('../../docs/screenshots/five-finger-flow.png',import.meta.url).pathname});
 for(const [width,height,name] of [[390,700,'mobile'],[1280,420,'short']]){
  await page.setViewportSize({width,height});await page.waitForFunction(({width,height})=>tangentDiagnostics().width===width&&tangentDiagnostics().height===height,{width,height});
  for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.x>=0&&b.x+b.w<=width&&b.y+b.h<=height,`${b.id} fits ${name}`);
  if(process.env.TANGENT_CAPTURE_FLOW==='1')await page.screenshot({path:new URL(`../../docs/screenshots/five-finger-flow-${name}.png`,import.meta.url).pathname});
 }
 // Pause releases voices and retains the partially completed sequence position.
 await step();await page.keyboard.press('Escape');await page.waitForFunction(()=>tangentDiagnostics().help);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).flow_step,1);
 await click(page,'play');await page.waitForFunction(()=>!tangentDiagnostics().help);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).flow_step,1);
 await page.keyboard.press('Escape');await click(page,'main');await click(page,'train');
 for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.y+b.h<=420,`${b.id} fits five-card Train menu`);
 assert.deepEqual(errors,[]);
 console.log('Five Finger Flow: 12 complete MIDI patterns, exact order/release, whole-pattern scoring, sticky help, partial skip, durable retry, levels, pause and responsive UI passed.');
}finally{await browser.close();await server.close();}
