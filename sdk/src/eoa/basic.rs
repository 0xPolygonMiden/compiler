extern "C" {
    #[link_name = "miden::eoa::basic::auth_tx_rpo_falcon512"]
    pub fn auth_tx_rpo_falcon512_inner();
}

pub fn auth_tx_rpo_falcon512() {
    unsafe { auth_tx_rpo_falcon512_inner() }
}
