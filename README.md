# pdf-r2l

PDFの綴じ方向(右綴じ R2L / 左綴じ L2R)を変更する Windows GUI アプリです。
[ottijp/pdf-r2l](https://github.com/ottijp/pdf-r2l)(Java CLI)を Rust で作り直したものです。

## 使い方
`pdf-r2l-setup.exe` でインストール後、PDFを選択(またはドラッグ&ドロップ)し、
右綴じ/左綴じを選んで「変換して保存…」を押します。

## 開発
```
cargo test
cargo build --release
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\pdf-r2l.iss   # dist\pdf-r2l-setup.exe
```
PDF処理は[lopdf](https://crates.io/crates/lopdf)、GUIは[egui](https://github.com/emilk/egui)。

## LICENSE
AGPL v3(元リポジトリを継承)
