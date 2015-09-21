extern crate nanny;

use nanny::*;

use std::ffi::CString;

// Rust implementations of Node functions must be linked without mangled names.
#[no_mangle]
pub extern fn make_a_pi(call: &mut Call) {
    // Allocating JS values requires executing in a Scope.
    Scope::run(|scope| {
        // Set the current JS function activation's return value to an allocated number.
        call.set_return(scope.number(3.14));
    });
}

#[no_mangle]
pub extern fn make_an_array(call: &mut Call) {
    let result = Scope::run(|scope| {
        let mut array = scope.array(3);
        array.set(0, scope.integer(17));
        array.set(1, scope.integer(42));
        array.set(2, scope.integer(1999));
        call.set_return(array);
    });
    //println!(">>> this is Rust speaking: I have a Rust value: {:?}", result);
}

/*
// This produces a lifetime error as expected:
fn naughty<'a>() -> Local<'a, Integer> {
    Scope::run(|scope| {
        scope.integer(17)
    })
}
 */

// The Rust native module must contain a function called `node_main` that takes the
// module object and can use this to set its exports.
#[no_mangle]
pub extern fn node_main(mut module: Local<Object>) {
    module.export(&CString::new("make_a_pi").unwrap(), make_a_pi);
    module.export(&CString::new("make_an_array").unwrap(), make_an_array);
}
