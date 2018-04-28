# `line-stickers-scraper-rs`

[![Build Status](https://travis-ci.org/guangie88/line-stickers-scraper-rs.svg?branch=master)](https://travis-ci.org/guangie88/line-stickers-scraper-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/0crl0n8bmx240nls/branch/master?svg=true)](https://ci.appveyor.com/project/guangie88/line-stickers-scraper-rs/branch/master)
[![Crates.io](https://img.shields.io/crates/v/line-stickers-scraper.svg)](https://crates.io/crates/line-stickers-scraper)

Experimental executable in Rust to scrap and save LINE sticker PNGs from URL.

Saves all scrapped LINE sticker PNGs from given URL into given output directory.

As with all scraper, this is highly dependent on the website DOM layout, so this
is likely to break over time (especially this is experimental 😊).

## Installation

```bash
cargo install line-stickers-scraper
```

This installs `lss` into your Cargo binary directory.

## Example usage

```bash
lss https://store.line.me/stickershop/product/7842/en -o output/ -vvv
```

This performs a GET request from the above URL:

* `-o output/`
  * and saves all the scrapped PNG image files into `output/` directory. The
    file names are extracted based on some unique ID from the image web URL.
* `-vvv`
  * and prints logs at verbosity level of 3.

For more argument details, type:

```bash
lss -h
```
