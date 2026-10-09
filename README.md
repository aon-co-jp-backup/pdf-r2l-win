# pdf-r2l-win

PDFの綴じ方向(右綴じ R2L / 左綴じ L2R)を変更する、インストーラー付き Windows 用 GUI アプリです。
Change the binding direction (R2L / L2R) of PDFs — a Windows GUI app with an installer.

[ottijp/pdf-r2l](https://github.com/ottijp/pdf-r2l)(Java CLI, AGPL-3.0)を Rust で作り直したものです。
A Rust reimplementation of [ottijp/pdf-r2l](https://github.com/ottijp/pdf-r2l).

## 使い方 / Usage
`pdf-r2l-win-setup.exe`(Releases)でインストール後、PDFを選択またはドラッグ&ドロップし、
右綴じ/左綴じを選んで「変換して保存…」。ライセンスはインストーラーの途中と、アプリの
「ライセンス / License」ボタンで英語・日本語併記で表示されます。

## 開発 / Build
```
cargo test
cargo build --release
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\pdf-r2l-win.iss   # dist\pdf-r2l-win-setup.exe
```
依存を変えたら `scripts/gen-third-party.ps1 > licenses/third-party.txt` の後に
`sh scripts/build-licenses.sh` で `LICENSES.txt` を再生成してください。

## LICENSE
AGPL-3.0-only(元リポジトリを継承)。詳細・第三者ライセンスは [LICENSES.txt](LICENSES.txt)。
