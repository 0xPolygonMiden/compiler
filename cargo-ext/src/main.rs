use clap::Parser;
use commands::CargoCli;
use commands::Commands;

mod commands;

fn main() {
    let args = match CargoCli::parse() {
        CargoCli::Miden(args) => args,
    };

    match args.command {
        Commands::Compile {
            target,
            bin_name,
            is_library,
            output_file,
        } => {
            // TODO: pass unrecognized flags to the midenc
            todo!("run cargo");
        }
    }
}
