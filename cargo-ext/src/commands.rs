use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use midenc_session::TargetEnv;

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Miden(MidenArgs),
}

#[derive(Parser, Debug)]
#[command(name = "miden")]
#[command(bin_name = "cargo miden")]
#[command(about = "Cargo command for developing Miden projects", long_about = None)]
pub struct MidenArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Invoke the compiler frontend
    #[command(next_display_order(10), name = "compile", about = "Compile to MASM")]
    Compile {
        /// The target environment to compile for
        #[arg(long = "target", value_name = "TARGET", default_value_t = TargetEnv::Base, display_order(2))]
        target: TargetEnv,

        /// Tells the compiler to produce an executable Miden program from the binary target
        #[arg(long = "bin-name", display_order(3))]
        bin_name: String,

        /// Tells the compiler to produce a Miden library from the lib target
        #[arg(
            long = "lib",
            conflicts_with("bin-name"),
            default_value_t = false,
            display_order(4)
        )]
        is_library: bool,

        /// Write output to `<filename>`
        #[arg(
            short = 'o',
            value_name = "FILENAME",
            id = "output-file",
            display_order(6)
        )]
        output_file: Option<PathBuf>,
    },
}
