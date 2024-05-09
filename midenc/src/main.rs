use std::env;

use anyhow::anyhow;
use midenc_driver::{self as driver, DriverError};

pub fn main() -> Result<(), DriverError> {
    if cfg!(not(debug_assertions)) && env::var_os("MIDENC_TRACE").is_none() {
        human_panic::setup_panic!();
    }

    // Initialize logger
    let mut builder = env_logger::Builder::from_env("MIDENC_TRACE");
    builder.format_indent(Some(2));
    if let Ok(precision) = env::var("MIDENC_TRACE_TIMING") {
        match precision.as_str() {
            "s" => builder.format_timestamp_secs(),
            "ms" => builder.format_timestamp_millis(),
            "us" => builder.format_timestamp_micros(),
            "ns" => builder.format_timestamp_nanos(),
            other => {
                return Err(DriverError::Failed(anyhow!(
                    "invalid MIDENC_TRACE_TIMING precision, expected one of [s, ms, us, ns], got \
                     '{}'",
                    other
                )))
            }
        };
    } else {
        builder.format_timestamp(None);
    }
    builder.init();

    // Get current working directory
    let cwd = env::current_dir()?;

    match driver::run(cwd, env::args_os()) {
        Err(DriverError::Clap(err)) => err.exit(),
        result => result,
    }
}
