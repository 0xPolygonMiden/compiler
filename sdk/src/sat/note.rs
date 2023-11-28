use crate::asset::Asset;

extern "C" {
    #[link_name = "miden::sat::note::get_assets"]
    pub fn get_assets_inner() -> Asset;
}

pub fn get_assets() -> Asset {
    unsafe { get_assets_inner() }
}
