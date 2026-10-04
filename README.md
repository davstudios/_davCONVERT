<div align="center">
  <img src="src-tauri/icons/app-icon.png" width="112" alt="_davCONVERT icon">

# _davCONVERT

**Converti immagini in batch, in locale e senza upload.**  
**Convert images in batches locally, with no uploads.**

Windows · macOS · Linux · Local-first · Open source

[![Italiano](https://img.shields.io/badge/Italiano-006EDB?style=for-the-badge)](#-italiano)
[![English](https://img.shields.io/badge/English-141416?style=for-the-badge)](#-english)
</div>

---

# 🇮🇹 Italiano

_davCONVERT è un'app desktop multipiattaforma di **_davstudios** per convertire immagini statiche in batch direttamente sul computer. Aggiungi i file tramite selettore nativo o drag & drop, scegli formato, qualità e destinazione e controlla lo stato di ogni elemento senza inviare i contenuti online.

<p>
  <a href="https://www.davstudios.it"><img src=".github/assets/website-it.svg" height="46" alt="Visita il sito"></a>
  <a href="https://buymeacoffee.com/davstudios"><img src=".github/assets/buy-coffee-it.svg" height="46" alt="Offrimi Un Caffè"></a>
</p>

## Funzioni principali

- conversione batch completamente locale;
- drag & drop e selezione file nativa;
- input/output PNG, JPG/JPEG, WEBP, BMP, TIFF/TIF e ICO;
- qualità JPEG regolabile;
- cartella di destinazione automatica o personalizzata;
- gestione sicura dei file omonimi senza sovrascritture accidentali;
- opzione di sovrascrittura esplicita quando desiderata;
- coda con stato, errori, dimensione sorgente e dimensione generata;
- apertura rapida della cartella contenente il risultato;
- interfaccia italiana e inglese;
- tema Sistema, Chiaro e Scuro;
- motion system coerente con il sito `_davstudios`.

## Privacy e local-first

- nessun account;
- nessun upload delle immagini;
- nessuna elaborazione cloud;
- nessuna telemetria integrata;
- conversione eseguita dal backend Rust tramite il crate `image`.

I file e i relativi contenuti restano sul dispositivo.

## Formati supportati

| Formato | Input | Output |
| --- | :---: | :---: |
| PNG | ✓ | ✓ |
| JPG / JPEG | ✓ | ✓ |
| WEBP | ✓ | ✓ |
| BMP | ✓ | ✓ |
| TIFF / TIF | ✓ | ✓ |
| ICO | ✓ | ✓ |

La qualità configurabile viene applicata all'output JPEG. Per gli altri formati viene utilizzata la codifica prevista dal backend locale.

## Piattaforme

| Sistema | Architettura | Pacchetto |
| --- | --- | --- |
| Windows 10/11 | x64 | NSIS `.exe` |
| macOS | Intel + Apple Silicon | Universal `.dmg` |
| Linux | x64 | `.AppImage` / `.deb` |

Le release vengono compilate tramite GitHub Actions sui rispettivi sistemi operativi.

## Installazione di release non firmate

Le build pubbliche non utilizzano attualmente un certificato commerciale Windows né Apple Developer ID/notarizzazione. Scarica sempre gli artefatti dalla repository GitHub ufficiale di `_davstudios`.

### Windows

SmartScreen può mostrare **Windows ha protetto il PC**. Se il file proviene dalla repository ufficiale, scegli **Ulteriori informazioni → Esegui comunque**. La build Release è configurata come applicazione GUI e non apre una finestra CMD separata.

### macOS

Se Gatekeeper blocca la prima apertura, prova ad aprire l'app e poi vai in **Impostazioni di Sistema → Privacy e Sicurezza → Apri comunque**.

### Linux

Per un'AppImage può essere necessario renderla eseguibile:

```bash
chmod +x _davCONVERT*.AppImage
```

## Sviluppo

Requisiti: Node.js, Rust e prerequisiti Tauri del sistema operativo.

```bash
npm install
npm run desktop
```

Test:

```bash
npm test
```

Build locale:

```bash
npm run bundle
```

Gli artefatti vengono generati in `src-tauri/target/release/bundle/`.

## Stack e identità

- Tauri 2;
- Rust;
- JavaScript + Vite;
- crate Rust `image` per la conversione;
- Plus Jakarta Sans con fallback di sistema;
- motion system coerente con il sito `_davstudios`;
- bundle identifier stabile: `studio.dav.convert`;
- licenza MIT.

La versione dell'app è gestita nei manifest tecnici e nelle GitHub Release; non viene mostrata nell'interfaccia ordinaria per mantenere la UI pulita e impedire stringhe di versione duplicate.

## Licenza

Distribuito con licenza **MIT**. Consulta [`LICENSE`](LICENSE).

---

# 🇺🇸 English

_davCONVERT is a cross-platform desktop app by **_davstudios** for converting static images in batches directly on your computer. Add files with the native picker or drag & drop, choose the format, quality and destination, and track every item without sending content online.

<p>
  <a href="https://www.davstudios.it/en"><img src=".github/assets/website-en.svg" height="46" alt="Visit website"></a>
  <a href="https://buymeacoffee.com/davstudios"><img src=".github/assets/buy-coffee-en.svg" height="46" alt="Buy Me A Coffee"></a>
</p>

## Main features

- fully local batch conversion;
- drag & drop and native file selection;
- PNG, JPG/JPEG, WEBP, BMP, TIFF/TIF and ICO input/output;
- adjustable JPEG quality;
- automatic or custom destination folder;
- safe handling of same-name files without accidental overwrites;
- explicit overwrite option when desired;
- queue with status, errors, source size and generated size;
- quick opening of the folder containing a result;
- Italian and English interface;
- System, Light and Dark themes;
- motion system aligned with the `_davstudios` website.

## Privacy and local-first

- no account;
- no image uploads;
- no cloud processing;
- no built-in telemetry;
- conversion is performed by the Rust backend through the `image` crate.

Files and their contents remain on your device.

## Supported formats

| Format | Input | Output |
| --- | :---: | :---: |
| PNG | ✓ | ✓ |
| JPG / JPEG | ✓ | ✓ |
| WEBP | ✓ | ✓ |
| BMP | ✓ | ✓ |
| TIFF / TIF | ✓ | ✓ |
| ICO | ✓ | ✓ |

Configurable quality is applied to JPEG output. Other formats use the encoding provided by the local backend.

## Platforms

| System | Architecture | Package |
| --- | --- | --- |
| Windows 10/11 | x64 | NSIS `.exe` |
| macOS | Intel + Apple Silicon | Universal `.dmg` |
| Linux | x64 | `.AppImage` / `.deb` |

Releases are compiled through GitHub Actions on the corresponding operating systems.

## Installing unsigned releases

Public builds currently do not use a commercial Windows signing certificate or Apple Developer ID/notarization. Always download artifacts from the official `_davstudios` GitHub repository.

### Windows

SmartScreen may display **Windows protected your PC**. If the file comes from the official repository, choose **More info → Run anyway**. Release builds use the Windows GUI subsystem and do not open a separate CMD window.

### macOS

If Gatekeeper blocks the first launch, attempt to open the app and then go to **System Settings → Privacy & Security → Open Anyway**.

### Linux

An AppImage may need executable permission:

```bash
chmod +x _davCONVERT*.AppImage
```

## Development

Requirements: Node.js, Rust and the Tauri prerequisites for your operating system.

```bash
npm install
npm run desktop
```

Tests:

```bash
npm test
```

Local build:

```bash
npm run bundle
```

Artifacts are generated under `src-tauri/target/release/bundle/`.

## Stack and identity

- Tauri 2;
- Rust;
- JavaScript + Vite;
- Rust `image` crate for conversion;
- Plus Jakarta Sans with system fallback;
- motion system aligned with the `_davstudios` website;
- stable bundle identifier: `studio.dav.convert`;
- MIT License.

The application version is managed by the technical manifests and GitHub Releases; it is intentionally omitted from the ordinary interface to keep the UI clean and prevent duplicated version strings.

## License

Released under the **MIT License**. See [`LICENSE`](LICENSE).
