// Directory permissions/iteration only. Parsing and source validation stay in Rust.
export const folderMigration='CREATE TABLE library_folder(id INTEGER PRIMARY KEY CHECK(id=1),path TEXT NOT NULL); PRAGMA user_version=7;';
const music=name=>/\.(mid|midi|xml|musicxml|mxl)$/i.test(name);
export async function directoryFiles(root){
 const files=[];let visited=0;
 async function walk(dir,depth){for await(const child of dir.values()){if(++visited>10000)throw Error('Folder exceeds the 10,000-entry scan limit.');if(child.kind==='directory'){if(depth>=32)throw Error('Folder nesting exceeds 32 levels.');await walk(child,depth+1);}else if(child.kind==='file'&&music(child.name))files.push(await child.getFile());}}
 await walk(root,0);return files;
}
export class FolderLibrary {
 constructor(storage,ingest,notify){this.storage=storage;this.ingest=ingest;this.notify=notify;this.handle=null;this.busy=false;}
 report(path,auto){this.notify({type:'library-folder',path,auto});}
 async init(){this.handle=await this.storage.readDirectoryHandle();const path=await this.storage.readFolderName();this.report(path,!!this.handle);if(this.handle){if(await this.permission(false))await this.scan(false);else this.notify({type:'library-scan',busy:false,text:'Press Scan now to grant folder read access.'});}}
 async permission(interactive){const h=this.handle;if(!h)return false;let p=h.queryPermission?await h.queryPermission({mode:'read'}):'granted';if(p!=='granted'&&interactive&&h.requestPermission)p=await h.requestPermission({mode:'read'});return p==='granted';}
 async choose(){if(this.busy)return;if(typeof window.showDirectoryPicker!=='function')return this.upload();try{const handle=await window.showDirectoryPicker({mode:'read',id:'tangent-library',startIn:'music'});await this.storage.saveDirectoryHandle(handle);await this.storage.saveFolderName(handle.name);this.handle=handle;this.report(handle.name,true);await this.scan(true);}catch(e){if(e.name!=='AbortError')this.notify({type:'library-scan',busy:false,text:String(e.message??e)});}}
 async scan(interactive=true){if(this.busy)return;if(!this.handle){if(interactive)return this.upload();return;}this.busy=true;try{if(!await this.permission(interactive))throw Error('Folder permission is required. Choose the folder again or grant read access.');this.notify({type:'library-scan',busy:true,text:'Scanning folder and subfolders…'});await this.ingest(await directoryFiles(this.handle),true);}catch(e){this.notify({type:'library-scan',busy:false,text:String(e.message??e)});}finally{this.busy=false;}}
 upload(){const input=document.createElement('input');input.type='file';input.multiple=true;input.webkitdirectory=true;input.onchange=async()=>{const files=[...input.files].filter(f=>music(f.name));if(!input.files.length)return;const path=input.files[0].webkitRelativePath.split('/')[0];await this.storage.saveDirectoryHandle(null);await this.storage.saveFolderName(path);this.handle=null;this.report(path,false);await this.ingest(files,true);};input.click();}
 async clear(){if(this.busy)return;await this.storage.saveDirectoryHandle(null);await this.storage.saveFolderName('');this.handle=null;this.report('',false);}
}
