import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {mkdtemp,mkdir,writeFile,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {serve,ready,click,gpuArgs} from './browser-helpers.mjs';
const xml=title=>`<score-partwise version="4.0"><work><work-title>${title}</work-title></work><part-list><score-part id="P"><part-name>Piano</part-name></score-part></part-list><part id="P"><measure><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>`;
const server=await serve(new URL('./',import.meta.url).pathname),browser=await chromium.launch({headless:true,args:gpuArgs}),temp=await mkdtemp(join(tmpdir(),'tangent-folder-'));
try{
 const page=await browser.newPage({viewport:{width:1200,height:800}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{window.showDirectoryPicker=async()=>{const root=await navigator.storage.getDirectory();return root.getDirectoryHandle('Selected music',{create:true});};});
 await ready(page,server.url);const title=await page.evaluate(()=>tangentDiagnostics().title);await click(page,'library');
 const chooser=page.waitForEvent('filechooser');await click(page,'import');const picked=await chooser;assert.equal(picked.isMultiple(),true);
 await picked.setFiles(Array.from({length:25},(_,i)=>({name:`Batch${i}.musicxml`,mimeType:'application/xml',buffer:Buffer.from(xml(`Batch ${i}`))})));
 await page.waitForFunction(()=>tangentDiagnostics().library_total===25&&!tangentDiagnostics().imports.pending,null,{timeout:120000});assert.equal((await page.evaluate(()=>tangentDiagnostics())).title,title,'Bulk import does not replace the active piece');
 console.log('Folders: imported a 25-file selection');
 async function put(name,body){await page.evaluate(async({name,body})=>{const root=await navigator.storage.getDirectory(),folder=await root.getDirectoryHandle('Selected music',{create:true}),sub=await folder.getDirectoryHandle('Nested',{create:true}),file=await sub.getFileHandle(name,{create:true}),writer=await file.createWritable();await writer.write(body);await writer.close();},{name,body});}
 await put('new.musicxml',xml('Folder one'));await put('duplicate.MXL',xml('Batch 0'));await put('ignored.txt','ignore');await put('broken.mid','bad');
 await click(page,'library-folder');await click(page,'folder-choose');await page.waitForFunction(()=>tangentDiagnostics().library_total===26&&!tangentDiagnostics().folder_busy,null,{timeout:120000});
 assert.ok((await page.evaluate(()=>tangentDiagnostics())).folder_status.includes('1 unchanged'));assert.ok((await page.evaluate(()=>tangentDiagnostics())).folder_status.includes('1 failed'));
 for(const [width,height] of [[390,700],[1280,420]]){await page.setViewportSize({width,height});await page.waitForFunction(w=>tangentDiagnostics().width===w,width);for(const b of (await page.evaluate(()=>tangentDiagnostics())).buttons)assert.ok(b.x>=0&&b.x+b.w<=width&&b.y+b.h<=height,`${b.id} fits`);}
 await page.setViewportSize({width:1200,height:800});await put('later.musicxml',xml('Folder two'));await click(page,'folder-scan');await page.waitForFunction(()=>tangentDiagnostics().library_total===27&&!tangentDiagnostics().folder_busy);
 if(process.env.TANGENT_CAPTURE_FOLDER==='1')await page.screenshot({path:new URL('../../docs/screenshots/library-folder.png',import.meta.url).pathname});
 await put('restart.musicxml',xml('Folder restart'));await page.reload();await ready(page,server.url);await click(page,'library');await page.waitForFunction(()=>tangentDiagnostics().library_total===28&&!tangentDiagnostics().folder_busy,null,{timeout:120000});assert.equal((await page.evaluate(()=>tangentDiagnostics())).folder_path,'Selected music');
 await click(page,'library-folder');await click(page,'folder-clear');await page.waitForFunction(()=>!tangentDiagnostics().folder_path);assert.equal((await page.evaluate(()=>tangentDiagnostics())).library_total,28,'Removing a folder setting keeps imported music');
 // Older browsers use the real directory-upload chooser, including nested paths.
 await page.evaluate(()=>window.showDirectoryPicker=undefined);await mkdir(join(temp,'nested'));await writeFile(join(temp,'nested','fallback.musicxml'),xml('Fallback folder'));await writeFile(join(temp,'ignored.txt'),'ignored');
 const fallback=page.waitForEvent('filechooser');await click(page,'folder-choose');await(await fallback).setFiles(temp);await page.waitForFunction(()=>tangentDiagnostics().library_total===29&&!tangentDiagnostics().folder_busy);
 assert.equal((await page.evaluate(()=>tangentDiagnostics())).folder_auto,false);assert.deepEqual(errors,[]);
 console.log('Folder Bevy: 25-file batch, nested scan/dedup/bad-file recovery, rescan, startup permission-backed import, active-piece preservation, clear and directory-upload fallback passed.');
}finally{await browser.close();await server.close();await rm(temp,{recursive:true,force:true});}
