extern crate nanny;

use nanny::vm::{Call, JS, Module, Result};
use nanny::value::{Integer, Number, Array, Tagged};
use nanny::scope::Scope;

fn make_a_pi(call: Call) -> JS<Number> {
    Ok(Number::new(call.scope, 3.14))
}

fn make_an_array(call: Call) -> JS<Array> {
    let scope = call.scope;
    let mut array = Array::new(scope, 3);
    array.set(0, Integer::new(scope, 17));
    array.set(1, Integer::new(scope, 42));
    array.set(2, Integer::new(scope, 1999));
    //println!(">>> this is Rust speaking: I have a Rust value: {:?}", array);
    Ok(array)
}

fn make_a_number(call: Call) -> JS<Integer> {
    let x = Integer::new(call.scope, 1999);
    let y = call.scope.nested(|_| {
        17
    });
    println!("y = {}", y);
    Ok(x)
}

fn escape_example(call: Call) -> JS<Array> {
    let mut x = None;
    call.scope.chained(|scope| {
        let mut array = Array::new(scope, 2);
        array.set(0, Integer::new(scope, 42));
        array.set(1, Number::new(scope, 6.28));
        x = Some(scope.escape(array));
    });
    Ok(x.unwrap())
}

fn string_internal_size(call: Call) -> JS<Integer> {
    let s = try!(try!(call.arguments.require(call.scope, 0)).to_string(call.scope));
    let size = s.size();
    let result = Integer::new(call.scope, size as i32);
    Ok(result)
}

/*
use nanny::value::Undefined;

fn static_error_shadow(call: Call) -> JS<Integer> {
    let outer = call.scope;
    let mut x = Integer::new(outer, 0);
    let y = outer.nested(|_| {
        // error: outer is inactive!
        x = Integer::new(outer, 1999);
        17
    });
    println!("y = {}", y);
    Ok(x)
}

fn static_error_escape(call: Call) -> JS<Undefined> {
    let _ = call.scope.nested(|scope| {
        Integer::new(scope, 17)
    });
    Ok(Undefined::new(call.scope))
}
*/

#[no_mangle]
pub fn node_main(mut module: Module) -> Result<()> {
    try!(module.export("make_a_pi", make_a_pi));
    try!(module.export("make_an_array", make_an_array));
    try!(module.export("make_a_number", make_a_number));
    try!(module.export("escape_example", escape_example));
    try!(module.export("string_internal_size", string_internal_size));
    Ok(())
}
