import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
 await ready(page,server.url);console.log('HUD: booted');await click(page,'start-play');
 const events=[];for(const key of [24,60,90,112])events.push(0,0x90,key,100,0x9e,0,0x80,key,0);events.push(0,0xff,0x2f,0);
 const track=Buffer.from(events),header=Buffer.from([77,84,104,100,0,0,0,6,0,0,0,1,1,224,77,84,114,107,0,0,0,track.length]);
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO',{delay:100});await(await chooser).setFiles({name:'Range check.mid',mimeType:'audio/midi',buffer:Buffer.concat([header,track])});
 await page.waitForFunction(()=>tangentDiagnostics().notes===4);await click(page,'help');await page.keyboard.press('KeyR');
 await page.waitForFunction(()=>tangentDiagnostics().key_rects.length===49);console.log('HUD: range checks');
 let d=await page.evaluate(()=>tangentDiagnostics());assert.deepEqual(d.offscreen,[1,2]);
 await click(page,'fit-range');await page.waitForFunction(()=>tangentDiagnostics().key_rects.length===88);
 d=await page.evaluate(()=>tangentDiagnostics());assert.deepEqual(d.offscreen,[0,1]);assert(d.hud_labels.some(s=>s.includes('higher')),'Out-of-piano note stays visible');
 await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');
 const initial=await page.evaluate(()=>tangentDiagnostics().preferences);assert.equal(initial.hud,1);
 await click(page,'hud');assert.equal((await page.evaluate(()=>tangentDiagnostics().preferences)).hud,2);
 await click(page,'settings-back');await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().hud_labels.some(s=>s.includes('pts')));
 await page.screenshot({path:'../../docs/screenshots/hud-arcade.png'});console.log('HUD: arcade');
 await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');await click(page,'hud');
 await click(page,'settings-back');await click(page,'help');
 d=await page.evaluate(()=>tangentDiagnostics());assert.equal(d.preferences.hud,0);assert.equal(d.preferences.effect_strength,initial.effect_strength);assert(!d.hud_labels.some(s=>s.includes('pts')));
 assert(d.hud_labels.some(s=>s.includes('Computer audio')),'Focus retains actual output');
 console.log('HUD: focus');await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);console.log('HUD: reload');await page.reload();await page.waitForFunction(()=>window.tangentDiagnostics?.()?.storage.ready);
 await page.waitForFunction(()=>tangentDiagnostics().preferences.hud===0);console.log('HUD: restored');
 await click(page,'start-play');await page.setViewportSize({width:3840,height:2160});await page.waitForFunction(()=>tangentDiagnostics().width===3840);
 await page.screenshot({path:'../../docs/screenshots/hud-focus-4k.png'});console.log('HUD: 4K');
 await click(page,'help');await page.setViewportSize({width:640,height:450});await page.waitForFunction(()=>tangentDiagnostics().width===640);
 await click(page,'passage-setup');assert.equal((await page.evaluate(()=>tangentDiagnostics())).settings_tab,'Practice');
 for(const size of [{width:390,height:450},{width:640,height:360}]) {
  await page.setViewportSize(size);await page.waitForFunction(w=>tangentDiagnostics().width===w,size.width);
  const buttons=await page.evaluate(()=>tangentDiagnostics().buttons);
  for(const b of buttons)assert(b.x>=0&&b.y>=0&&b.x+b.w<=size.width&&b.y+b.h<=size.height,`Clipped ${b.id} at ${JSON.stringify(size)}: ${JSON.stringify(b)}`);
 }
 await click(page,'settings-back');
 d=await page.evaluate(()=>tangentDiagnostics());for(const b of d.buttons)assert(b.y>=0&&b.y+b.h<=360,`Short pause clipped ${b.id}`);
 await page.screenshot({path:'../../docs/screenshots/pause-short.png'});
 assert.deepEqual(errors,[]);
 console.log('HUD browser: independent persistent presets, actual output in Focus, honest offscreen counts, fit-range, 4K and short-window passage controls passed.');
}finally{await browser.close();await server.close();}
