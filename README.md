A demo of using [Neon](http://github.com/dherman/neon) and [rust-bindings](http://github.com/dherman/rust-bindings) to integrate Rust and Node.

## Setup

**Note: this is currently only working on OS X.**

### OS X

* [XCode](https://developer.apple.com/xcode/download/)
* Node: v4 or later. I recommend using [nvm](https://github.com/creationix/nvm#install-script):

```
% nvm install 4
```

## Running the Demo

The whole build process is automated as part of package installation, so building and running is straightforward:

```
% nvm use 4
% npm install
% node -e 'require("./")'
```
