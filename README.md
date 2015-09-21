A demo of using [nanny](http://github.com/dherman/nanny) and [rust-bindings](http://github.com/dherman/rust-bindings) to integrate Rust and Node.

## Setup

**Note: this is currently only working on OS X.**

### OS X

* [XCode](https://developer.apple.com/xcode/download/)
* Node: io.js v3 or later. I recommend using [nvm](https://github.com/creationix/nvm#install-script):

```
% nvm install iojs
```

* [multirust](https://github.com/brson/multirust#quick-installation)

*Right now multirust is a mandatory dependency because it's used to run on Rust nightly by default. Once the [fix for a jemalloc linking bug](https://github.com/rust-lang/rust/pull/27400) makes it through the trains to stable, multirust will be an optional dependency and rust-bindings will default to the system Rust compiler.*

## Running the Demo

Make sure you are using io.js:

```
% nvm use iojs
```

Since [rust-bindings](http://github.com/dherman/rust-bindings) automates the build process, setting up and running this demo is just like any other Node package:

```
% npm install
% node -e 'require("./")'
```
