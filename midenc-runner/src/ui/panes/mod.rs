use midenc_session::diagnostics::Report;
use ratatui::{
    crossterm::event::{self, KeyEvent, MouseEvent},
    layout::{Constraint, Rect},
};

use super::{
    action::Action,
    state::State,
    tui::{Event, EventResponse, Frame},
};

pub mod breakpoints;
pub mod disasm;
pub mod footer;
pub mod header;
pub mod source_code;
pub mod stack;
pub mod stacktrace;

pub trait Pane {
    fn init(&mut self, _state: &State) -> Result<(), Report> {
        Ok(())
    }

    fn height_constraint(&self) -> Constraint;

    fn handle_events(
        &mut self,
        event: Event,
        state: &mut State,
    ) -> Result<Option<EventResponse<Action>>, Report> {
        let r = match event {
            Event::Key(key_event) => self.handle_key_events(key_event, state)?,
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event, state)?,
            _ => None,
        };
        Ok(r)
    }

    fn handle_key_events(
        &mut self,
        _key: KeyEvent,
        _state: &mut State,
    ) -> Result<Option<EventResponse<Action>>, Report> {
        Ok(None)
    }

    fn handle_mouse_events(
        &mut self,
        _mouse: MouseEvent,
        _state: &mut State,
    ) -> Result<Option<EventResponse<Action>>, Report> {
        Ok(None)
    }

    fn update(&mut self, _action: Action, _state: &mut State) -> Result<Option<Action>, Report> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<(), Report>;
}
