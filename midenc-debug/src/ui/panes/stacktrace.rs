use midenc_session::diagnostics::{Report, SourceId, SourceSpan};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::{
    ui::{action::Action, panes::Pane, state::State, tui::Frame},
    ResolvedLocation,
};

pub struct StackTracePane {
    focused: bool,
    focused_border_style: Style,
}

impl StackTracePane {
    pub fn new(focused: bool, focused_border_style: Style) -> Self {
        Self {
            focused,
            focused_border_style,
        }
    }

    fn border_style(&self) -> Style {
        match self.focused {
            true => self.focused_border_style,
            false => Style::default(),
        }
    }

    fn border_type(&self) -> BorderType {
        match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        }
    }
}

impl Pane for StackTracePane {
    fn height_constraint(&self) -> Constraint {
        match self.focused {
            true => Constraint::Max(15),
            false => Constraint::Max(15),
        }
    }

    fn update(&mut self, action: Action, _state: &mut State) -> Result<Option<Action>, Report> {
        match action {
            Action::Focus => {
                self.focused = true;
            }
            Action::UnFocus => {
                self.focused = false;
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, state: &State) -> Result<(), Report> {
        let mut lines = Vec::default();
        let num_frames = state.execution_state.callstack.frames().len();
        for (i, frame) in state.execution_state.callstack.frames().iter().enumerate() {
            let is_top = i + 1 == num_frames;
            let mut parts = vec![];
            /*
            let gutter = if is_top {
                Span::styled(" `-> ", Color::Magenta)
            } else {
                Span::styled(" |-> ", Color::Gray)
            };
            */
            let gutter = Span::styled(" ", Color::White);
            parts.push(gutter);
            let name = frame.procedure(state.session.name());
            let name = name.as_deref().unwrap_or("<unknown>").to_string();
            let name = if is_top {
                Span::styled(name, Color::Gray)
            } else {
                Span::styled(
                    name,
                    Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD),
                )
            };
            parts.push(name);
            if let Some(resolved) = frame.last_resolved(&state.session) {
                parts.push(Span::styled(" in ", Color::DarkGray));
                let path = resolved.source_file.path();
                let path = path
                    .strip_prefix(state.session.options.current_dir.as_path())
                    .ok()
                    .unwrap_or(path);
                let path_str = path.to_string_lossy();
                let max_width = (area.as_size().width as usize).saturating_sub(4);
                let path_width = path_str.chars().count();
                if path_width >= max_width {
                    let trim_min = path_width - max_width;
                    let mut taken = 0;
                    let mut components = path.components();
                    while taken < trim_min {
                        match components.next() {
                            Some(std::path::Component::CurDir) => break,
                            Some(
                                std::path::Component::ParentDir
                                | std::path::Component::Prefix(_)
                                | std::path::Component::RootDir,
                            ) => continue,
                            Some(std::path::Component::Normal(c)) => {
                                let c = c.to_string_lossy();
                                taken += c.chars().count();
                            }
                            None => break,
                        }
                    }
                    parts.push(Span::styled(
                        format!("{}", components.as_path().display()),
                        Color::Cyan,
                    ));
                } else {
                    parts.push(Span::styled(path_str, Color::Cyan));
                }
                parts.push(Span::styled(
                    format!(" {}:{}", resolved.line, resolved.col),
                    Color::Green,
                ));
            } else {
                parts.push(Span::styled(" in <unknown>", Color::DarkGray));
            }
            lines.push(Line::from(parts));
        }

        let selected_line = lines.len().saturating_sub(1);

        let list = List::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        let mut list_state = ListState::default().with_selected(Some(selected_line));

        frame.render_stateful_widget(list, area, &mut list_state);
        frame.render_widget(
            Block::default()
                .title("Stack Trace")
                .borders(Borders::ALL)
                .border_style(self.border_style())
                .border_type(self.border_type()),
            area,
        );
        Ok(())
    }
}
