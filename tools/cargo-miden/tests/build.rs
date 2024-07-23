use std::{env, fs, vec};

use cargo_component_core::terminal;
use cargo_miden::run;

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
    // Signal to `cargo-miden` that we're running in a test harness.
    //
    // This is necessary because cfg!(test) does not work for integration tests, so we're forced
    // to use an out-of-band signal like this instead
    env::set_var("TEST", "1");

    let restore_dir = env::current_dir().unwrap();
    let temp_dir = env::temp_dir();
    env::set_current_dir(&temp_dir).unwrap();
    let project_name = "test-proj";
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

    let terminal = terminal::Terminal::new(terminal::Verbosity::Verbose, terminal::Color::Auto);
    let outputs = run(args.into_iter(), &terminal).expect("Failed to create new project");
    let new_project_path = outputs.first().unwrap().canonicalize().unwrap();
    dbg!(&new_project_path);
    assert!(new_project_path.exists());
    assert_eq!(new_project_path, expected_new_project_dir.canonicalize().unwrap());
    env::set_current_dir(&new_project_path).unwrap();
    let args = ["cargo", "miden", "build", "--release"].iter().map(|s| s.to_string());
    let outputs = run(args, &terminal).expect("Failed to compile");
    let expected_masm_path = outputs.first().unwrap();
    dbg!(&expected_masm_path);
    // eprintln!("{}", String::from_utf8(fs::read(expected_masm_path).unwrap()).unwrap());
    assert!(expected_masm_path.exists());
    assert!(expected_masm_path.metadata().unwrap().len() > 0);

    env::set_current_dir(restore_dir).unwrap();
    fs::remove_dir_all(new_project_path).unwrap();
}
