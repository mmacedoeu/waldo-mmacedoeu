# Subimages

[![Build Status](https://travis-ci.org/mmacedoeu/waldo-mmacedoeu.svg?branch=master)](https://travis-ci.org/mmacedoeu/waldo-mmacedoeu)
[![Language (Rust)](https://img.shields.io/badge/powered_by-Rust-blue.svg)](http://www.rust-lang.org/)

Waldo is program to check if images is a cropped part of the other one:

## Features

* Async/Sync [actors](https://github.com/actix/actix).
* Actor communication in a local/thread context.
* Uses [Futures](https://crates.io/crates/futures) for asynchronous message handling.
* Typed messages (No `Any` type).
* [OpenCV 3](https://opencv.org/)
* Speeded up robust features [SURF](https://en.wikipedia.org/wiki/Speeded_up_robust_features)

## Not Featured

* Preprocessing scaling optimization

## Working prototype

There is a working python prototype running under folder prototype

Display help:

`python subimage.py -h`

```text
python subimage.py -h
usage: subimage.py [-h] [--input1 INPUT1] [--input2 INPUT2]

Prototype for finding if one image is cropped from another.

optional arguments:
  -h, --help       show this help message and exit
  --input1 INPUT1  Path to input image 1.
  --input2 INPUT2  Path to input image 2.

```

## Install

You need to manually install [OpenCV 3](https://www.learnopencv.com/install-opencv3-on-ubuntu/)

To compile and install you need to first install Rust [compiler](https://www.rust-lang.org/en-US/install.html)

Compile for release

`cargo build --release`

## Platform support

Should compile and work on all rust compiler supported [plataforms](https://forge.rust-lang.org/platform-support.html) but only tested for 64bit linux

## Usage

Display help:

`./target/release/subimage --help`

```text
Subimages 0.1.0
Marcos Macedo <contato@mmacedo.eu.org>
Provided two jpeg images find if one is a cropped image of the other

USAGE:
    subimage [OPTIONS] <IMAGE1> <IMAGE2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --log <LOG_PATTERN>    Sets a custom logging

ARGS:
    <IMAGE1>    Sets the first image file to use
    <IMAGE2>    Sets the second image file to use
```

Run with full trace:

`./target/release/subimage -l trace <IMAGE1> <IMAGE2>`

Run with no logging:

`./target/release/swapi -l warn <IMAGE1> <IMAGE2>`

## Testing

### Manual Testing

For now some images are provided in the assets folder

### Lossy robustness

For some scenarios even a selfie with as low quality as 10% still could be matched on the scene like morning_selfie9_10quality.jpg

### Debug Image out

Python Prototype outputs an image on current folder with selfie highlighted with green squares

![alt text](https://github.com/mmacedoeu/waldo-mmacedoeu/raw/master/matches.jpg "Image output from prototype for debug")

## Progress

Rust version is now barely some scafolding project but still reading images with openCV. Due to lack of complete and official openCV binding for rust the only one avaliable is incomplete and some functions used on the prototype are missing like findHomography and perspectiveTransform. Provided enough time I will try to include those functions myself.