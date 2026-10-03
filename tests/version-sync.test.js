import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import { resolve } from 'node:path';

const root=resolve(import.meta.dirname,'..');
const packageJson=JSON.parse(fs.readFileSync(resolve(root,'package.json'),'utf8'));
const packageLock=JSON.parse(fs.readFileSync(resolve(root,'package-lock.json'),'utf8'));
const tauri=JSON.parse(fs.readFileSync(resolve(root,'src-tauri/tauri.conf.json'),'utf8'));
const cargo=fs.readFileSync(resolve(root,'src-tauri/Cargo.toml'),'utf8');
const cargoLock=fs.readFileSync(resolve(root,'src-tauri/Cargo.lock'),'utf8');
const cargoVersion=cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
const cargoLockVersion=cargoLock.match(/\[\[package\]\]\r?\nname = "davconvert"\r?\nversion = "([^"]+)"/)?.[1];
const mainSource=fs.readFileSync(resolve(root,'src/main.js'),'utf8');

test('versioni tecniche sincronizzate',()=>{
  assert.equal(packageJson.version,'26.10.2');
  assert.equal(packageLock.version,packageJson.version);
  assert.equal(packageLock.packages[''].version,packageJson.version);
  assert.equal(tauri.version,packageJson.version);
  assert.equal(cargoVersion,packageJson.version);
  assert.equal(cargoLockVersion,packageJson.version);
});

test('Cargo.lock version parser supports Windows CRLF checkouts',()=>{
  const windowsLock=cargoLock.replace(/\r?\n/g,'\r\n');
  const parsed=windowsLock.match(/\[\[package\]\]\r?\nname = "davconvert"\r?\nversion = "([^"]+)"/)?.[1];
  assert.equal(parsed,packageJson.version);
});

test('interfaccia legge versione da Tauri senza fallback di release hardcoded',()=>{
  assert.match(mainSource,/getVersion/);
  assert.doesNotMatch(mainSource,/version:'26\.10\.2'/);
});

