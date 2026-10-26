import assert from 'node:assert/strict';
import {MidiOutput,midiBytes,restoreMidi} from './midi-output.js';
const sent=[],fail=[];
const port={send:b=>sent.push([...b]),clear:()=>sent.push('clear')};
const out=new MidiOutput(e=>fail.push(e));out.select(port);
out.on('midi:P45:0:60',60,100);assert.deepEqual(sent,[],'Never echo hardware input');
out.on('key:0',60,100);out.off('key:0');assert.deepEqual(sent,[[0x9f,60,100],[0x8f,60,0]]);
out.on('touch:1',62,100);out.panic();assert.equal(out.live.size,0);assert.ok(sent.includes('clear'));assert.ok(sent.some(b=>b[0]===0xbf&&b[1]===64&&b[2]===0));
assert.deepEqual(midiBytes({channel:1,message:{Bend:{value:16383}}}),[0xe1,127,127]);
assert.equal(midiBytes({channel:9,message:{On:{key:60,velocity:90}}}),null);
const events=[{time_us:0,channel:0,message:{Control:{controller:64,value:127}}},{time_us:10,channel:0,message:{On:{key:60,velocity:80}}},{time_us:20,channel:0,message:{Off:{key:60}}},{time_us:30,channel:0,message:{Control:{controller:64,value:0}}}];
assert.deepEqual(restoreMidi(events,25),[[0xb0,64,127],[0x90,60,80],[0x80,60,0]]);
assert.deepEqual(restoreMidi(events,31),[[0xb0,64,0]]);
// Switching destinations silences the old port, then leaves the new one silent.
sent.length=0;const next=[];out.select({send:b=>next.push(b),clear(){}});assert.ok(sent.length>0);assert.equal(next.length,0);
out.select({send(){throw Error('unplugged')},clear(){}});out.on('key:0',60,90);assert.equal(out.port,null);assert.equal(fail.length,1);
console.log('MIDI output: encoding, seek/sustain, no echo, release, switch and disconnect passed.');

const boundaryEvents=[{time_us:0,channel:0,message:{On:{key:60,velocity:90}}},{time_us:500,channel:0,message:{Off:{key:60}}},{time_us:500,channel:0,message:{On:{key:62,velocity:90}}}];
assert.equal(restoreMidi(boundaryEvents,500,true).filter(b=>(b[0]&240)===144).length,0,'A guided gate restores releases, never its pending attack');
const captured=[];const preserved=new MidiOutput(()=>{});preserved.port={send:b=>captured.push([...b]),clear(){}};
preserved.on('key:test',64,90);preserved.event({channel:15,message:{On:{key:67,velocity:90}}});captured.length=0;
preserved.panic(true);
assert.equal(preserved.live.get('key:test'),64);
assert(!captured.some(b=>b[0]===0xbf&&(b[1]===120||b[1]===123)));
assert(!captured.some(b=>b[0]===0x8f&&b[1]===64));
assert(captured.some(b=>b[0]===0x8f&&b[1]===67));
preserved.off('key:test');assert.deepEqual(captured.at(-1),[0x8f,64,0]);
console.log('Guided MIDI boundary: no pending/ended attacks and held live voices survive song resets.');
