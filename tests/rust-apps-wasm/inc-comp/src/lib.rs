#![no_std]

cargo_component_bindings::generate!();

use crate::bindings::miden::add::add::add;
use crate::bindings::Guest;

struct Component;

impl Guest for Component {
    fn inc(a: u32) -> u32 {
        add(a, 1)
    }
}
