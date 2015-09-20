// The rust-bindings package infers the project name from Cargo.toml,
// builds the native module if it is not yet built, and loads it.
var demo = require("rust-bindings")();

console.log(demo);
console.log(demo.make_an_array());
