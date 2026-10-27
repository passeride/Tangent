import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {chromium} from 'playwright';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs});
const cover=await readFile(new URL('../../crates/tangent-player/tests/fixtures/metadata-cover.png',import.meta.url));
const recording='11111111-1111-1111-1111-111111111111',release='22222222-2222-2222-2222-222222222222';
const result={recordings:[{id:recording,title:'Morning lantern',score:100,'artist-credit':[{name:'Example pianist'}],releases:[{id:release,title:'Quiet keys',date:'2001',country:'NO'},{id:'33333333-3333-3333-3333-333333333333',title:'A different edition',date:'2002',country:'GB'}]}]};
const xml=`<score-partwise version="4.0"><work><work-title>Morning lantern</work-title></work><identification><creator type="composer">Local composer</creator></identification><part-list><score-part id="P"><part-name>Piano</part-name></score-part></part-list><part id="P"><measure number="1"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><type>whole</type></note></measure></part></score-partwise>`;
let debugPage;
try {
 const page=await browser.newPage({viewport:{width:1200,height:800}}),errors=[];let searches=0,covers=0;
 debugPage=page;page.on('pageerror',e=>errors.push(e.message));
 await page.route('https://musicbrainz.org/ws/2/**',async route=>{searches++;assert.equal(route.request().method(),'GET');assert.ok(route.request().url().includes('Morning%20lantern'));await route.fulfill({contentType:'application/json',headers:{'Access-Control-Allow-Origin':'*'},body:JSON.stringify(result)});});
 await page.route('https://coverartarchive.org/**',async route=>{covers++;await route.fulfill({contentType:'image/png',headers:{'Access-Control-Allow-Origin':'*'},body:cover});});
 await ready(page,server.url);assert.equal(searches,0);await click(page,'library');
 const chooser=page.waitForEvent('filechooser');await click(page,'import');await(await chooser).setFiles({name:'Morning_lantern.musicxml',mimeType:'application/xml',buffer:Buffer.from(xml)});
 await page.waitForFunction(()=>tangentDiagnostics().library_total===1);await click(page,'library-edit');
 await click(page,'metadata-open');await page.waitForFunction(()=>tangentDiagnostics().metadata_candidates.length===2,null,{timeout:15000});
 console.log('Metadata: search results loaded');assert.equal(searches,1);assert.equal(covers,0,'Search never downloads source files or unsolicited covers');
 await page.keyboard.press('Enter');const entered=await page.evaluate(()=>tangentDiagnostics().frame);await page.waitForFunction(f=>tangentDiagnostics().frame>=f+2,entered);assert.equal((await page.evaluate(()=>tangentDiagnostics())).metadata_open,true,'Enter does not silently apply provider results');
 await click(page,'metadata-preview');await page.waitForFunction(()=>tangentDiagnostics().metadata_ready,null,{timeout:15000});
 console.log('Metadata: cover preview loaded');assert.equal(covers,1);assert.equal((await page.evaluate(()=>tangentDiagnostics())).library_items[0].enrichment,null,'Preview is not saved');
 for(const [width,height] of [[390,700],[1280,420],[390,420]]){
  await page.setViewportSize({width,height});await page.waitForFunction(w=>tangentDiagnostics().width===w,width);
  for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.x>=0&&b.y>=0&&b.x+b.w<=width&&b.y+b.h<=height,`${b.id} fits ${width}x${height}`);
 }
 await page.setViewportSize({width:1200,height:800});await click(page,'metadata-save');
 await page.waitForFunction(()=>!tangentDiagnostics().metadata_open&&tangentDiagnostics().library_items[0]?.has_cover&&tangentDiagnostics().cover_count===1);
 let entry=(await page.evaluate(()=>tangentDiagnostics())).library_items[0];assert.equal(entry.title,'Morning lantern');assert.equal(entry.composer,'Local composer');assert.equal(entry.enrichment.artists[0],'Example pianist');
 await click(page,'library-cancel-edit');
 if(process.env.TANGENT_CAPTURE_METADATA==='1')await page.screenshot({path:new URL('../../docs/screenshots/library-covers.png',import.meta.url).pathname});
 await page.reload();await ready(page,server.url);await click(page,'library');await page.waitForFunction(()=>tangentDiagnostics().library_items[0]?.has_cover&&tangentDiagnostics().cover_count===1);
 assert.equal(searches,1);assert.equal(covers,1,'Reopen uses persisted cover bytes, with no remote lookup');
 await click(page,'library-edit');await click(page,'metadata-open');await page.waitForFunction(()=>tangentDiagnostics().metadata_candidates.length===2,null,{timeout:15000});
 await click(page,'metadata-preview');await page.waitForFunction(()=>tangentDiagnostics().metadata_ready,null,{timeout:15000});
 if(process.env.TANGENT_CAPTURE_METADATA==='1')await page.screenshot({path:new URL('../../docs/screenshots/metadata-review.png',import.meta.url).pathname});
 await click(page,'metadata-close');await click(page,'metadata-clear');await page.waitForFunction(()=>tangentDiagnostics().library_items[0]?.enrichment===null);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).library_items[0].composer,'Local composer');
 assert.deepEqual(errors,[]);
 console.log('Metadata Bevy: explicit search, review before save, artist/composer separation, local covers after reload, removal, and narrow/short preview controls passed.');
}catch(error){if(debugPage)console.error('Metadata diagnostic:',await debugPage.evaluate(()=>{const d=tangentDiagnostics();return {open:d.metadata_open,status:d.metadata_status,library_status:d.library_status,candidates:d.metadata_candidates,buttons:d.buttons.map(b=>b.id)}}).catch(()=>null));throw error;}finally{await browser.close();await server.close();}
