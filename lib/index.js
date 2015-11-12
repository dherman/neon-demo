// rust-bindings infers the project name from Cargo.toml by default
var demo = require("rust-bindings")();

console.log(demo);
console.log(demo.make_an_array());
console.log(demo.make_a_number());
console.log(demo.escape_example());
console.log(demo.string_internal_size("hello world"));
console.log(demo.string_internal_size("a𝄞b"));

exports.makeAnArray = demo.make_an_array;
