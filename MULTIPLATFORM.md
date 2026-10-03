# Supporto multipiattaforma

`_davCONVERT v26.10.2` usa Tauri 2 e Rust. La conversione immagini avviene localmente nel backend Rust tramite il crate `image`; il selettore file usa il plugin dialog ufficiale Tauri.

## Windows

Usa `RUN-WINDOWS.bat` per lo sviluppo e `BUILD-WINDOWS.bat` per il bundle. Le release GitHub non sono attualmente firmate con un certificato Authenticode commerciale; SmartScreen può quindi mostrare un avviso.

## macOS

Usa `RUN-MACOS.sh` e `BUILD-MACOS.sh`. Le release GitHub non sono attualmente firmate con Developer ID né notarizzate da Apple; Gatekeeper può quindi richiedere l'apertura manuale da Privacy e Sicurezza.

## Linux

Su Ubuntu/Debian esegui prima `INSTALL-LINUX-DEPS-UBUNTU.sh`, poi `RUN-LINUX.sh` o `BUILD-LINUX.sh`. Il workflow GitHub disabilita eventuali sorgenti Microsoft non raggiungibili prima di installare le dipendenze Tauri.

## Privacy e rete

I file vengono convertiti sul dispositivo. Il backend di conversione non usa upload o richieste HTTP per elaborare i file.

