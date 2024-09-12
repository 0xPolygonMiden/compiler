use std::path::PathBuf;

use cargo_component::load_metadata;
use clap::{CommandFactory, Parser};
use config::CargoArguments;
use midenc_session::diagnostics::Report;
use new_project::NewCommand;

use crate::run_cargo_command::run_cargo_command;

mod build;
pub mod config;
mod new_project;
mod run_cargo_command;
mod target;

// re-export cargo-component's terminal module
pub use cargo_component_core::terminal;

fn version() -> &'static str {
    option_env!("CARGO_VERSION_INFO").unwrap_or(env!("CARGO_PKG_VERSION"))
}

/// The list of commands that are built-in to `cargo-miden`.
const BUILTIN_COMMANDS: &[&str] = &[
    "miden", // for indirection via `cargo miden`
    "new",
];

const AFTER_HELP: &str = "Unrecognized subcommands will be passed to cargo verbatim
     and the artifacts will be processed afterwards (e.g. `build` command compiles MASM).
     \nSee `cargo help` for more information on available cargo commands.";

/// Cargo integration for Miden
#[derive(Parser)]
#[clap(
    bin_name = "cargo",
    version,
    propagate_version = true,
    arg_required_else_help = true,
    after_help = AFTER_HELP
)]
#[command(version = version())]
enum CargoMiden {
    /// Cargo integration for Miden
    #[clap(subcommand, hide = true, after_help = AFTER_HELP)]
    Miden(Command), // indirection via `cargo miden`
    #[clap(flatten)]
    Command(Command),
}

#[derive(Parser)]
enum Command {
    New(NewCommand),
}

fn detect_subcommand<I, T>(args: I) -> Option<String>
where
    I: IntoIterator<Item = T>,
    T: Into<String> + Clone,
{
    let mut iter = args.into_iter().map(Into::into).skip(1).peekable();

    // Skip the first argument if it is `miden` (i.e. `cargo miden`)
    if let Some(arg) = iter.peek() {
        if arg == "miden" {
            iter.next().unwrap();
        }
    }

    for arg in iter {
        // Break out of processing at the first `--`
        if arg == "--" {
            break;
        }

        if !arg.starts_with('-') {
            return Some(arg);
        }
    }

    None
}

pub fn run<T>(args: T, terminal: &terminal::Terminal) -> Result<Vec<PathBuf>, Report>
where
    T: Iterator<Item = String>,
{
    let args = args.collect::<Vec<_>>();
    let subcommand = detect_subcommand(args.clone());

    let outputs = match subcommand.as_deref() {
        // Check for built-in command or no command (shows help)
        Some(cmd) if BUILTIN_COMMANDS.contains(&cmd) => {
            match CargoMiden::parse_from(args.clone()) {
                CargoMiden::Miden(cmd) | CargoMiden::Command(cmd) => match cmd {
                    Command::New(cmd) => vec![cmd.exec().map_err(Report::msg)?],
                },
            }
        }

        // If no subcommand was detected,
        None => {
            // Attempt to parse the supported CLI (expected to fail)
            CargoMiden::parse_from(args);

            // If somehow the CLI parsed correctly despite no subcommand,
            // print the help instead
            CargoMiden::command().print_long_help().map_err(Report::msg)?;
            Vec::new()
        }

        _ => {
            // Not a built-in command, run the cargo command
            let cargo_args =
                CargoArguments::parse_from(args.clone().into_iter()).map_err(Report::msg)?;
            let metadata = load_metadata(terminal, cargo_args.manifest_path.as_deref(), false)
                .map_err(Report::msg)?;
            if metadata.packages.is_empty() {
                return Err(Report::msg(format!(
                    "manifest `{path}` contains no package or the workspace has no members",
                    path = metadata.workspace_root.join("Cargo.toml")
                )));
            }

            let spawn_args: Vec<_> = args.into_iter().skip(1).collect();
            run_cargo_command(&metadata, subcommand.as_deref(), &cargo_args, &spawn_args)?
        }
    };
    Ok(outputs)
}
