use cargo_component_core::terminal::{Terminal, Verbosity};
use cargo_miden::{config::CargoArguments, run};

fn main() -> anyhow::Result<()> {
    // Initialize logger
    let mut builder = env_logger::Builder::from_env("CARGO_MIDEN_TRACE");
    builder.format_indent(Some(2));
    builder.format_timestamp(None);
    builder.init();

    let cargo_args = CargoArguments::parse_from(std::env::args())?;
    let terminal = Terminal::new(
        if cargo_args.quiet {
            Verbosity::Quiet
        } else {
            match cargo_args.verbose {
                0 => Verbosity::Normal,
                _ => Verbosity::Verbose,
            }
        },
        cargo_args.color.unwrap_or_default(),
    );

    if let Err(e) = run(std::env::args(), &terminal) {
        terminal.error(format!("{e:?}"))?;
        std::process::exit(1);
    }

    Ok(())
}
