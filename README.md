A demo of using [nanny](http://github.com/dherman/nanny) and [rust-bindings](http://github.com/dherman/rust-bindings) to integrate Rust and Node.

## Dependencies

**Note: This is currently only working on OS X.**

Basically you just need Node and Rust.

### Node

I strongly recommend using [nvm](https://github.com/creationix/nvm#install-script) for managing Node installations. For this project, io.js version 3 or greater is required:

```
% nvm use iojs
```

### Rust

For this project, [multirust](https://github.com/brson/multirust#quick-installation) is required. The rust-bindings library uses it to automatically select the appropriate version of Rust.

## Usage

**Note: This is currently only working on OS X.**

The point of rust-bindings is to automate the build process, so you don't need any special build steps to use this project. Simply install the npm dependencies as usual and you can run the demo:

```
% npm install
% node -e 'require("./")'
```

Running the demo causes the Rust native module to build and load automatically.
