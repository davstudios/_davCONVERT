# CHANGELOG

## 26.10.3

- Allineato il motion system dell'app al linguaggio visivo del sito `_davstudios` v52, con transizioni di pagina, reveal, stagger, hover ed easing condivisi.
- Aggiunte transizioni reali tra Converti, Coda e Impostazioni e reveal radiale del cambio tema, con supporto `prefers-reduced-motion`.
- Rimossa la versione dall'interfaccia ordinaria; la versione resta gestita nei manifest tecnici e nelle GitHub Release.
- Aggiornata la dicitura italiana Buy Me A Coffee a `Offrimi Un Caffè`.
- Normalizzato il README bilingue e reso indipendente dalla release corrente.
- Configurata la build Windows Release con GUI subsystem per evitare la finestra CMD separata.
- Verificato il processo figlio Windows usato per aprire la cartella di output e configurato senza console.
- Rafforzati i test di contratto per motion v52, UI senza versione, Buy Me A Coffee, Windows GUI subsystem e CRLF.
- Repository normalization completa senza modificare il motore di conversione locale.

# Changelog

## 26.10.2

Normalization completa del repository e rafforzamento dei controlli di release senza modifiche funzionali al motore di conversione.

- Versione sincronizzata a `26.10.2` in npm, Tauri, Cargo, lockfile, launcher, interfaccia, test e documentazione.
- Repository normalization applicata a tutti i file del pacchetto con modifiche reali ma neutre.
- Controllo versione esteso a `package-lock.json` e `Cargo.lock`, con parser compatibile LF/CRLF e regressione dedicata ai checkout Windows.
- Rimossi i riferimenti hardcoded alla versione corrente dalla UI: la versione visualizzata viene letta da Tauri.
- Workflow GitHub Actions rafforzato con verifica completa dei manifest prima della build e della pubblicazione.
- Asset grafici preservati visivamente durante la normalization binaria.
- Nessuna modifica al motore di conversione, ai formati supportati, al backend Rust o al comportamento dell'app.

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

