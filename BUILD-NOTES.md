# Build notes — _davCONVERT v26.10.2

## Requisiti

- Node.js 20+
- npm
- Rust/Cargo compatibile con Tauri 2
- dipendenze native Tauri del sistema operativo

## Test

```bash
npm test
```

## Sviluppo desktop

```bash
npm install --no-audit --no-fund
npm run desktop
```

## Bundle

```bash
npm run bundle
```

La v26.10.2 mantiene invariato il motore di conversione locale e applica una repository normalization completa dell’intero pacchetto, sincronizzazione forte dei manifest e dei lockfile, controllo Cargo.lock compatibile LF/CRLF e versione UI letta direttamente da Tauri.

## Metadata bundle

- Publisher: `_davstudios`
- Homepage: `https://davstudios.it`
- License: `MIT`
- Copyright: `© 2026 _davstudios`
- Identifier preservato: `studio.dav.convert`
- Categoria: `Utility`

## Firma

Le release attuali non usano certificati commerciali di firma Windows né Developer ID/notarizzazione Apple. Il README contiene le istruzioni per gli utenti che incontrano SmartScreen o Gatekeeper.

## Icone bundle

Il set di icone Tauri esistente viene preservato senza modifiche.

