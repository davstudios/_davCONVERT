<div align="center">
  <img src="src-tauri/icons/app-icon.png" width="112" alt="_davCONVERT icon">
</div>

# `_davCONVERT`

Convertitore batch locale di immagini per Windows, macOS e Linux.  
Local batch image converter for Windows, macOS and Linux.

**v26.10.2 · Stable · Local-first · No telemetry**

Interfaccia e motion system condivisi con la suite `_davstudios`.

## Italiano

`_davCONVERT` converte immagini statiche in batch interamente in locale, con drag & drop, scelta del formato, qualità JPEG, destinazione personalizzata e stato per ogni file.

### Formati supportati

Input/output: PNG, JPG/JPEG, WEBP, BMP, TIFF/TIF e ICO.

### Funzioni

- Conversione batch locale.
- Drag & drop e selettore file nativo.
- Formato di uscita configurabile.
- Qualità JPEG regolabile.
- Cartella di destinazione automatica o personalizzata.
- Gestione dei file omonimi.
- Coda con stato, errori e risultati per file.
- Tema Sistema / Chiaro / Scuro.
- Italiano e English.
- Nessun upload dei file e nessuna telemetria.
- Design, testi, colori, icone e animazioni coerenti con il design system `_davstudios`.

Audio, video e documenti potranno essere aggiunti in revisioni successive senza modificare la natura local-first dell'app.

### Installazione delle release GitHub non firmate

Le release di `_davCONVERT` sono distribuite direttamente tramite GitHub e, al momento, non utilizzano certificati commerciali di code signing o notarizzazione Apple. Il codice sorgente è disponibile pubblicamente con licenza MIT.

#### Windows

Windows SmartScreen può mostrare l'avviso **“Windows ha protetto il PC”** perché l'installer non è firmato con un certificato di publisher attendibile. Se hai scaricato il file dalla repository GitHub ufficiale di `_davstudios`, seleziona **Ulteriori informazioni** e poi **Esegui comunque**.

#### macOS

Gatekeeper può impedire la prima apertura perché l'app non è firmata con Developer ID e non è notarizzata da Apple. Dopo aver tentato di aprire l'app, vai in **Impostazioni di Sistema → Privacy e Sicurezza**, individua il messaggio relativo a `_davCONVERT` e scegli **Apri comunque**.

#### Linux

Per un'AppImage può essere necessario rendere il file eseguibile prima dell'avvio:

```bash
chmod +x _davCONVERT*.AppImage
```

Scarica sempre le release dalla repository GitHub ufficiale di `_davstudios`. Quando viene pubblicato un hash SHA-256, puoi usarlo per verificare l'integrità del file scaricato.

### Avvio in sviluppo su Windows

`RUN-WINDOWS.bat`

Oppure:

```bash
npm install --no-audit --no-fund
npm run desktop
```

## English

`_davCONVERT` converts static images locally in batch with drag & drop, output-format selection, JPEG quality controls, custom destinations and per-file status.

### Supported formats

Input/output: PNG, JPG/JPEG, WEBP, BMP, TIFF/TIF and ICO.

### Features

- Local batch conversion.
- Drag & drop and native file picker.
- Configurable output format.
- Adjustable JPEG quality.
- Automatic or custom destination folder.
- Same-name file handling.
- Queue with per-file status, errors and results.
- System, Light and Dark themes.
- Italiano and English.
- No file uploads and no telemetry.
- UI and motion aligned with the shared `_davstudios` design system.

### Installing unsigned GitHub releases

`_davCONVERT` releases are distributed directly through GitHub and currently do not use a commercial code-signing certificate or Apple notarization. The source code is publicly available under the MIT License.

#### Windows

Windows SmartScreen may display **“Windows protected your PC”** because the installer is not signed by a trusted publisher certificate. If you downloaded it from the official `_davstudios` GitHub repository, select **More info** and then **Run anyway**.

#### macOS

Gatekeeper may block the first launch because the app is not signed with Developer ID and is not notarized by Apple. After attempting to open it, go to **System Settings → Privacy & Security**, locate the `_davCONVERT` notice and choose **Open Anyway**.

#### Linux

An AppImage may need executable permission before launch:

```bash
chmod +x _davCONVERT*.AppImage
```

Always download releases from the official `_davstudios` GitHub repository. When a SHA-256 hash is published, you can use it to verify the integrity of the downloaded file.

## Package information

- Developer / Publisher: `_davstudios`
- Homepage: https://davstudios.it
- License: MIT
- Bundle identifier: `studio.dav.convert`
- Current version: `26.10.2`

## Support _davstudios

Website: https://davstudios.it  
Buy Me A Coffee: https://buymeacoffee.com/davstudios

## License

MIT — see [`LICENSE`](LICENSE).

