import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const packageVersion=JSON.parse(fs.readFileSync('package.json','utf8')).version;
const tauriVersion=JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json','utf8')).version;
const cargo=fs.readFileSync('src-tauri/Cargo.toml','utf8');
const cargoVersion=cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
test('versioni tecniche sincronizzate',()=>{assert.equal(packageVersion,'26.10.1');assert.equal(tauriVersion,packageVersion);assert.equal(cargoVersion,packageVersion);});
