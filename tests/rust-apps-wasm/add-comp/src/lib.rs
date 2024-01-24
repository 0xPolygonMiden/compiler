#![no_std]

cargo_component_bindings::generate!();

use crate::bindings::exports::miden::add::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
