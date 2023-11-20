use crate::utils::get_test_path;
use cargo_miden::compile;
use midenc_session::TargetEnv;
use std::env;

// rusty_fork::rusty_fork_test! {

#[test]
fn compile_template() {
    let restore_dir = env::current_dir().unwrap();
    let test_dir = get_test_path("template");
    env::set_current_dir(&test_dir).unwrap();
    let masm_path_rel = "target/miden_lib.masm";
    // dbg!(&test_dir);
    compile(TargetEnv::Base, None, test_dir.join(masm_path_rel));
    env::set_current_dir(restore_dir).unwrap();
    let masm_file_path = test_dir.join(masm_path_rel);
    assert!(masm_file_path.exists());
    assert!(masm_file_path.metadata().unwrap().len() > 0);
}

// }
