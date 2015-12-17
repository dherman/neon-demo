A demo of using [Neon](http://github.com/dherman/neon) and [rust-bindings](http://github.com/dherman/rust-bindings) to integrate Rust and Node.

## Setup

**Note: this is currently only working on OS X.**

### OS X

* [XCode](https://developer.apple.com/xcode/download/)
* Node: Node 4 or later. I recommend using [nvm](https://github.com/creationix/nvm#install-script):

```
% nvm install 4
```

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
