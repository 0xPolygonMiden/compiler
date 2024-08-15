use std::collections::VecDeque;

use crossterm::event::KeyCode;
use midenc_session::diagnostics::Report;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::{
    logger::{DebugLogger, LogEntry},
    ui::{
        action::Action,
        panes::Pane,
        state::{InputMode, State},
        tui::{EventResponse, Frame},
    },
};

pub struct DebugPane {
    logger: &'static DebugLogger,
    entries: VecDeque<LogEntry>,
    selected_entry: Option<usize>,
}
impl Default for DebugPane {
    fn default() -> Self {
        Self {
            logger: DebugLogger::get(),
            entries: Default::default(),
            selected_entry: None,
        }
    }
}

impl DebugPane {
    fn level_color(level: log::Level) -> Color {
        use log::Level;
        match level {
            Level::Trace => Color::LightCyan,
            Level::Debug => Color::LightMagenta,
            Level::Info => Color::LightGreen,
            Level::Warn => Color::LightYellow,
            Level::Error => Color::LightRed,
        }
    }
}

impl Pane for DebugPane {
    fn height_constraint(&self) -> Constraint {
        Constraint::Fill(3)
    }

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
        state: &mut State,
    ) -> Result<Option<EventResponse<Action>>, Report> {
        match state.input_mode {
            InputMode::Normal => {
                let response = match key.code {
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        EventResponse::Stop(Action::Down)
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        EventResponse::Stop(Action::Up)
                    }
                    KeyCode::Esc => EventResponse::Stop(Action::ClosePopup),
                    _ => {
                        return Ok(Some(EventResponse::Stop(Action::Noop)));
                    }
                };
                Ok(Some(response))
            }
            InputMode::Insert => Ok(Some(EventResponse::Stop(Action::Noop))),
            InputMode::Command => Ok(Some(EventResponse::Stop(Action::Noop))),
        }
    }

    fn update(&mut self, action: Action, _state: &mut State) -> Result<Option<Action>, Report> {
        let added = self.logger.take_captured();
        self.entries.extend(added);
        match action {
            Action::Down => {
                let selected_entry = self
                    .selected_entry
                    .map(|s| s.saturating_add(1) % self.entries.len())
                    .unwrap_or(self.entries.len().saturating_sub(1));
                self.selected_entry = Some(selected_entry);
                return Ok(Some(Action::Update));
            }
            Action::Up => {
                let selected_entry = self
                    .selected_entry
                    .map(|s| s.wrapping_sub(1) % self.entries.len())
                    .unwrap_or(self.entries.len().saturating_sub(1));
                self.selected_entry = Some(selected_entry);
                return Ok(Some(Action::Update));
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, _state: &State) -> Result<(), Report> {
        frame.render_widget(Clear, area);
        let items = self.entries.iter().map(|entry| {
            Line::from(vec![
                Span::styled(format!(" {:6} | ", entry.level), Self::level_color(entry.level)),
                Span::styled(entry.message.as_str(), Self::level_color(entry.level)),
            ])
        });
        let selected = if self.entries.is_empty() {
            None
        } else {
            Some(self.selected_entry.unwrap_or(self.entries.len().saturating_sub(1)))
        };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        let mut list_state = ListState::default().with_selected(selected);

        frame.render_stateful_widget(list, area, &mut list_state);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title("Debug Log")
                .style(Style::default()),
            area,
        );
        Ok(())
    }
}
