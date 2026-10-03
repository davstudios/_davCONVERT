import test from 'node:test';
import assert from 'node:assert/strict';
import { conversionStats,displayFormat,formatBytes,isSupportedInput,normalizeExtension,outputFormats,uniqueFiles } from '../src/convert-engine.js';

test('normalizza estensioni',()=>{assert.equal(normalizeExtension('.JPEG'),'jpeg');});
test('riconosce formati input supportati',()=>{assert.equal(isSupportedInput('PNG'),true);assert.equal(isSupportedInput('svg'),false);});
test('elenca formati output',()=>{assert.deepEqual(outputFormats(),['png','jpg','webp','bmp','tiff','ico']);});
test('formatta dimensioni',()=>{assert.equal(formatBytes(1024),'1.00 KB');assert.equal(formatBytes(1048576),'1.00 MB');});
test('rimuove duplicati per percorso',()=>{const a={path:'a.png'},b={path:'b.png'};assert.deepEqual(uniqueFiles([a],[a,b]).map(x=>x.path),['a.png','b.png']);});
test('calcola statistiche conversione',()=>{const stats=conversionStats([{status:'done',size:10,outputSize:8},{status:'error',size:20},{status:'pending',size:30}]);assert.deepEqual(stats,{total:3,done:1,failed:1,pending:1,sourceBytes:60,outputBytes:8});});
test('mostra nomi formato coerenti',()=>{assert.equal(displayFormat('jpg'),'JPG');assert.equal(displayFormat('tiff'),'TIFF');});

