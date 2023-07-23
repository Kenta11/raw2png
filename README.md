# raw2png

RAW 画像を PNG 画像に変換するツールです．
FPGAプログラミング大全 Xilinx編 第2版の第3章と第7章で生成する RAW 画像の内容を確認できます．

## 使い方

```
$ ls
Cargo.lock  Cargo.toml  LICENSE  README.md  src  target
$ cargo run -- path/to/imagedata.raw
$ ls
Cargo.lock  Cargo.toml  LICENSE  README.md  imagedata.png  src  target
```

## 注意

- 入力画像のサイズは 640x480 であると想定されます
    - 別のサイズを扱いたい場合はソースコードを編集してください
- 出力画像のファイル名は imagedata.png です

## リンク

- [FPGAプログラミング大全 Xilinx編 第2版](https://www.shuwasystem.co.jp/book/9784798063263.html)

## ライセンス

- [The MIT LICENSE](LICENSE)
