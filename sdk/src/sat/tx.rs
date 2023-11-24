use crate::asset::Asset;
use crate::note::Recipient;
use crate::note::Tag;

extern "C" {
    #[link_name = "miden::sat::tx::create_note"]
    pub fn create_note_inner(asset: Asset, tag: Tag, recipient: Recipient);
}

pub fn create_note(asset: Asset, tag: Tag, recipient: Recipient) {
    unsafe { create_note_inner(asset, tag, recipient) }
}
