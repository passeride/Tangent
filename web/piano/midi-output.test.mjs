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
