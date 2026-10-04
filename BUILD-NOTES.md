# Build notes — _davCONVERT v26.10.3

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

La v26.10.3 mantiene invariato il motore di conversione locale e introduce il final polish condiviso della suite `_davstudios`: motion system derivato dal sito v52, versione rimossa dalla UI ordinaria, supporto Buy Me A Coffee aggiornato, README stabile e build Windows Release configurata come applicazione GUI.

## Windows

La build Release usa `windows_subsystem = "windows"`, quindi l'eseguibile finale non apre una console CMD separata. L'unico processo figlio Windows usato da `_davCONVERT`, `explorer.exe` per mostrare la cartella di un risultato, viene avviato con `CREATE_NO_WINDOW`.

## Motion

Le transizioni principali usano gli stessi riferimenti del sito `_davstudios` v52: reveal base da 720 ms, slow da 940 ms, stagger da 72 ms, page-out da 170 ms e page-in da 430 ms. Il cambio tema usa il reveal radiale da 680 ms e rispetta `prefers-reduced-motion`.

## Metadata bundle

- Publisher: `_davstudios`
- Homepage: `https://davstudios.it`
- License: `MIT`
- Copyright: `© 2026 _davstudios`
- Identifier preservato: `studio.dav.convert`
- Categoria: `Utility`

## Firma

Le release attuali non usano certificati commerciali di firma Windows né Developer ID/notarizzazione Apple. Il README contiene le istruzioni per SmartScreen, Gatekeeper e AppImage.

## Icone bundle

Il contenuto grafico del set di icone Tauri viene preservato pixel-per-pixel; i contenitori/metadata possono essere aggiornati durante la repository normalization della release.
