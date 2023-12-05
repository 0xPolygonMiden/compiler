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
    /// Compile the current project to MASM
    #[command(name = "compile")]
    Compile {
        /// The target environment to compile for
        #[arg(long = "target", value_name = "TARGET", default_value_t = TargetEnv::Base)]
        target: TargetEnv,

        /// Tells the compiler to produce an executable Miden program from the binary target or a library from the lib target if not specified
        #[arg(long = "bin-name", display_order(3))]
        bin_name: Option<String>,

        /// Output directory for the compiled MASM file(s)
        #[arg(
            long = "out-dir",
            value_name = "FOLDER",
            id = "output-folder",
            display_order(6)
        )]
        output_folder: PathBuf,
    },
    /// Scaffold a new Miden project at the given path
    #[command(name = "new")]
    New { path: PathBuf },
}
