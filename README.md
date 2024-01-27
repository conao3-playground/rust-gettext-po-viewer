# rust-gettext-po-viewer

## Pre-requires

- gettext
  - include/gettext-po.h
  - lib/libgettextlib.a

## Usage

```bash
$ cargo clean && cargo run sample/magit-section.ja.po
   Compiling libc v0.2.152
   Compiling cc v1.0.83
   Compiling gettext-po-viewer v0.1.0 (/home/conao/dev/tmp/git/rust-gettext-po-viewer)
    Finished dev [unoptimized + debuginfo] target(s) in 1.87s
     Running `target/debug/gettext-po-viewer sample/magit-section.ja.po`
Hello, world!
4 * 2 = 8
["target/debug/gettext-po-viewer", "sample/magit-section.ja.po"]
0x555e6f933c80
0x555e6f96f3f0
0x555e6f941b30
msgid: ""
msgstr: "Project-Id-Version: PACKAGE VERSION\nPOT-Creation-Date: 2024-01-27 17:27+0900\nPO-Revision-Date: 2024-01-27 16:20+0900\nLast-Translator: Automatically generated\nLanguage-Team: none\nLanguage: ja\nMIME-Version: 1.0\nContent-Type: text/plain; charset=UTF-8\nContent-Transfer-Encoding: 8bit\nPlural-Forms: nplurals=1; plural=0;\n"
```
