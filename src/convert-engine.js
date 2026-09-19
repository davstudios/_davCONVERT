const supportedInputs=new Set(['png','jpg','jpeg','webp','bmp','tif','tiff','ico']);
const supportedOutputs=['png','jpg','webp','bmp','tiff','ico'];

export function normalizeExtension(value){return String(value||'').trim().toLowerCase().replace(/^\./,'');}
export function isSupportedInput(extension){return supportedInputs.has(normalizeExtension(extension));}
export function outputFormats(){return [...supportedOutputs];}
export function formatBytes(bytes){const value=Number(bytes)||0;if(value<1024)return `${value} B`;const units=['KB','MB','GB','TB'];let size=value/1024;let index=0;while(size>=1024&&index<units.length-1){size/=1024;index+=1;}return `${size>=100?size.toFixed(0):size>=10?size.toFixed(1):size.toFixed(2)} ${units[index]}`;}
export function uniqueFiles(existing,incoming){const map=new Map(existing.map((item)=>[item.path,item]));for(const item of incoming){if(item&&item.path&&!map.has(item.path))map.set(item.path,item);}return [...map.values()];}
export function conversionStats(items){const total=items.length;const done=items.filter((item)=>item.status==='done').length;const failed=items.filter((item)=>item.status==='error').length;const pending=total-done-failed;const sourceBytes=items.reduce((sum,item)=>sum+(Number(item.size)||0),0);const outputBytes=items.reduce((sum,item)=>sum+(Number(item.outputSize)||0),0);return{total,done,failed,pending,sourceBytes,outputBytes};}
export function displayFormat(format){const value=normalizeExtension(format);if(value==='jpg')return 'JPG';if(value==='tiff')return 'TIFF';return value.toUpperCase();}
