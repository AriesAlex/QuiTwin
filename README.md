<p align="center">
  <img src="site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Keep Equicord installed through Discord updates.
</p>

<p align="center">
  <a href="README.md">EN</a> ·
  <a href="docs/README.ru.md">RU</a> ·
  <a href="docs/README.sr.md">SR</a> ·
  <a href="docs/README.pl.md">PL</a> ·
  <a href="docs/README.tr.md">TR</a> ·
  <a href="docs/README.fr.md">FR</a> ·
  <a href="docs/README.ar.md">AR</a> ·
  <a href="docs/README.zh.md">ZH</a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/AriesAlex/QuiTwin/ci.yml?branch=main&style=flat-square&label=CI"></a>
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="LICENSE"><img alt="MIT license" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="Download QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>Download. Run once. Done.</strong></p>

QuiTwin finds Discord or installs it when missing, installs Equicord, and replaces Discord's launcher with an update-safe twin. Discord keeps using its native updater, while Equicord survives the next update. No service, scheduled task, or resident watcher is added.

After a successful setup, the downloaded installer removes itself and starts Discord.

> QuiTwin is an independent project and is not affiliated with Discord, Vencord, or Equicord. Client modifications may violate Discord's terms of service. Use it at your own risk.

## Uninstall

Uninstall Discord from Windows Settings as usual. QuiTwin restores the stock updater before handing control to Discord's uninstaller.

## Build

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

The binary is written to `target/release/quitwin.exe`.
