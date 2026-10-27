import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname);
const browser=await chromium.launch({headless:true,args:gpuArgs});
try {
 const page=await browser.newPage({viewport:{width:1280,height:960}}),errors=[];
 page.on('pageerror',e=>errors.push(e.message));
 await ready(page,server.url);await click(page,'start-play');
 const chooser=page.waitForEvent('filechooser');await page.keyboard.press('Control+KeyO');
 await(await chooser).setFiles({name:'Rhythm groups.musicxml',mimeType:'application/xml',buffer:await readFile(new URL('../../crates/tangent-musicxml/tests/fixtures/beams.musicxml',import.meta.url))});
 await page.waitForFunction(()=>tangentDiagnostics().title==='Rhythm groups'&&tangentDiagnostics().written_measures===6);
 if(!await page.evaluate(()=>tangentDiagnostics().help))await click(page,'help');
 await click(page,'settings');await click(page,'settings-visuals');
 if(!await page.evaluate(()=>tangentDiagnostics().show_score))await click(page,'toggle-score');
 if(await page.evaluate(()=>tangentDiagnostics().show_vertical))await click(page,'toggle-vertical');
 await click(page,'score-larger');await page.waitForFunction(()=>tangentDiagnostics().score_scale===125);
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);await click(page,'play');
 await page.keyboard.press('Space');await page.waitForFunction(()=>!tangentDiagnostics().playing&&!tangentDiagnostics().help);
 await page.waitForFunction(()=>tangentDiagnostics().beam_groups.length>10);
 const report=await page.evaluate(()=>tangentDiagnostics());
 assert.ok(report.beam_groups.some(g=>g.stems===2&&g.notes===4),'A two-chord group has one stem per chord');
 assert.ok(report.beam_groups.some(g=>g.up)&&report.beam_groups.some(g=>!g.up),'Up/down stems are both drawn');
 assert.ok(report.beam_groups.some(g=>g.levels.includes(3)),'32nd notes have a third beam');
 assert.ok(report.beam_groups.some(g=>g.hooks>0),'Mixed durations have partial secondary beams');
 assert.ok(report.beam_groups.some(g=>g.measure==='3'&&g.stems===3),'6/8 groups three eighths per beat');
 assert.ok(report.beam_groups.some(g=>g.measure==='4'&&g.stems===4),'Authored group overrides the beat boundary');
 assert.ok(report.beam_groups.every(g=>Number.isFinite(g.slope)));
 if(process.env.TANGENT_CAPTURE_BEAMS==='1')await page.screenshot({path:new URL('../../docs/screenshots/grouped-notes.png',import.meta.url).pathname});
 for(const [width,height] of [[390,844],[1280,420]]) {
  await page.setViewportSize({width,height});await page.waitForFunction(w=>tangentDiagnostics().width===w,width);
  await page.waitForFunction(()=>tangentDiagnostics().beam_groups.some(g=>g.notes===4&&g.stems===2));
  assert.deepEqual(errors,[]);
  if(process.env.TANGENT_CAPTURE_BEAMS==='1'&&width===390)await page.screenshot({path:new URL('../../docs/screenshots/grouped-notes-mobile.png',import.meta.url).pathname});
 }
 // Returning to an unbeamed view exercises pooled rotation reset as shapes are reused.
 await click(page,'help');await click(page,'settings');await click(page,'settings-visuals');await click(page,'toggle-score');await click(page,'toggle-vertical');
 await page.keyboard.press('Escape');await page.waitForFunction(()=>!tangentDiagnostics().settings_open);await click(page,'play');
 await page.waitForFunction(()=>tangentDiagnostics().show_vertical&&!tangentDiagnostics().show_score&&tangentDiagnostics().beam_groups.length===0);
 assert.deepEqual(errors,[]);
 console.log('Beaming: authored/automatic groups, shared chord stems, 6/8 beats, 32nd/partial beams, directions, scaled systems and view switching passed.');
} finally {await browser.close();await server.close();}
