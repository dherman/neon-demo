// rust-bindings infers the project name from Cargo.toml by default
var demo = require("rust-bindings")();

console.log(demo);
console.log(demo.make_an_array());
console.log(demo.make_a_number());
console.log(demo.escape_example());
console.log(demo.should_panic());

exports.makeAnArray = demo.make_an_array;
