# Build notes — _davCONVERT v26.10.1

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

La v26.10.1 mantiene invariato il motore di conversione locale e tutti i comportamenti della precedente release stabile. Questa release inaugura il ciclo di ottobre secondo il versioning `_davstudios` `YY.M.REVISIONE`; metadata, automazione bilingue delle GitHub Release e configurazione multipiattaforma restano quelli già validati nella v26.9.1.

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
