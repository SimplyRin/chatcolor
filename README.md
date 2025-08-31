# chatcolor

`chatcolor` は Minecraft のチャットカラーコード（§コードおよび &コード）を Bash などのターミナル上で色付き・装飾付きの文字列を表示できる Rust 製 CLI ツールです。

## 特徴

- 従来の Minecraft 16色カラーコード (`§a`, `§c` など) に対応
- フォーマットコード（太字、下線、斜体、取り消し線など）に対応
- Minecraft 1.16 以降の RGB / Hex カラーコード (`§x§R§R§G§G§B§B`) に対応

---

## インストール

### Rust がインストールされている場合

```sh
git clone https://github.com/SimplyRin/chatcolor.git
cd chatcolor
cargo build --release
```
