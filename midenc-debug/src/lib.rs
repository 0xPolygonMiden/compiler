#![feature(iter_array_chunks)]
#![feature(lazy_cell)]
#![allow(unused)]

mod cli;
mod config;
mod debug;
mod exec;
mod felt;
mod logger;
mod ui;

use std::rc::Rc;

use midenc_session::{
    diagnostics::{IntoDiagnostic, Report},
    Session,
};

pub use self::{
    cli::Debugger,
    config::DebuggerConfig,
    debug::*,
    exec::*,
    felt::{bytes_to_words, Felt, Felt as TestFelt, PopFromStack, PushToStack},
};

pub type ExecutionResult<T> = Result<T, Report>;

pub fn run(
    inputs: Option<DebuggerConfig>,
    args: Vec<miden_processor::Felt>,
    session: Rc<Session>,
    logger: Box<dyn log::Log>,
) -> ExecutionResult<()> {
    let mut builder = tokio::runtime::Builder::new_current_thread();
    let rt = builder.enable_all().build().into_diagnostic()?;
    rt.block_on(async move { start_ui(inputs, args, session, logger).await })
}

pub fn trace(
    _options: Option<DebuggerConfig>,
    _args: Vec<String>,
    _session: Rc<Session>,
) -> ExecutionResult<ExecutionTrace> {
    todo!()
}

pub async fn start_ui(
    inputs: Option<DebuggerConfig>,
    args: Vec<miden_processor::Felt>,
    session: Rc<Session>,
    logger: Box<dyn log::Log>,
) -> Result<(), Report> {
    use ratatui::crossterm as term;

    logger::DebugLogger::install(logger);

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = term::terminal::disable_raw_mode();
        let _ = term::execute!(std::io::stdout(), term::terminal::LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    let mut app = ui::App::new(inputs, args, session).await?;
    app.run().await?;

    Ok(())
}
