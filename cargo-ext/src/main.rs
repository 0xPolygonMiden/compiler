use anyhow::Context;
use cargo_miden::compile;
use cargo_miden::new_project;
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
            compile(target, bin_name, &output_file).context(format!("Failed to compile {}", target))
        }
        Commands::New { path } => new_project(path).context("Failed to scaffold a new project"),
    }
}
