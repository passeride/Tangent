// Device capability only: the Bevy scene still owns selection and transport.
export function midiBytes(e) {
  const ch=e.channel, m=e.message;
  if (!Number.isInteger(ch)||ch<0||ch>15||ch===9) return null;
  const seven=n=>Number.isInteger(n)&&n>=0&&n<128;
  if(m.On&&seven(m.On.key)&&seven(m.On.velocity)) return [0x90|ch,m.On.key,m.On.velocity];
  if(m.Off&&seven(m.Off.key)) return [0x80|ch,m.Off.key,0];
  if(m.Control&&seven(m.Control.controller)&&seven(m.Control.value)) return [0xb0|ch,m.Control.controller,m.Control.value];
  if(m.Program&&seven(m.Program.program)) return [0xc0|ch,m.Program.program];
  if(m.Bend&&Number.isInteger(m.Bend.value)&&m.Bend.value>=0&&m.Bend.value<16384) return [0xe0|ch,m.Bend.value&127,m.Bend.value>>7];
  return null;
}
export function restoreMidi(events,position,boundary=false) {
  const controls=new Map(), keys=new Map(), sustained=new Map(), pedal=new Set();
  const clearChannel=(map,ch)=>{for(const [k,v] of map) if(v.ch===ch) map.delete(k);};
  const push=(map,ch,key,velocity)=>{const id=`${ch}:${key}`;if(!map.has(id))map.set(id,{ch,key,velocities:[]});map.get(id).velocities.push(velocity);};
  for(const e of events) {
    if(e.time_us>position||e.time_us===position&&!boundary) break;
    if(boundary&&e.time_us===position&&e.message.On)continue;
    const b=midiBytes(e);if(!b)continue;
    const ch=e.channel,m=e.message;
    if(m.On&&m.On.velocity) push(keys,ch,m.On.key,m.On.velocity);
    else if(m.Off||m.On) {
      const key=(m.Off??m.On).key,n=keys.get(`${ch}:${key}`),v=n?.velocities.shift();
      if(v!==undefined&&pedal.has(ch))push(sustained,ch,key,v);
    } else if(m.Control) {
      const {controller:c,value:v}=m.Control;
      if(c===64) {if(v>=64)pedal.add(ch);else{pedal.delete(ch);clearChannel(sustained,ch);}}
      if(c===120||c===123) {
        if(c===123&&pedal.has(ch))for(const n of keys.values())if(n.ch===ch)for(const v of n.velocities)push(sustained,ch,n.key,v);
        clearChannel(keys,ch);if(c===120)clearChannel(sustained,ch);
      } else if(c===121) {clearChannel(controls,ch);pedal.delete(ch);clearChannel(sustained,ch);}
      else controls.set(`${ch}:${c}`,{ch,b});
    } else controls.set(`${ch}:${b[0]&240}`,{ch,b});
  }
  const result=[...controls.values()].map(v=>v.b);
  for(const n of sustained.values())for(const v of n.velocities)result.push([0x90|n.ch,n.key,v],[0x80|n.ch,n.key,0]);
  for(const n of keys.values())for(const v of n.velocities)result.push([0x90|n.ch,n.key,v]);
  return result;
}
export class MidiOutput {
  port=null;
  live=new Map();
  songKeys=new Set();
  constructor(fail) {this.fail=fail;}
  send(bytes) {
    try {this.port?.send(bytes);} catch(e) {this.port=null;this.live.clear();this.fail(e);}
  }
  panic(preserveLive=false) {
    // No future timestamped messages: clearing and silencing also cancels
    // queued output on pause, seek, focus loss, device change and navigation.
    try{this.port?.clear();}catch{}
    for(let ch=0;ch<16;ch++)if(ch!==9) {
      const preserve=preserveLive&&ch===15;
      for(const [c,v] of (preserve?[[64,0],[121,0]]:[[64,0],[123,0],[120,0],[121,0]]))this.send([0xb0|ch,c,v]);
      if(preserve)for(const key of this.songKeys)if(![...this.live.values()].includes(key))this.send([0x8f,key,0]);
      this.send([0xe0|ch,0,64]);
    }
    this.songKeys.clear();
    if(!preserveLive)this.live.clear();
  }
  select(port) {this.panic();this.port=port;}
  on(id,key,velocity) {
    if(id.startsWith("midi:"))return; // Local Control On: avoid a second attack.
    this.off(id);this.send([0x9f,key,velocity]);this.live.set(id,key);
  }
  off(id) {const key=this.live.get(id);this.live.delete(id);if(key!==undefined&&![...this.live.values()].includes(key))this.send([0x8f,key,0]);}
  event(e) {const bytes=midiBytes(e);if(bytes){if(bytes[0]===0x9f&&bytes[2])this.songKeys.add(bytes[1]);this.send(bytes);}}
  restore(events,position,boundary=false) {for(const bytes of restoreMidi(events,position,boundary)){if(bytes[0]===0x9f&&bytes[2])this.songKeys.add(bytes[1]);this.send(bytes);}}
}
