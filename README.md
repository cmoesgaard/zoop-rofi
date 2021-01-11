# zoop-rofi 👉😎👉

Have you ever found yourself in a situation where you _really_ want to respond to a message with a sequence of dumb emojis, 
but by the time you finally type it out, the moment has passed and you come off as some kind of out-of-touch weirdo? 😔

Fret not.

![](https://i.imgur.com/QoPr4ih.png)

A [`rofi`](https://github.com/davatorium/rofi) based tool to easily allow you to paste your favorite emoji sequences.

Search for what you need from a user-defined list. The selection is automatically copied to the clipboard. 🪄

Easily configurable using Lispy S-expressions. 🤩

```lisp
(((name . "👉😎👉") (aliases "zoop" "fingerguns")))
```

The tool automatically generates a template config on startup in `$XDG_CONFIG_HOME/zoop-rofi/`


## Why?

This tool serves several purposes:

* A way for me to learn Rust 🦀
* A way to allow a 30-something millennial to cling to youth juuust a bit longer, by providing him easy access to Cool Emojis (tm) 😎

## System dependencies

* [`rofi`](https://github.com/davatorium/rofi)
* `xclip`/`xsel`

## TODO

* Clean up implementation
* Build binaries
