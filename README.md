# nctok

A program for interactive navigating through weighted hierarchical structured
data and analyzing them. (weighted means that each item have its own value -
number)

This is a nice tool for analyzing and researching datasets, and simplicity of
input format makes it easy to write your own scripts that produces data you
want to analyze fast, right in shell (like something in files, git meta data
(number of commits per each file), etc. For more see
[examples](doc/examples.md)).

## Demo

File space usage:

![](https://github.com/UnkwUsr/nctok/assets/49063932/046a4cea-c37d-4827-bbc5-d523ffb58f43)

Git commits per file, linux kernel 6.8 repo:

![](https://github.com/UnkwUsr/nctok/assets/49063932/c30daaed-c873-4d74-bef1-ba3b714fb9ba)

## Features

* interactive navigation
* items sorted by their recursive weight-sums
* accepts any data from stdin, so you can easily make your owns (see
  [examples](doc/examples.md))
  * input format can be altered (see [Configuration](#Configuration) section)
* interface:
  * preview window (can be toggled)
  * human-readable big numbers formatting (can be toggled)

## Usage

```shell
... | nctok
```

Where `...` should produce output in format:

```
<number> <path/to/item>
<number> <path/to/another_item>
```

For concrete usecases see [examples](doc/examples.md).

### Keys/control

* `j`/`k` - go down/up in list
* `l`/`h` - go deep down/up (inside/out)
* `g` - go to top (very first item)
* `G` - go to bottom (latest item)
* `w` - toggle preview window
* `a` - toggle human-readable numbers formatting
* `q` - quit

## Configuration

```
$ nctok --help
...
Options:
      --reverse  Invert sort order
Parser:
      --number-delimiter <NUMBER_DELIMITER>
          Delimiter between items number value and path [default: " "]
      --path-separator <PATH_SEPARATOR>
          Separator in items path [default: /]
Interface:
      --preview
          Show preview window for entry under cursor (can also toggle with 'w' key)
      --no-human-readable
          Disable formatting big numbers in human-readable (can also toggle with 'a' key)
```

## Installation

### With cargo (from [crates.io](https://crates.io/crates/nctok))

```shell
cargo install nctok
```

### From sources

```shell
cargo install --path .
```

## Inspiration

* [ncdu](https://dev.yorhel.nl/ncdu) - (ncurses) disk usage analyzer. This is
  main source of inspiration. I took exactly their idea and made unified
  interface for analyzing any data with the same (and a bit better) interface.
* [vifm](https://github.com/vifm/vifm) - terminal file manager. I took preview
  window from it.
* [tokei](https://github.com/XAMPPRocky/tokei) - lines-of-code counter. Well,
  at very first I wanted tokei+ncdu experience, so this project was born.
