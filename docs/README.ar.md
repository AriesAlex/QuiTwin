<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center" dir="rtl">
  حافظ على تثبيت Equicord بعد تحديثات Discord.
</p>

<p align="center">
  <a href="../README.md">EN</a> ·
  <a href="README.ru.md">RU</a> ·
  <a href="README.sr.md">SR</a> ·
  <a href="README.pl.md">PL</a> ·
  <a href="README.tr.md">TR</a> ·
  <a href="README.fr.md">FR</a> ·
  <strong>AR</strong> ·
  <a href="README.zh.md">ZH</a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/AriesAlex/QuiTwin/ci.yml?branch=main&style=flat-square&label=CI"></a>
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="أحدث إصدار" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="ترخيص MIT" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="تنزيل QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center" dir="rtl"><strong>نزّل. شغّل مرة واحدة. تم.</strong></p>

يعثر QuiTwin على Discord أو يثبّته إن لم يكن موجودا، ثم يثبّت Equicord ويستبدل مشغّل Discord بتوأم يحافظ على Equicord بعد التحديثات. يواصل Discord استخدام أداة التحديث الأصلية، فيما يظل Equicord مثبّتا بعد التحديث التالي. ولا يضيف QuiTwin خدمة أو مهمة مجدولة أو عملية مراقبة مقيمة.

بعد اكتمال الإعداد بنجاح، يحذف المثبّت الذي نزّلته نفسه ويشغّل Discord.

> هذا مشروع مستقل، ولا يتبع QuiTwin أيا من Discord أو Vencord أو Equicord. قد تخالف تعديلات العميل شروط خدمة Discord. استخدمه على مسؤوليتك.

## إزالة التثبيت

أزل Discord بالطريقة المعتادة من إعدادات Windows. يعيد QuiTwin أداة التحديث الأصلية قبل تسليم التحكم إلى برنامج إزالة تثبيت Discord.

## البناء

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

يكتب الملف التنفيذي في `target/release/quitwin.exe`.
