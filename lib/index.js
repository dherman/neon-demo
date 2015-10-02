// rust-bindings infers the project name from Cargo.toml by default
var demo = require("rust-bindings")();

console.log(demo);
console.log(demo.make_an_array());

exports.makeAnArray = demo.make_an_array;
