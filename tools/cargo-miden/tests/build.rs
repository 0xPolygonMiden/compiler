use std::{env, fs, vec};

use cargo_miden::{run, OutputType};

// NOTE: This test sets the current working directory so don't run it in parallel with tests
// that depend on the current directory

fn new_project_args(project_name: &str, template_path: Option<&str>) -> Vec<String> {
    let mut args = vec!["cargo", "miden", "new", project_name];
    if let Some(template_path) = template_path {
        args.extend(["--template-path", template_path]);
    };
    args.into_iter().map(|s| s.to_string()).collect()
}

#[test]
fn build_new_project_from_template() {
    // let _ = env_logger::builder().is_test(true).try_init();
    // Signal to `cargo-miden` that we're running in a test harness.
    //
    // This is necessary because cfg!(test) does not work for integration tests, so we're forced
    // to use an out-of-band signal like this instead
    env::set_var("TEST", "1");

    let restore_dir = env::current_dir().unwrap();
    let temp_dir = env::temp_dir();
    env::set_current_dir(&temp_dir).unwrap();
    let project_name = "test_proj_underscore";
    let expected_new_project_dir = &temp_dir.join(project_name);
    dbg!(&expected_new_project_dir);
    if expected_new_project_dir.exists() {
        fs::remove_dir_all(expected_new_project_dir).unwrap();
    }

    let args = new_project_args(project_name, None);
    // let args = new_project_args(
    //     project_name,
    //     Some(
    //         &(format!(
    //             "{}/../../../rust-templates/account",
    //             std::env::var("CARGO_MANIFEST_DIR").unwrap()
    //         )),
    //     ),
    // );

    let outputs = run(args.into_iter(), OutputType::Masm).expect("Failed to create new project");
    let new_project_path = outputs.first().unwrap().canonicalize().unwrap();
    dbg!(&new_project_path);
    assert!(new_project_path.exists());
    assert_eq!(new_project_path, expected_new_project_dir.canonicalize().unwrap());
    env::set_current_dir(&new_project_path).unwrap();

    // build with the dev profile
    let args = ["cargo", "miden", "build"].iter().map(|s| s.to_string());
    let outputs = run(args, OutputType::Masm).expect("Failed to compile with the dev profile");
    assert_eq!(outputs.len(), 1);
    let expected_masm_path = outputs.first().unwrap();
    dbg!(&expected_masm_path);
    assert!(expected_masm_path.exists());
    assert!(expected_masm_path.to_str().unwrap().contains("/debug/"));
    assert_eq!(expected_masm_path.extension().unwrap(), "masp");
    assert!(expected_masm_path.metadata().unwrap().len() > 0);
    // assert_eq!(expected_masm_path.metadata().unwrap().len(), 0);

    // build with the release profile
    let args = ["cargo", "miden", "build", "--release"].iter().map(|s| s.to_string());
    let outputs = run(args, OutputType::Masm).expect("Failed to compile with the release profile");
    assert_eq!(outputs.len(), 1);
    let expected_masm_path = outputs.first().unwrap();
    dbg!(&expected_masm_path);
    assert!(expected_masm_path.exists());
    assert_eq!(expected_masm_path.extension().unwrap(), "masp");
    assert!(expected_masm_path.to_str().unwrap().contains("/release/"));
    assert!(expected_masm_path.metadata().unwrap().len() > 0);

    env::set_current_dir(restore_dir).unwrap();
    fs::remove_dir_all(new_project_path).unwrap();
}
