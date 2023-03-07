use std::env;
use std::process;

use anyhow::{anyhow, bail};

mod compiler;
mod driver;
mod utils;

pub fn main() -> anyhow::Result<()> {
    if cfg!(not(debug_assertions)) {
        if env::var_os("MIDEN_TRACE").is_none() {
            human_panic::setup_panic!();
        }
    }

    // Initialize logger
    let mut builder = env_logger::Builder::from_env("MIDEN_TRACE");
    builder.format_indent(Some(2));
    if let Ok(precision) = env::var("MIDEN_TRACE_TIMING") {
        match precision.as_str() {
            "s" => builder.format_timestamp_secs(),
            "ms" => builder.format_timestamp_millis(),
            "us" => builder.format_timestamp_micros(),
            "ns" => builder.format_timestamp_nanos(),
            other => bail!(
                "invalid MIDEN_TRACE_TIMING precision, expected one of [s, ms, us, ns], got '{}'",
                other
            ),
        };
    } else {
        builder.format_timestamp(None);
    }
    builder.init();

    // Get current working directory
    let cwd = env::current_dir().map_err(|e| anyhow!("Current directory is invalid: {}", e))?;

    match driver::run_compiler(cwd, env::args_os()) {
        Ok(status) => process::exit(status),
        Err(err) => {
            if let Some(err) = err.downcast_ref::<clap::Error>() {
                err.exit()
            } else {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    }
}
