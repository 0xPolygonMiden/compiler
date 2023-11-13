use crate::utils::get_test_path;
use cargo_miden::compile;
use midenc_session::TargetEnv;
use std::env;
use std::fs;

#[test]
fn compile_template() {
    let restore_dir = env::current_dir().unwrap();
    let test_dir = get_test_path("template");
    env::set_current_dir(&test_dir).unwrap();
    let masm_path_rel = "target/miden_lib.masm";
    // dbg!(&test_dir);
    let output_file = test_dir.join(masm_path_rel);
    // dbg!(&output_file);
    compile(TargetEnv::Base, None, &output_file).expect("Failed to compile");
    env::set_current_dir(restore_dir).unwrap();
    assert!(output_file.exists());
    assert!(output_file.metadata().unwrap().len() > 0);
    fs::remove_file(output_file).unwrap();
}
