import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
function files(dir){return fs.readdirSync(dir,{withFileTypes:true}).flatMap((entry)=>{const full=path.join(dir,entry.name);return entry.isDirectory()?files(full):[full];});}
test('sorgenti senza commenti',()=>{for(const file of [...files('src'),...files('src-tauri/src')]){const text=fs.readFileSync(file,'utf8');assert.doesNotMatch(text,/^\s*\/\//m,file);assert.doesNotMatch(text,/\/\*[\s\S]*?\*\//,file);}});
