A demo of using [nanny](http://github.com/dherman/nanny) and [rust-bindings](http://github.com/dherman/rust-bindings) to integrate Rust and Node.

## Setup

**Note: this is currently only working on OS X.**

### OS X

* [XCode](https://developer.apple.com/xcode/download/)
* Node: Node 4 or later. I recommend using [nvm](https://github.com/creationix/nvm#install-script):

```
% nvm install 4
```

* [multirust](https://github.com/brson/multirust#quick-installation)

*Right now multirust is a mandatory dependency because it's used to run on Rust nightly by default. Once [a few patches land in Rust stable](https://github.com/dherman/rust-bindings/issues/2), rust-bindings will default to the system Rust compiler and multirust will become an optional dependency.*

## Running the Demo

Make sure you are using the right Node version:

```
% nvm use 4
```

Since [rust-bindings](http://github.com/dherman/rust-bindings) automates the build process, setting up and running this demo is just like any other Node package:

```
% npm install
% node -e 'require("./")'
```
