<p align="center">
  <img src="../site/public/icon.png" width="112" alt="QuiTwin">
</p>

<h1 align="center">QuiTwin</h1>

<p align="center">
  让 Equicord 不受 Discord 更新影响。
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
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest"><img alt="最新版本" src="https://img.shields.io/github/v/release/AriesAlex/QuiTwin?style=flat-square"></a>
  <a href="../LICENSE"><img alt="MIT 许可证" src="https://img.shields.io/github/license/AriesAlex/QuiTwin?style=flat-square"></a>
</p>

<p align="center">
  <a href="https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe">
    <img alt="下载 QuiTwin.exe" src="https://img.shields.io/badge/DOWNLOAD-QuiTwin.exe-5865F2?style=for-the-badge">
  </a>
</p>

<p align="center"><strong>下载。运行一次。完成。</strong></p>

QuiTwin 会寻找 Discord，如果未安装则先完成安装。随后安装 Equicord，并将 Discord 启动器替换为不受更新影响的孪生启动器。Discord 仍通过原生机制更新，而 Equicord 在后续更新中也会保留。整个过程不会添加后台服务、计划任务或常驻监控进程。

设置成功后，下载的安装程序会自行删除并启动 Discord。

> QuiTwin 是独立项目，与 Discord、Vencord 或 Equicord 均无关联。修改客户端可能违反 Discord 的服务条款，使用风险请自行承担。

## 卸载

照常在 Windows 设置中卸载 Discord。QuiTwin 会先恢复原版更新程序，再将控制权交给 Discord 卸载程序。

## 构建

```powershell
cargo test --all-targets --locked
cargo build --release --locked
```

生成的二进制文件位于 `target/release/quitwin.exe`。
