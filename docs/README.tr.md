<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  Discord güncellense de Equicord kurulu kalsın.
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
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="Son sürüm" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="MIT lisansı" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="QuiTwin.exe indir" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>İndir. Bir kez çalıştır. Bitti.</strong></p>

QuiTwin Discord'u bulur, yoksa kurar; Equicord'u yükler ve Discord başlatıcısını güncellemelere dayanıklı ikiziyle değiştirir. Discord kendi güncelleyicisini kullanmayı sürdürürken Equicord bir sonraki güncellemeden sonra da çalışır. QuiTwin arka plan servisi, zamanlanmış görev veya sürekli çalışan bir izleme süreci eklemez.

Kurulum başarıyla tamamlandığında indirilen kurucu kendini siler ve Discord'u başlatır.

> QuiTwin bağımsız bir projedir; Discord, Vencord veya Equicord ile bağlantılı değildir. İstemciyi değiştirmek Discord'un kullanım koşullarını ihlal edebilir. Kullanım riski size aittir.

## Kaldırma

Discord'u Windows Ayarları'ndan her zamanki gibi kaldırın. QuiTwin, denetimi Discord'un kaldırma programına devretmeden önce Discord'un standart güncelleyicisini geri yükler.

## Derleme

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

Derlenen ikili `target/release/quitwin.exe` konumuna yazılır.
