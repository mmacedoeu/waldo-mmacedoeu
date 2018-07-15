# Subimages

[![Build Status](https://travis-ci.org/mmacedoeu/waldo-mmacedoeu.svg?branch=master)](https://travis-ci.org/mmacedoeu/waldo-mmacedoeu)
[![Language (Rust)](https://img.shields.io/badge/powered_by-Rust-blue.svg)](http://www.rust-lang.org/)

Waldo is program to check if images is a cropped part of the other one:

## Features

- [x] Async/Sync [actors](https://github.com/actix/actix).
- [x] Actor communication in a local/thread context.
- [x] Uses [Futures](https://crates.io/crates/futures) for asynchronous message handling.
- [x] Typed messages (No `Any` type).
- [x] [OpenCV 3](https://opencv.org/)
- [x] Speeded up robust features [SURF](https://en.wikipedia.org/wiki/Speeded_up_robust_features)

## Goals

- [ ] Parallelism with [Scatter-Gather](http://www.enterpriseintegrationpatterns.com/patterns/messaging/BroadcastAggregate.html) Pattern over Graph processing system
- [ ] Efficient partitioning and scheduling of computer vision and image processing data on bus networks using divisible load analysis [DLA](http://www.s3lab.ece.ufl.edu/publication/ivc2000.pdf)
- [ ] Image Processing Using [Graphs](http://www.cb.uu.se/~filip/ImageProcessingUsingGraphs/LectureNotes/Lecture1.pdf)
- [ ] Multi scaling [analysis](https://en.wikipedia.org/wiki/Pyramid_(image_processing))
- [ ] [Differential Dataflow](https://github.com/frankmcsherry/differential-dataflow)

## Background

### About the problem

Detecting a lossy image/object is the start of the most advanced image processing field. It could develop to harder problems like face recognition and medical image analysis.

### Base solution and optimization evolutions

The base solution consist on image features detecting and description extraction on a query image and also on the scene image target them searching for a match of the two. The descriptors must be closed enough on the scene in order to eliminated scathered descriptors providing false positives.

By features you have a number of algorithms like SURF we are using here and you also train a set of objects, like a set of selfies, using deep learning in order to have optimized features and later detected them on the scene.

The solution here use a know algorithm Speeded up robust features [SURF](https://en.wikipedia.org/wiki/Speeded_up_robust_features).

Optimization on search space could be done by pre processing the image to a single channel, grayscale, and dimension scaling to 1/2, 1/4, 1/8 the original size. There is a mininum size where you get the best speed/precision compromise.
![alt text](https://docs.opencv.org/3.4.1/Pyramids_Tutorial_Pyramid_Theory.png "A set of layers in which the higher the layer, the smaller the size.")

Partition space and representation optimization could be done by dividing the work to a multi core computer or cluster of computers in order to get almost linear speedups. By representing the original image in a graph with data and responsabilities you can submit the problem to be solved in Distributed Dataflow graph processing systems like GraphX. But you can further optimize it's efficiency by using [Differential Dataflow](https://github.com/frankmcsherry/differential-dataflow/blob/master/differentialdataflow.pdf). With Differential Dataflow you also get perfomance due to [COST analysis](http://www.frankmcsherry.org/graph/scalability/cost/2015/01/15/COST.html).

### Resilience to failures

The immutability nature and functional approach makes it easy to resume faulty computations by storing and retrieving it's state on it's internal data store see: (https://github.com/frankmcsherry/differential-dataflow#fault-tolerance)

Our solution also implements the Actor model by using the actix framework and it means Actor could be back to action by it's [supervisor](https://www.queryhome.com/tech/133270/what-is-the-supervisor-or-supervision-concept-in-actor-model)

## Working prototype

There is a working python prototype running under folder prototype, it is a proof of concept without performance optimization like pyramid or image partition but demostrate the solution works

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

`./target/release/subimage -l warn <IMAGE1> <IMAGE2>`

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