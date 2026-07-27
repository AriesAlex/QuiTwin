<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Gardez Equicord installé malgré les mises à jour de Discord.
</p>

<p align="center">
  <a href="../README.md">EN</a> ·
  <a href="README.ru.md">RU</a> ·
  <a href="README.sr.md">SR</a> ·
  <a href="README.pl.md">PL</a> ·
  <a href="README.tr.md">TR</a> ·
  <a href="README.fr.md">FR</a> ·
  <a href="README.ar.md">AR</a> ·
  <a href="README.zh.md">ZH</a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/AriesAlex/QuiTwin/ci.yml?branch=main&style=flat-square&label=CI"></a>
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Dernière version" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="Licence MIT" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="Télécharger QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>Téléchargez. Lancez une fois. C’est tout.</strong></p>

QuiTwin détecte Discord ou l’installe s’il est absent, installe Equicord et remplace le lanceur de Discord par un jumeau conçu pour résister aux mises à jour. Discord continue d’utiliser son mécanisme de mise à jour natif et Equicord reste en place après la mise à jour suivante. QuiTwin n’ajoute ni service, ni tâche planifiée, ni processus de surveillance permanent.

Une fois la configuration terminée, l’installateur téléchargé se supprime et lance Discord.

> QuiTwin est un projet indépendant, sans affiliation avec Discord, Vencord ou Equicord. Les modifications du client peuvent enfreindre les conditions d’utilisation de Discord. Utilisez-les à vos propres risques.

## Désinstallation

Désinstallez Discord normalement depuis les Paramètres Windows. QuiTwin rétablit le programme de mise à jour d’origine avant de transmettre la main au programme de désinstallation de Discord.

## Compilation

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

Le binaire est généré dans `target/release/quitwin.exe`.
