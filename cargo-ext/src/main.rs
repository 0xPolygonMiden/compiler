use anyhow::Context;
use cargo_miden::compile;
use clap::Parser;
use cli_commands::CargoCli;
use cli_commands::Commands;

mod cli_commands;

fn main() -> anyhow::Result<()> {
    let args = match CargoCli::parse() {
        CargoCli::Miden(args) => args,
    };

    match args.command {
        Commands::Compile {
            target,
            bin_name,
            output_file,
        } => {
            // TODO: ensure wasm32-unknown-unknown target is installed
            // TODO: pass unrecognized flags to the midenc
            compile(target, bin_name, &output_file).context(format!("Failed to compile {}", target))
        }
    }
}
