use crate::utils::get_test_path;
use cargo_miden::compile;
use midenc_session::TargetEnv;
use std::env;
use std::fs;

#[test]
fn compile_template() {
    let restore_dir = env::current_dir().unwrap();
    let test_dir = get_test_path("template");
    // dbg!(&test_dir);
    env::set_current_dir(&test_dir).unwrap();
    let masm_path_rel = "target";
    let output_folder = test_dir.join(masm_path_rel);
    compile(TargetEnv::Base, None, &output_folder).expect("Failed to compile");
    env::set_current_dir(restore_dir).unwrap();
    let expected_masm_path = output_folder.join("miden_wallet_lib.masm");
    assert!(expected_masm_path.exists());
    assert!(expected_masm_path.metadata().unwrap().len() > 0);
    fs::remove_file(expected_masm_path).unwrap();
}
