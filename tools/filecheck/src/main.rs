use std::env;
use std::path::PathBuf;

use anyhow::{anyhow, bail};
use clap::Parser;
use lit::event_handler::Default as EventHandler;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Config {
    /// Path to directory containing the lit tests to be run
    #[arg(value_name = "DIR")]
    tests: PathBuf,
    /// Specify one or more file extensions to include when searching for test inputs
    #[arg(long = "type", short = 't', default_value = "hir")]
    file_types: Vec<String>,
    /// Define one or more variables to make available in test commands
    ///
    /// You must specify each one in `key=value` format, comma-separated.
    #[arg(long = "define", short = 'D')]
    defines: Vec<String>,
}

pub fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let cwd = env::current_dir().unwrap();
    let test_path = config.tests.as_path();
    let lit_dir = if test_path.is_file() {
        test_path.parent().unwrap().to_str().unwrap()
    } else {
        test_path.to_str().unwrap()
    };

    let midenc_exe = cwd.join("bin/midenc");
    if !midenc_exe.is_file() {
        bail!(
            "expected to find midenc at {}, but it either doesn't exist, or is not a file",
            midenc_exe.display()
        );
    }

    let mut defines = Vec::with_capacity(config.defines.len());
    for define in config.defines.iter() {
        if let Some((key, value)) = define.split_once('=') {
            defines.push((key.to_string(), value.to_string()));
        } else {
            bail!(
                "invalid variable definition, expected 'key=value', got '{}'",
                define
            );
        }
    }

    lit::run::tests(EventHandler::default(), move |runner| {
        runner.add_search_path(test_path.to_str().unwrap());
        for ty in config.file_types.iter() {
            runner.add_extension(ty.as_str());
        }

        runner
            .constants
            .insert("tests".to_string(), lit_dir.to_string());
        runner.constants.insert(
            "midenc".to_string(),
            midenc_exe.to_str().unwrap().to_string(),
        );

        for (k, v) in defines.drain(..) {
            runner.constants.insert(k, v);
        }
    })
    .map_err(|_| anyhow!("lit tests failed, see output for details"))
}
