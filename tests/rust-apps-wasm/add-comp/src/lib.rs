#![no_std]

cargo_component_bindings::generate!();

use crate::bindings::Guest;

struct Component;

impl Guest for Component {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
