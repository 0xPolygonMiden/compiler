use std::path::PathBuf;

use anyhow::bail;
use cargo_component::{
    config::{CargoArguments, Config},
    load_component_metadata, load_metadata, run_cargo_command,
};
use cargo_component_core::{
    command::{CACHE_DIR_ENV_VAR, CONFIG_FILE_ENV_VAR},
    terminal::{Color, Terminal, Verbosity},
};
use clap::{CommandFactory, Parser};
use commands::NewCommand;
use compile_masm::wasm_to_masm;
use non_component::run_cargo_command_for_non_component;

mod commands;
mod compile_masm;
mod non_component;
mod target;

fn version() -> &'static str {
    option_env!("CARGO_VERSION_INFO").unwrap_or(env!("CARGO_PKG_VERSION"))
}

/// The list of commands that are built-in to `cargo-miden`.
const BUILTIN_COMMANDS: &[&str] = &[
    "miden", // for indirection via `cargo miden`
    "new",
];

/// The list of commands that are explicitly unsupported by `cargo-miden`.
///
/// These commands are intended to integrate with `crates.io` and have no
/// analog in `cargo-miden` currently.
const UNSUPPORTED_COMMANDS: &[&str] =
    &["install", "login", "logout", "owner", "package", "search", "uninstall"];

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
    let mut iter = args.into_iter().map(Into::into).peekable();

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

/// Requested output type for the `build` command
pub enum OutputType {
    Wasm,
    Masm,
    // Hir,
}

/// Runs the cargo-miden command
/// The arguments are expected to start with `["cargo", "miden", ...]` followed by a subcommand with options
// TODO: Use Report instead of anyhow?
pub fn run<T>(args: T, build_output_type: OutputType) -> anyhow::Result<Vec<PathBuf>>
where
    T: Iterator<Item = String>,
{
    let args = args.skip_while(|arg| arg == "cargo").collect::<Vec<_>>();
    let subcommand = detect_subcommand(args.clone());

    let outputs = match subcommand.as_deref() {
        // Check for built-in command or no command (shows help)
        Some(cmd) if BUILTIN_COMMANDS.contains(&cmd) => {
            match CargoMiden::parse_from(args.clone()) {
                CargoMiden::Miden(cmd) | CargoMiden::Command(cmd) => match cmd {
                    Command::New(cmd) => vec![cmd.exec()?],
                },
            }
        }
        // Check for explicitly unsupported commands (e.g. those that deal with crates.io)
        Some(cmd) if UNSUPPORTED_COMMANDS.contains(&cmd) => {
            let terminal = Terminal::new(Verbosity::Normal, Color::Auto);
            terminal.error(format!(
                "command `{cmd}` is not supported by `cargo component`\n\nuse `cargo {cmd}` \
                 instead"
            ))?;
            std::process::exit(1);
        }
        // If no subcommand was detected,
        None => {
            // Attempt to parse the supported CLI (expected to fail)
            CargoMiden::parse_from(args);

            // If somehow the CLI parsed correctly despite no subcommand,
            // print the help instead
            CargoMiden::command().print_long_help()?;
            Vec::new()
        }

        _ => {
            // Not a built-in command, run the cargo command
            let args = args.into_iter().skip_while(|arg| arg == "miden").collect::<Vec<_>>();
            let cargo_args = CargoArguments::parse_from(args.clone().into_iter())?;
            dbg!(&cargo_args);
            let cache_dir = std::env::var(CACHE_DIR_ENV_VAR).map(PathBuf::from).ok();
            let config_file = std::env::var(CONFIG_FILE_ENV_VAR).map(PathBuf::from).ok();
            let config = Config::new(
                Terminal::new(
                    if cargo_args.quiet {
                        Verbosity::Quiet
                    } else {
                        match cargo_args.verbose {
                            0 => Verbosity::Normal,
                            _ => Verbosity::Verbose,
                        }
                    },
                    cargo_args.color.unwrap_or_default(),
                ),
                config_file,
            )?;
            let metadata = load_metadata(cargo_args.manifest_path.as_deref())?;
            let mut packages = load_component_metadata(
                &metadata,
                cargo_args.packages.iter(),
                cargo_args.workspace,
            )?;

            if packages.is_empty() {
                bail!(
                    "manifest `{path}` contains no package or the workspace has no members",
                    path = metadata.workspace_root.join("Cargo.toml")
                );
            }

            for package in packages.iter_mut() {
                // TODO: do we want/need to explicitly specify the package version?
                package.metadata.section.bindings.with = [
                    ("miden:base/core-types@1.0.0/felt", "miden_sdk::Felt"),
                    ("miden:base/core-types@1.0.0/word", "miden_sdk::Word"),
                    ("miden:base/core-types@1.0.0/core-asset", "miden_sdk::CoreAsset"),
                    ("miden:base/core-types@1.0.0/tag", "miden_sdk::Tag"),
                    ("miden:base/core-types@1.0.0/note-type", "miden_sdk::NoteType"),
                    ("miden:base/core-types@1.0.0/recipient", "miden_sdk::Recipient"),
                ]
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            }

            let mut spawn_args: Vec<_> = args.into_iter().collect();
            spawn_args.extend_from_slice(
                &[
                    "-Z",
                    // compile std as part of crate graph compilation
                    // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
                    // to abort on panic below
                    "build-std=std,core,alloc,panic_abort",
                    "-Z",
                    // abort on panic without message formatting (core::fmt uses call_indirect)
                    "build-std-features=panic_immediate_abort",
                ]
                .map(|s| s.to_string()),
            );

            let env_vars =
                vec![("RUSTFLAGS".to_string(), "-C target-feature=+bulk-memory".to_string())]
                    .into_iter()
                    .collect();

            let mut builder = tokio::runtime::Builder::new_current_thread();
            let rt = builder.enable_all().build()?;
            dbg!(&packages);
            let mut wasm_outputs = rt.block_on(async {
                let client = config.client(cache_dir, cargo_args.offline).await?;
                run_cargo_command(
                    client,
                    &config,
                    &metadata,
                    &packages,
                    subcommand.as_deref(),
                    &cargo_args,
                    &spawn_args,
                    &env_vars,
                )
                .await
            })?;
            // TODO: analyze `packages` and find the ones that don't have a WIT component and get Wasm binary (core module) for them with our own version of run_cargo_command
            if wasm_outputs.is_empty() {
                // crates that don't have a WIT component are ignored by the `cargo-component` run_cargo_command
                // build them with our own version of run_cargo_command
                wasm_outputs = run_cargo_command_for_non_component(
                    &config,
                    subcommand.as_deref(),
                    &cargo_args,
                    &spawn_args,
                    &env_vars,
                )?;
            }
            dbg!(&wasm_outputs);
            match build_output_type {
                OutputType::Wasm => wasm_outputs,
                OutputType::Masm => {
                    let miden_out_dir =
                        metadata.target_directory.join("miden").join(if cargo_args.release {
                            "release"
                        } else {
                            "debug"
                        });
                    if !miden_out_dir.exists() {
                        std::fs::create_dir_all(&miden_out_dir)?;
                    }

                    let mut outputs = Vec::new();
                    for wasm in wasm_outputs {
                        let is_bin = false;
                        let output = wasm_to_masm(&wasm, miden_out_dir.as_std_path(), is_bin)
                            .map_err(|e| anyhow::anyhow!("{e}"))?;
                        outputs.push(output);
                    }
                    outputs
                }
            }
        }
    };
    Ok(outputs)
}
