extern crate nanny;

use nanny::vm::Call;
use nanny::value::Object;
use nanny::mem::Handle;
use nanny::scope::Scope;

use std::ffi::CString;

// Rust implementations of Node functions must be linked without mangled names.
#[no_mangle]
pub extern fn make_a_pi(call: &Call) {
    // Allocating JS values requires executing in a Scope.
    call.realm().scoped(|scope| {
        // Set the current JS function activation's return value to an allocated number.
        call.activation().set_return(scope.number(3.14));
    });
}

#[no_mangle]
pub extern fn make_an_array(call: &Call) {
    call.realm().scoped(|scope| {
        let mut array = scope.array(3);
        array.set(0, scope.integer(17));
        array.set(1, scope.integer(42));
        array.set(2, scope.integer(1999));
        call.activation().set_return(array);
    });
    //println!(">>> this is Rust speaking: I have a Rust value: {:?}", result);
}

#[no_mangle]
pub extern fn make_a_number(call: &Call) {
    call.realm().scoped(|scope| {
        let x = scope.integer(1999);
        let y = scope.nested(|_| {
            17
        });
        println!("y = {}", y);
        call.activation().set_return(x);
    });
}

#[no_mangle]
pub extern fn escape_example(call: &Call) {
    call.realm().scoped(|scope| {
        let mut x = None;
        scope.chained(|scope| {
            let mut array = scope.array(2);
            array.set(0, scope.integer(42));
            array.set(1, scope.number(6.28));
            x = Some(scope.escape(array));
        });
        call.activation().set_return(x.unwrap());
    });
}

#[no_mangle]
pub extern fn should_panic(call: &Call) {
    call.realm().scoped(|outer| {
        let mut x = outer.integer(0);
        let y = outer.nested(|_| {
            // panic: outer scope is inactive!
            x = outer.integer(1999);
            17
        });
        println!("y = {}", y);
        call.activation().set_return(x);
    });
}

/*
// This produces a lifetime error as expected:
fn naughty<'a>() -> Handle<'a, Integer> {
    call.realm().scoped(|scope| {
        scope.integer(17)
    })
}
 */

// The Rust native module must contain a function called `node_main` that takes the
// module object and can use this to set its exports.
#[no_mangle]
pub extern fn node_main(mut module: Handle<Object>) {
    module.export(&CString::new("make_a_pi").unwrap(), make_a_pi);
    module.export(&CString::new("make_an_array").unwrap(), make_an_array);
    module.export(&CString::new("make_a_number").unwrap(), make_a_number);
    module.export(&CString::new("escape_example").unwrap(), escape_example);
    module.export(&CString::new("should_panic").unwrap(), should_panic);
}
