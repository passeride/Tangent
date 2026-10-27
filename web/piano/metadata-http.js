// HTTP capability only. Provider queries, matching and review remain in shared Rust.
export function allowedMetadataUrl(url,cover){return cover?/^https:\/\/coverartarchive\.org\/release\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\/front-250$/i.test(url):typeof url==='string'&&url.startsWith('https://musicbrainz.org/ws/2/recording/?query=')&&url.length<4000&&!/[\x00-\x1f]/.test(url);}
export class MetadataHttp {
 constructor(fetcher=(...args)=>globalThis.fetch(...args),now=()=>Date.now(),wait=ms=>new Promise(r=>setTimeout(r,ms))){this.fetcher=fetcher;this.now=now;this.wait=wait;this.tail=Promise.resolve();this.next=0;this.cache=new Map();}
 request(request){const job=this.tail.then(()=>this.run(request));this.tail=job.catch(()=>{});return job;}
 async run({url,kind}){
  const cover=kind==='cover';if(!['cover','search'].includes(kind)||!allowedMetadataUrl(url,cover))throw Error('Unsupported metadata request.');
  const prior=this.cache.get(url);if(prior&&this.now()-prior.at<600000)return prior.value;
  if(!cover){await this.wait(Math.max(0,this.next-this.now()));this.next=this.now()+1100;}
  const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),20000);
  try{
   const response=await this.fetcher(url,{signal:controller.signal,credentials:'omit',referrerPolicy:'origin',headers:{Accept:cover?'image/jpeg,image/png':'application/json'}});
   if(!response.ok)throw Error(response.status===404?'No front cover is available for this release.':`Provider unavailable (HTTP ${response.status}). Try again later.`);
   const limit=1024*1024;if(Number(response.headers.get('Content-Length'))>limit)throw Error('Provider response is too large.');
   const reader=response.body.getReader();const chunks=[];let total=0;
   while(true){const {done,value}=await reader.read();if(done)break;total+=value.length;if(total>limit){await reader.cancel();throw Error('Provider response is too large.');}chunks.push(value);}
   const bytes=new Uint8Array(total);let offset=0;for(const c of chunks){bytes.set(c,offset);offset+=c.length;}
   const value=cover?{bytes}:{body:new TextDecoder().decode(bytes)};
   if(!cover){try{JSON.parse(value.body);}catch{throw Error("Provider returned unreadable data. Try again later.");}}
   if(this.cache.size>=32)this.cache.delete(this.cache.keys().next().value);this.cache.set(url,{at:this.now(),value});return value;
  }catch(error){if(error.name==='AbortError')throw Error('Provider timed out. Try again later.');throw error;}finally{clearTimeout(timer);}
 }
}
