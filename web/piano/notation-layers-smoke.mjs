import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}), errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  const input={id:'in',name:'Simulated piano',state:'connected',onmidimessage:null};
  window.fakeMidi={inputs:new Map([[input.id,input]]),outputs:new Map(),onstatechange:null};
  Object.defineProperty(navigator,'requestMIDIAccess',{value:async()=>window.fakeMidi});
  Object.defineProperty(navigator,'permissions',{configurable:true,value:{query:async()=>({state:'granted'})}});
 });
 await ready(page,server.url);
 await click(page,'settings');await click(page,'midi');
 await page.waitForFunction(()=>fakeMidi.inputs.get('in').onmidimessage);
 await page.keyboard.press('Escape');await click(page,'start-play');
 await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');
 for(const id of ['toggle-score','toggle-vertical','vertical-style','note-names'])
  assert.ok((await page.evaluate(()=>tangentDiagnostics().buttons)).some(b=>b.id===id),`${id} is available`);
 await click(page,'toggle-score');await click(page,'note-names');
 await page.waitForFunction(()=>tangentDiagnostics().show_score && tangentDiagnostics().show_vertical && tangentDiagnostics().note_names);
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);await click(page,'play');
 await page.waitForFunction(()=>!tangentDiagnostics().help);
 await page.waitForFunction(()=>tangentDiagnostics().block_labels.length>0);
 assert.ok((await page.evaluate(()=>tangentDiagnostics().block_labels)).every(n=>/^[A-G]#?\d$/.test(n)));
 await page.evaluate(()=>fakeMidi.inputs.get('in').onmidimessage({data:[0x90,60,100]}));
 await page.waitForFunction(()=>tangentDiagnostics().live.includes(60)&&tangentDiagnostics().visible_labels.includes('Played C4'));
 await page.evaluate(()=>fakeMidi.inputs.get('in').onmidimessage({data:[0x90,61,100]}));
 await page.waitForFunction(()=>tangentDiagnostics().visible_labels.includes('Played C4 / C#4'));
 if(process.env.TANGENT_NOTATION_PREVIEW)await page.screenshot({path:process.env.TANGENT_NOTATION_PREVIEW});
 await page.evaluate(()=>{for(const k of [60,61])fakeMidi.inputs.get('in').onmidimessage({data:[0x80,k,0]});});
 await page.waitForFunction(()=>!tangentDiagnostics().live.length);
 for(const [width,height] of [[390,700],[1280,420]]) {
  await page.setViewportSize({width,height}); await page.waitForFunction(w=>tangentDiagnostics().width===w,width);
  await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');
  for(const id of ['toggle-score','toggle-vertical','vertical-style','note-names']) {
   const b=(await page.evaluate(()=>tangentDiagnostics().buttons)).find(x=>x.id===id);
   assert.ok(b && b.x>=0 && b.y>=0 && b.x+b.w<=width && b.y+b.h<=height,`${id} fits ${width}x${height}`);
  }
  await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);await click(page,'play');
 }
 await page.reload();await ready(page,server.url);await click(page,'start-play');
 await page.waitForFunction(()=>tangentDiagnostics().show_score && tangentDiagnostics().show_vertical && tangentDiagnostics().note_names);
 await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');await click(page,'toggle-vertical');
 await page.waitForFunction(()=>tangentDiagnostics().show_score&&!tangentDiagnostics().show_vertical);
 await click(page,'toggle-score');await page.waitForFunction(()=>!tangentDiagnostics().show_score&&!tangentDiagnostics().show_vertical);
 assert.deepEqual(errors,[]);
 console.log('Notation layers: combined and individual views, live chords, block labels, persistence and responsive controls passed.');
} finally {await browser.close();await server.close();}
