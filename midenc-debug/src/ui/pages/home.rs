use midenc_session::diagnostics::{IntoDiagnostic, Report};
use ratatui::{
    crossterm::{
        self,
        event::{KeyCode, KeyEvent},
    },
    prelude::*,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    ui::{
        action::Action,
        pages::Page,
        panes::{
            breakpoints::BreakpointsPane, disasm::DisassemblyPane, source_code::SourceCodePane,
            stack::OperandStackPane, stacktrace::StackTracePane, Pane,
        },
        state::{InputMode, State},
        tui::EventResponse,
    },
    Breakpoint, BreakpointType, ReadMemoryExpr,
};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    panes: Vec<Box<dyn Pane>>,
    focused_pane_index: usize,
    fullscreen_pane_index: Option<usize>,
}

impl Home {
    pub fn new() -> Result<Self, Report> {
        let focused_border_style = Style::default().fg(Color::LightGreen);

        Ok(Self {
            command_tx: None,
            panes: vec![
                Box::new(SourceCodePane::new(true, focused_border_style)),
                Box::new(DisassemblyPane::new(false, focused_border_style)),
                Box::new(StackTracePane::new(false, focused_border_style)),
                Box::new(OperandStackPane::new(false, focused_border_style)),
                Box::new(BreakpointsPane::new(false, focused_border_style)),
            ],

            focused_pane_index: 0,
            fullscreen_pane_index: None,
        })
    }
}

impl Page for Home {
    fn init(&mut self, state: &State) -> Result<(), Report> {
        for pane in self.panes.iter_mut() {
            pane.init(state)?;
        }
        Ok(())
    }

    fn focus(&mut self) -> Result<(), Report> {
        if let Some(command_tx) = &self.command_tx {
            const ARROW: &str = symbols::scrollbar::HORIZONTAL.end;
            let status_line =
                format!("[l,h {ARROW} pane movement] [: {ARROW} commands] [q {ARROW} quit]");
            command_tx.send(Action::StatusLine(status_line)).into_diagnostic()?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), Report> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>, Report> {
        let mut actions: Vec<Option<Action>> = vec![];
        match action {
            Action::Tick => {}
            Action::FocusNext => {
                let next_index = self.focused_pane_index.saturating_add(1) % self.panes.len();
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::UnFocus, state)?);
                }
                self.focused_pane_index = next_index;
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::Focus, state)?);
                }
            }
            Action::FocusPrev => {
                let prev_index =
                    self.focused_pane_index.saturating_add(self.panes.len() - 1) % self.panes.len();
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::UnFocus, state)?);
                }
                self.focused_pane_index = prev_index;
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::Focus, state)?);
                }
            }
            Action::Update => {
                for pane in self.panes.iter_mut() {
                    actions.push(pane.update(action.clone(), state)?);
                }
            }
            Action::ToggleFullScreen => {
                self.fullscreen_pane_index =
                    self.fullscreen_pane_index.map_or(Some(self.focused_pane_index), |_| None);
            }
            Action::FocusFooter(..) => {
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::UnFocus, state)?);
                }
            }
            Action::FooterResult(cmd, Some(args)) if cmd.eq(":") => {
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    pane.update(Action::Focus, state)?;
                }
                // Dispatch commands of the form: CMD [ARGS..]
                match args.split_once(' ') {
                    Some((cmd, rest)) => match cmd.trim() {
                        "b" | "break" | "breakpoint" => match rest.parse::<BreakpointType>() {
                            Ok(ty) => {
                                state.create_breakpoint(ty);
                                actions.push(Some(Action::TimedStatusLine(
                                    "breakpoint created".to_string(),
                                    1,
                                )));
                            }
                            Err(err) => {
                                actions.push(Some(Action::TimedStatusLine(err, 5)));
                            }
                        },
                        "r" | "read" => match rest.parse::<ReadMemoryExpr>() {
                            Ok(expr) => match state.read_memory(&expr) {
                                Ok(result) => actions.push(Some(Action::StatusLine(result))),
                                Err(err) => actions.push(Some(Action::TimedStatusLine(err, 5))),
                            },
                            Err(err) => actions.push(Some(Action::TimedStatusLine(err, 5))),
                        },
                        _ => {
                            log::debug!("unknown command with arguments: '{cmd} {args}'");
                            actions.push(Some(Action::TimedStatusLine("unknown command".into(), 1)))
                        }
                    },
                    None => match args.trim() {
                        "q" | "quit" => actions.push(Some(Action::Quit)),
                        "reload" => {
                            actions.push(Some(Action::Reload));
                        }
                        "debug" => {
                            actions.push(Some(Action::ShowDebug));
                        }
                        invalid => {
                            log::debug!("unknown command: '{invalid}'");
                            actions.push(Some(Action::TimedStatusLine("unknown command".into(), 1)))
                        }
                    },
                }
            }
            Action::FooterResult(_cmd, None) => {
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(Action::Focus, state)?);
                }
            }
            Action::Continue => {
                let start_cycle = state.executor.cycle;
                let mut breakpoints = core::mem::take(&mut state.breakpoints);
                state.stopped = false;
                let stopped = loop {
                    // If stepping the program results in the program terminating succesfully, stop
                    if state.executor.stopped {
                        break true;
                    }

                    let mut consume_most_recent_finish = false;
                    match state.executor.step() {
                        Ok(Some(exited)) if exited.should_break_on_exit() => {
                            consume_most_recent_finish = true;
                        }
                        Ok(_) => (),
                        Err(err) => {
                            // Execution terminated with an error
                            state.execution_failed = Some(err);
                            break true;
                        }
                    }

                    if breakpoints.is_empty() {
                        // No breakpoint management needed, keep executing
                        continue;
                    }

                    let (op, is_op_boundary, proc, loc) = match state.executor.last.as_ref() {
                        Some(last_state) => {
                            let op = last_state.op;
                            let is_boundary = last_state
                                .asmop
                                .as_ref()
                                .map(|info| info.cycle_idx() == 1)
                                .unwrap_or(false);
                            let (proc, loc) = match state.executor.callstack.current_frame() {
                                Some(frame) => {
                                    let loc = frame
                                        .recent()
                                        .back()
                                        .and_then(|detail| detail.resolve(&state.session))
                                        .cloned();
                                    (frame.procedure(state.session.name()), loc)
                                }
                                None => (None, None),
                            };
                            (op, is_boundary, proc, loc)
                        }
                        None => (None, false, None, None),
                    };

                    // Remove all breakpoints triggered at this cycle
                    let current_cycle = state.executor.cycle;
                    let cycles_stepped = current_cycle - start_cycle;
                    breakpoints.retain_mut(|bp| {
                        if let Some(n) = bp.cycles_to_skip(current_cycle) {
                            if cycles_stepped >= n {
                                let retained = !bp.is_one_shot();
                                if retained {
                                    state.breakpoints_hit.push(bp.clone());
                                } else {
                                    state.breakpoints_hit.push(core::mem::take(bp));
                                }
                                return retained;
                            } else {
                                return true;
                            }
                        }

                        if cycles_stepped > 0
                            && is_op_boundary
                            && matches!(&bp.ty, BreakpointType::Next)
                        {
                            state.breakpoints_hit.push(core::mem::take(bp));
                            return false;
                        }

                        if let Some(loc) = loc.as_ref() {
                            if bp.should_break_at(loc) {
                                let retained = !bp.is_one_shot();
                                if retained {
                                    state.breakpoints_hit.push(bp.clone());
                                } else {
                                    state.breakpoints_hit.push(core::mem::take(bp));
                                }
                                return retained;
                            }
                        }

                        if let Some(proc) = proc.as_deref() {
                            if bp.should_break_in(proc) {
                                let retained = !bp.is_one_shot();
                                if retained {
                                    state.breakpoints_hit.push(bp.clone());
                                } else {
                                    state.breakpoints_hit.push(core::mem::take(bp));
                                }
                                return retained;
                            }
                        }

                        true
                    });

                    if consume_most_recent_finish {
                        if let Some(id) = breakpoints.iter().rev().find_map(|bp| {
                            if matches!(bp.ty, BreakpointType::Finish) {
                                Some(bp.id)
                            } else {
                                None
                            }
                        }) {
                            breakpoints.retain(|bp| bp.id != id);
                            break true;
                        }
                    }

                    if !state.breakpoints_hit.is_empty() {
                        break true;
                    }
                };

                // Restore the breakpoints state
                state.breakpoints = breakpoints;

                // Ensure that if we yield to the runtime, that we resume executing when
                // resumed, unless we specifically stopped for a breakpoint or other condition
                state.stopped = stopped;

                // Report program termination to the user
                if stopped && state.executor.stopped {
                    if let Some(err) = state.execution_failed.as_ref() {
                        actions.push(Some(Action::StatusLine(err.to_string())));
                    } else {
                        actions.push(Some(Action::StatusLine(
                            "program terminated successfully".to_string(),
                        )));
                    }
                }

                // Update the UI with latest state
                for pane in self.panes.iter_mut() {
                    actions.push(pane.update(Action::Update, state)?);
                }
            }
            Action::Reload => match state.reload() {
                Ok(_) => {
                    for pane in self.panes.iter_mut() {
                        actions.push(pane.update(Action::Reload, state)?);
                    }
                }
                Err(err) => {
                    actions.push(Some(Action::TimedStatusLine(err.to_string(), 5)));
                }
            },
            _ => {
                if let Some(pane) = self.panes.get_mut(self.focused_pane_index) {
                    actions.push(pane.update(action, state)?);
                }
            }
        }

        if let Some(tx) = &mut self.command_tx {
            actions.into_iter().flatten().for_each(|action| {
                tx.send(action).ok();
            });
        }
        Ok(None)
    }

    fn handle_key_events(
        &mut self,
        key: KeyEvent,
        state: &mut State,
    ) -> Result<Option<EventResponse<Action>>, Report> {
        match state.input_mode {
            InputMode::Normal => {
                let response = match key.code {
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                        EventResponse::Stop(Action::FocusNext)
                    }
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                        EventResponse::Stop(Action::FocusPrev)
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        EventResponse::Stop(Action::Down)
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        EventResponse::Stop(Action::Up)
                    }
                    KeyCode::Char('g') | KeyCode::Char('G') => EventResponse::Stop(Action::Go),
                    KeyCode::Backspace | KeyCode::Char('b') | KeyCode::Char('B') => {
                        EventResponse::Stop(Action::Back)
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        EventResponse::Stop(Action::ToggleFullScreen)
                    }
                    KeyCode::Char(c) if ('1'..='9').contains(&c) => {
                        EventResponse::Stop(Action::Tab(c.to_digit(10).unwrap_or(0) - 1))
                    }
                    KeyCode::Char(']') => EventResponse::Stop(Action::TabNext),
                    KeyCode::Char('[') => EventResponse::Stop(Action::TabPrev),
                    KeyCode::Char(':') => {
                        EventResponse::Stop(Action::FocusFooter(":".into(), None))
                    }
                    KeyCode::Char('q') => EventResponse::Stop(Action::Quit),
                    KeyCode::Char('e') => {
                        state.create_breakpoint(BreakpointType::Finish);
                        state.stopped = false;
                        EventResponse::Stop(Action::Continue)
                    }
                    // Only step if we're stopped, and execution has not terminated
                    KeyCode::Char('s') if state.stopped && !state.executor.stopped => {
                        state.create_breakpoint(BreakpointType::Step);
                        state.stopped = false;
                        EventResponse::Stop(Action::Continue)
                    }
                    // Only step-next if we're stopped, and execution has not terminated
                    KeyCode::Char('n') if state.stopped && !state.executor.stopped => {
                        state.create_breakpoint(BreakpointType::Next);
                        state.stopped = false;
                        EventResponse::Stop(Action::Continue)
                    }
                    // Only resume execution if we're stopped, and execution has not terminated
                    KeyCode::Char('c') if state.stopped && !state.executor.stopped => {
                        state.stopped = false;
                        EventResponse::Stop(Action::Continue)
                    }
                    // Do not try to continue if execution has terminated, but warn user
                    KeyCode::Char('c' | 's' | 'n') if state.stopped && state.executor.stopped => {
                        EventResponse::Stop(Action::TimedStatusLine(
                            "program has terminated, cannot continue".to_string(),
                            3,
                        ))
                    }
                    KeyCode::Char('d') => EventResponse::Stop(Action::Delete),
                    _ => {
                        return Ok(None);
                    }
                };
                Ok(Some(response))
            }
            InputMode::Insert => Ok(None),
            InputMode::Command => Ok(None),
        }
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, state: &State) -> Result<(), Report> {
        if let Some(fullscreen_pane_index) = self.fullscreen_pane_index {
            self.panes[fullscreen_pane_index].draw(frame, area, state)?;
        } else {
            let outer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(3), Constraint::Fill(1)])
                .split(area);

            let left_panes = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    self.panes[0].height_constraint(),
                    self.panes[1].height_constraint(),
                    self.panes[2].height_constraint(),
                ])
                .split(outer_layout[0]);

            let right_panes = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    self.panes[3].height_constraint(),
                    self.panes[4].height_constraint(),
                ])
                .split(outer_layout[1]);
            self.panes[0].draw(frame, left_panes[0], state)?;
            self.panes[1].draw(frame, left_panes[1], state)?;
            self.panes[2].draw(frame, left_panes[2], state)?;
            self.panes[3].draw(frame, right_panes[0], state)?;
            self.panes[4].draw(frame, right_panes[1], state)?;
        }
        Ok(())
    }
}
