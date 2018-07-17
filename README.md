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

The base solution consist on image features detecting and description extraction on a query image and also on the scene image and searching for a match of the two. The descriptors must be closed enough on the scene in order to eliminate scathered descriptors providing false positives.

By features you have a number of algorithms, like SURF we are using here, and you could also train a set of objects, like a set of selfies, using deep learning in order to have optimized features and later detected them on the scene.

The solution here use a know algorithm know as Speeded up robust features [SURF](https://en.wikipedia.org/wiki/Speeded_up_robust_features).

Optimization on search space could be done by pre processing the image to a single channel, grayscale, and dimension scaling to 1/2, 1/4, 1/8 the original size. There is a mininum size where you get the best speed/precision compromise.
![alt text](https://docs.opencv.org/3.4.1/Pyramids_Tutorial_Pyramid_Theory.png "A set of layers in which the higher the layer, the smaller the size.")

[Partition search space](https://en.wikipedia.org/wiki/Image_segmentation) and representation optimization could be done by dividing the work to a multi core computer or cluster of computers in order to get almost linear speedups. By representing the original image in a graph with data and responsibilities you can submit the problem to be solved in Distributed Dataflow graph processing systems like GraphX. But you can further optimize it's efficiency by using [Differential Dataflow](https://github.com/frankmcsherry/differential-dataflow/blob/master/differentialdataflow.pdf). With Differential Dataflow you also get perfomance due to [COST analysis](http://www.frankmcsherry.org/graph/scalability/cost/2015/01/15/COST.html).

### Robustness and tradeoffs

Since the problem is somewhat relaxed by detecting just cropped images on another image, other algorithms could probably be applied like [Parametric Image Alignment using Enhanced Correlation Coefficient Maximization](http://xanthippi.ceid.upatras.gr/people/evangelidis/george_files/PAMI_2008.pdf)
which determine the translation transform with main advantages:

* Unlike the traditional similarity measure of difference in pixel intensities, ECC is invariant to photometric distortions in contrast and brightness;
* Although the objective function is nonlinear function of the parameters, the iterative scheme to solve the optimization problem is linear. In other words, a problem that looks computationally expensive on the surface uses a simpler way to solve it iteratively;

Or [Template matching](https://en.wikipedia.org/wiki/Template_matching) but since features based matching have advantages with partial matching in a segmented image it could scale better in a cluster of computers. So I decided to stick with features based solution mainly for this while also gaining robustness using lossy images with quality as low as 10% and still have accuracy.

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

The recommended version is the one present at https://github.com/mmacedoeu/cv-rs
which is the one the binding is tested against. You can also reuse the installation script used by CI at https://github.com/mmacedoeu/cv-rs/blob/master/.ci/travis_build_opencv.sh

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

## Evolution

This solution could evolve to deep learning based feature detection using CUDA or OpenCL while also retaining it's main graph based Differential Dataflow

## Runtime characteristics

By using the rust language which features minimum runtime library and overhead while also don't make use of Garbage Collectors for memory management means you have not all the penalties of other languages and graph solutions under stress conditions like facebook research have show on this [article](https://code.fb.com/core-data/a-comparison-of-state-of-the-art-graph-processing-systems/) "...when the machines start getting low on memory, performance drops dramatically. This happens at a different number of machines for Giraph and GraphX." So our system have predictable runtime characteristics and don't sufer from such performance degradation under stress load and states of low memory avaliable.

By evaluating the [COST](http://www.frankmcsherry.org/graph/scalability/cost/2015/01/15/COST.html) you can see the runtime characteristics of our solution is scalable to order of magnitude better than know graph processing systems like graphX: "With 128x as many cores, none of the scalable systems consistently out-perform a single thread at PageRank, which is probably one of the simplest graph computations (sparse matrix-vector multiplications). The systems are almost a factor of two slower than the single-threaded implementation for label propagation."
