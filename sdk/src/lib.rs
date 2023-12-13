#![no_std]

cargo_component_bindings::generate!();

use crate::bindings::exports::miden::base::types::{GuestAsset, Word};

pub struct Asset {
    inner: Word,
}

impl GuestAsset for Asset {
    fn new(word: Word) -> Self {
        Self { inner: word }
    }

    fn as_word(&self) -> Word {
        self.inner
    }
}
