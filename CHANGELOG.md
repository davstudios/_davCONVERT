# Changelog

## 26.10.1

Prima release di ottobre 2026 secondo lo schema `_davstudios` `YY.M.REVISIONE`.

- Versione sincronizzata a `26.10.1` in npm, Tauri, Cargo, lockfile, launcher, interfaccia, test e documentazione.
- Aggiornato il tag pubblico di riferimento a `v26.10.1`.
- Mantenuti invariati metadata del pacchetto, licenza MIT, identifier storico e automazione bilingue delle GitHub Release.
- Nessuna modifica al motore di conversione, ai formati supportati, all'interfaccia o alla logica applicativa.

## 26.9.1

Standardizzazione della release `_davstudios`, adozione dello schema di versioning `YY.M.REVISIONE` e automazione bilingue delle GitHub Release.

- Versione sincronizzata a `26.9.1` in npm, Tauri, Cargo, lockfile, launcher e interfaccia.
- Aggiunti publisher `_davstudios`, homepage ufficiale, copyright, licenza MIT e file di licenza nei metadata del bundle.
- Aggiunti i metadata Debian per la distribuzione Linux.
- Mantenuto invariato l'identifier storico `studio.dav.convert` per preservare l'identità dell'app.
- Aggiornato il README con istruzioni per installare release non firmate su Windows, macOS e Linux.
- Il workflow GitHub Actions riutilizza automaticamente la Description bilingue del commit associato al tag come descrizione della GitHub Release.
- Aggiunta una verifica che richiede entrambe le sezioni `🇮🇹` e `🇺🇸` prima della pubblicazione.
- Rafforzato il workflow Linux contro repository Microsoft non raggiungibili sui runner Ubuntu.
- Nessuna modifica al motore di conversione, ai formati supportati o alla logica applicativa.

## 1.0.0
- Prima release stabile di _davCONVERT.
- Nuova icona definitiva dell’app e set completo di icone Tauri rigenerato.
- Conversione batch locale PNG, JPG/JPEG, WEBP, BMP, TIFF/TIF e ICO.
- Drag & drop, selezione file, qualità JPEG, destinazione personalizzata e gestione dei file omonimi.
- Coda con stato e risultati per file.
- Design system, tema, lingua e motion coerenti con la suite _davstudios.
- Workflow GitHub configurata per creare release stabili da tag.

## 0.1.1
- Migliorata l'icona di conversione con una doppia freccia vettoriale più pulita, simmetrica e coerente con il design system _davstudios.
- Nessuna modifica al motore di conversione o al layout.

## 0.1.0
- Prima preview di _davCONVERT.
- Conversione batch locale PNG, JPG, WEBP, BMP, TIFF e ICO.
- Drag & drop e selettore file nativo.
- Qualità JPEG configurabile.
- Destinazione automatica o cartella personalizzata.
- Coda risultati con stato per file.
- Design system, tema, lingua e motion coerenti con _davSPACE e _davCLIPBOARD.
