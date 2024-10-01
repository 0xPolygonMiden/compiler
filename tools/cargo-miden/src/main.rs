use anyhow::Ok;
use cargo_component_core::terminal::{Color, Terminal, Verbosity};
use cargo_miden::run;

fn main() -> anyhow::Result<()> {
    // Initialize logger
    let mut builder = env_logger::Builder::from_env("CARGO_MIDEN_TRACE");
    builder.format_indent(Some(2));
    builder.format_timestamp(None);
    builder.init();

    if let Err(e) = run(std::env::args()) {
        let terminal = Terminal::new(Verbosity::Normal, Color::Auto);
        terminal.error(format!("{e}"))?;
        std::process::exit(1);
    }
    Ok(())
}
