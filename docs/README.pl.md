<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Equicord nie znika po aktualizacjach Discorda.
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
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Najnowsze wydanie" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="Licencja MIT" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="Pobierz QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>Pobierz. Uruchom raz. Gotowe.</strong></p>

QuiTwin znajduje Discorda lub instaluje go, jeśli go brakuje, dodaje Equicord i zastępuje program startowy Discorda odpowiednikiem odpornym na aktualizacje. Wbudowany mechanizm aktualizacji Discorda nadal działa bez zmian, a Equicord nie znika po kolejnej aktualizacji. QuiTwin nie dodaje usługi, zaplanowanego zadania ani stale działającego procesu.

Po udanej konfiguracji pobrany instalator usuwa się i uruchamia Discorda.

> QuiTwin jest niezależnym projektem i nie jest powiązany z Discordem, Vencordem ani Equicordem. Modyfikacje klienta mogą naruszać warunki korzystania z Discorda. Używasz ich na własne ryzyko.

## Odinstalowanie

Odinstaluj Discorda jak zwykle w Ustawieniach systemu Windows. QuiTwin przywraca oryginalny mechanizm aktualizacji, zanim przekaże sterowanie deinstalatorowi Discorda.

## Budowanie

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

Plik wykonywalny zostanie zapisany w `target/release/quitwin.exe`.
