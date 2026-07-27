<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Сохраняет Equicord после обновлений Discord.
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
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Последний релиз" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="Лицензия MIT" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="Скачать QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>Скачайте. Запустите один раз. Готово.</strong></p>

QuiTwin находит Discord или устанавливает его, если нужно, добавляет Equicord и заменяет лаунчер Discord двойником, которому не мешают обновления. Discord продолжает обновляться своим штатным способом, а Equicord остаётся на месте. Никаких фоновых служб, задач в планировщике и постоянно работающих наблюдателей.

После успешной настройки скачанный установщик удаляет себя и запускает Discord.

> QuiTwin является независимым проектом и не связан с Discord, Vencord или Equicord. Модификации клиента могут нарушать условия использования Discord. Используйте их на свой риск.

## Удаление

Удалите Discord обычным способом в параметрах Windows. Перед запуском деинсталлятора Discord QuiTwin вернёт штатный апдейтер.

## Сборка

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

Бинарный файл появится в `target/release/quitwin.exe`.
