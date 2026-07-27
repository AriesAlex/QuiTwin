<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Equicord ostaje instaliran i posle Discord ažuriranja.
</p>

<p align="center">
  <a href="../README.md">EN</a> ·
  <a href="README.ru.md">RU</a> ·
  <strong>SR</strong> ·
  <a href="README.pl.md">PL</a> ·
  <a href="README.tr.md">TR</a> ·
  <a href="README.fr.md">FR</a> ·
  <a href="README.ar.md">AR</a> ·
  <a href="README.zh.md">ZH</a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/AriesAlex/QuiTwin/ci.yml?branch=main&style=flat-square&label=CI"></a>
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Najnovije izdanje" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="MIT licenca" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="Preuzmi QuiTwin.exe" src="https://img.shields.io/badge/PREUZMI-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>Preuzmi. Pokreni jednom. Gotovo.</strong></p>

QuiTwin pronalazi Discord ili ga instalira ako nedostaje, instalira Equicord i zamenjuje Discordov pokretač blizancem koji preživljava ažuriranja. Discord nastavlja da koristi svoj sistem za ažuriranje, a Equicord ostaje aktivan i posle narednog ažuriranja. Pritom ne dodaje servis, zakazani zadatak niti stalni pozadinski proces.

Posle uspešne instalacije, preuzeti instalater se sam briše i pokreće Discord.

> QuiTwin je nezavisan projekat i nije povezan sa Discordom, Vencordom ili Equicordom. Izmene klijenta mogu da krše Discordove uslove korišćenja. Koristi ih na sopstveni rizik.

## Deinstalacija

Deinstaliraj Discord kao i obično, kroz podešavanja sistema Windows. Pre nego što pokrene Discordov program za deinstalaciju, QuiTwin vraća originalni program za ažuriranje.

## Izgradnja

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

Izvršna datoteka se nalazi u `target/release/quitwin.exe`.
