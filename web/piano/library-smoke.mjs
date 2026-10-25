import assert from "node:assert/strict";
import { readFile, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const server=await serve(new URL("./",import.meta.url).pathname);
const profile=await mkdtemp(join(tmpdir(),"tangent-library-"));
let context;
const options={ headless:true,args:gpuArgs,viewport:{width:1100,height:800} };
try {
  context=await chromium.launchPersistentContext(profile,options);
  let page=await context.newPage();
  page.on("console",message=>console.log("LIBRARY BROWSER",message.text()));
  page.on("pageerror",error=>console.error("LIBRARY PAGE ERROR",error.message));
  await page.addInitScript(()=>{
    Object.defineProperty(window,"tangentPlatform",{configurable:true,set(platform){
      const action=platform.action,store=platform.store_import,pull=platform.pull;
      platform.action=function(name,data){if(["import-result","library-list"].includes(name)) console.log("adapter action",name,data);return action.call(this,name,data);};
      platform.store_import=function(bytes,data){const meta=JSON.parse(data);console.log("adapter save",bytes.length,meta.title,Boolean(meta.fingerprint));return store.call(this,bytes,data);};
      platform.pull=function(){const value=pull.call(this);for(const item of JSON.parse(value)) if(item.type.startsWith("library")||item.type==="file") console.log("adapter event",item.type,item.text??item.title??item.total,item.import_ticket??item.ticket??"");return value;};
      delete window.tangentPlatform;window.tangentPlatform=platform;
    }});
  });
  await ready(page,server.url);await click(page,"library");
  const buffer=await readFile(new URL("samples/first-light.musicxml",import.meta.url));
  const second=Buffer.from(buffer.toString().replace("First light - Tangent piano study","Second test study"));
  const firstFile={name:"first.musicxml",mimeType:"application/xml",buffer};
  await click(page,"import");
  await page.locator("#midi-file").setInputFiles([firstFile,{name:"second.musicxml",mimeType:"application/xml",buffer:second}]);
  await page.waitForFunction(()=>tangentDiagnostics().library_total===2 && !document.querySelector("#midi-file").value);
  await click(page,"library-sort");
  await page.waitForFunction(()=>tangentDiagnostics().library_items[0]?.title==="First light - Tangent piano study");
  await click(page,"library-edit");await click(page,"library-title");await page.waitForFunction(()=>tangentDiagnostics().library_focus==="title");
  await page.keyboard.press("Control+A");await page.keyboard.type("Evening study");
  await click(page,"library-composer");await page.waitForFunction(()=>tangentDiagnostics().library_focus==="composer");await page.keyboard.press("Control+A");await page.keyboard.type("Bach");
  await click(page,"library-save-metadata");
  await page.waitForFunction(()=>!tangentDiagnostics().library_editing && tangentDiagnostics().library_items.some(s=>s.title==="Evening study"&&s.composer==="Bach"));
  await click(page,"library-search");await page.waitForFunction(()=>tangentDiagnostics().library_focus==="search");await page.keyboard.type("BACH");
  await page.waitForFunction(()=>tangentDiagnostics().library_total===1&&tangentDiagnostics().library_items[0]?.composer==="Bach");
  await click(page,"library-format"); // MIDI excludes our MusicXML sources.
  await page.waitForFunction(()=>tangentDiagnostics().library_total===0);
  await click(page,"library-format"); // MusicXML includes the edited piece.
  await page.waitForFunction(()=>tangentDiagnostics().library_total===1);
  await click(page,"import");await page.locator("#midi-file").setInputFiles(firstFile);
  await page.waitForFunction(()=>!document.querySelector("#midi-file").value&&tangentDiagnostics().storage.pending===0);
  await page.waitForFunction(()=>tangentDiagnostics().library_items[0]?.title==="Evening study");
  await context.close();context=await chromium.launchPersistentContext(profile,options);
  page=await context.newPage();await ready(page,server.url);await click(page,"library");
  await page.waitForFunction(()=>tangentDiagnostics().library_total===2);
  await click(page,"library-search");await page.waitForFunction(()=>tangentDiagnostics().library_focus==="search");await page.keyboard.type("bach");
  await page.waitForFunction(()=>tangentDiagnostics().library_total===1&&tangentDiagnostics().library_items[0]?.title==="Evening study");
  await click(page,"library-open");
  await page.waitForFunction(()=>tangentDiagnostics().screen==="Session"&&tangentDiagnostics().title==="Evening study"&&tangentDiagnostics().status.includes("notes loaded"));
  await click(page,"help");await click(page,"main");await click(page,"library");
  await page.waitForFunction(()=>tangentDiagnostics().library_total===1);
  await click(page,"library-remove");
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
  assert(await page.evaluate(()=>tangentDiagnostics().buttons.some(b=>b.id==="library-open")));
  await click(page,"library-remove");await page.waitForFunction(()=>tangentDiagnostics().library_total===0);
  await click(page,"library-clear-search");await page.waitForFunction(()=>tangentDiagnostics().library_total===1);
  await page.setViewportSize({width:390,height:720});
  await page.waitForFunction(()=>tangentDiagnostics().width===390);
  const buttons=await page.evaluate(()=>tangentDiagnostics().buttons);
  for(const button of buttons) assert(button.x>=0&&button.y>=0&&button.x+button.w<=390&&button.y+button.h<=720,JSON.stringify(button));
  console.log("Library browser passed: batch import, real Bevy search/edit/filter, reimport preservation, full restart, edited reopen, deletion and mobile layout.");
} catch(error) {
  if(context) for(const page of context.pages()) {
    try { console.error("LIBRARY FAILURE STATE",JSON.stringify(await page.evaluate(()=>({
      diagnostics:window.tangentDiagnostics?.(),fileValue:document.querySelector("#midi-file")?.value,
      startup:document.querySelector("#startup")?.textContent
    })))); } catch {}
  }
  throw error;
} finally {await context?.close();await server.close();await rm(profile,{recursive:true,force:true});}
