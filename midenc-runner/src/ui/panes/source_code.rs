use midenc_session::diagnostics::{Report, SourceId, SourceSpan};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::{
    ui::{action::Action, panes::Pane, state::State, tui::Frame},
    ResolvedLocation,
};

pub struct SourceCodePane {
    focused: bool,
    focused_border_style: Style,
    current_source_id: SourceId,
    current_span: SourceSpan,
    current_line: u32,
    current_col: u32,
    num_lines: u32,
    selected_line: u32,
}

impl SourceCodePane {
    pub fn new(focused: bool, focused_border_style: Style) -> Self {
        Self {
            focused,
            focused_border_style,
            current_source_id: SourceId::UNKNOWN,
            num_lines: 0,
            selected_line: 0,
            current_line: 0,
            current_col: 0,
            current_span: SourceSpan::default(),
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

impl Pane for SourceCodePane {
    fn init(&mut self, state: &State) -> Result<(), Report> {
        if let Some(frame) = state.execution_state.callstack.current_frame() {
            if let Some(loc) = frame.last_resolved(&state.session) {
                self.current_source_id = loc.source_file.id();
                self.current_span = loc.span;
                self.current_line = loc.line;
                self.current_col = loc.col;
                self.num_lines = loc.source_file.line_count() as u32;
                self.selected_line = loc.line;
            }
        }

        Ok(())
    }

    fn height_constraint(&self) -> Constraint {
        match self.focused {
            true => Constraint::Fill(3),
            false => Constraint::Fill(3),
        }
    }

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>, Report> {
        match action {
            Action::Down => {
                if self.num_lines > 0 {
                    self.selected_line = self.selected_line.saturating_add(1) % self.num_lines;
                }
                return Ok(Some(Action::Update));
            }
            Action::Up => {
                if self.num_lines > 0 {
                    self.selected_line =
                        self.selected_line.saturating_add(self.num_lines.saturating_sub(1))
                            % self.num_lines;
                }
                return Ok(Some(Action::Update));
            }
            Action::Focus => {
                self.focused = true;
                static STATUS_LINE: &str = "[j,k → movement]";
                return Ok(Some(Action::TimedStatusLine(STATUS_LINE.into(), 3)));
            }
            Action::UnFocus => {
                self.focused = false;
            }
            Action::Submit => {}
            Action::Update => {
                if let Some(frame) = state.execution_state.callstack.current_frame() {
                    if let Some(loc) = frame.last_resolved(&state.session) {
                        let source_id = loc.source_file.id();
                        if source_id != self.current_source_id {
                            self.current_source_id = source_id;
                            self.num_lines = loc.source_file.line_count() as u32;
                            self.selected_line = loc.line;
                        } else if self.selected_line != loc.line {
                            self.selected_line = loc.line;
                        }
                        self.current_span = loc.span;
                        self.current_line = loc.line;
                        self.current_col = loc.col;
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, state: &State) -> Result<(), Report> {
        let resolved = match state.execution_state.callstack.current_frame() {
            Some(frame) => {
                let resolved = frame.last_resolved(&state.session);
                resolved.cloned()
            }
            None if !self.current_source_id.is_unknown() => {
                let source_file = state.session.source_manager.get(self.current_source_id).ok();
                source_file.map(|src| ResolvedLocation {
                    source_file: src,
                    line: self.current_line,
                    col: self.current_col,
                    span: self.current_span,
                })
            }
            None => {
                // Render empty source pane
                None
            }
        };

        if resolved.is_none() {
            frame.render_widget(
                Block::default()
                    .title("Source Code")
                    .borders(Borders::ALL)
                    .border_style(self.border_style())
                    .border_type(self.border_type())
                    .title_bottom(
                        Line::from("no source code available for current instruction")
                            .right_aligned(),
                    )
                    .title(
                        Line::styled("nofile", Style::default().add_modifier(Modifier::ITALIC))
                            .right_aligned(),
                    ),
                area,
            );
            return Ok(());
        }

        let (start_line, lines) = match resolved.as_ref() {
            None => (
                0,
                vec![Line::from(vec![
                    Span::styled("0 | ", Color::Gray),
                    Span::styled(
                        format!(
                            "No resolved location for at cycle {}",
                            state.execution_state.cycle
                        ),
                        Color::White,
                    ),
                ])],
            ),
            Some(resolved) => {
                let resolved_span = resolved.span.into_slice_index();
                let content = resolved.source_file.content();
                let last_line = content.last_line_index();
                let max_line_no = last_line.number().get() as usize;
                let gutter_width = max_line_no.ilog10() as usize;
                /*
                let line_context_start = core::cmp::max(self.selected_line.saturating_sub(15), 0);
                let line_context_end = core::cmp::min(
                    self.selected_line.saturating_add(15),
                    last_line.to_usize() as u32,
                );
                */
                //let lines = (line_context_start..line_context_end)
                let lines = (0..(max_line_no - 1))
                    .map(|line_index| {
                        let line_index = miden_core::debuginfo::LineIndex::from(line_index as u32);
                        let line_no = line_index.number().get();
                        let span = content.line_range(line_index).expect("invalid line index");
                        let span = span.start.to_usize()..span.end.to_usize();
                        // Only highlight a portion of the line if the full span fits on that line
                        let is_highlighted = span.contains(&resolved_span.start)
                            && span.contains(&resolved_span.end)
                            && span != resolved_span;
                        if is_highlighted {
                            let prefix_content = String::from_utf8_lossy(
                                &content.as_bytes()[span.start..resolved_span.start],
                            );
                            let highlight_content = &content.as_bytes()[resolved_span.clone()];
                            let suffix_content = &content.as_bytes()[resolved_span.end..span.end];
                            let (highlight_content, suffix_content) = if suffix_content.is_empty() {
                                (
                                    strip_newline(highlight_content),
                                    String::from_utf8_lossy(suffix_content),
                                )
                            } else {
                                (
                                    String::from_utf8_lossy(highlight_content),
                                    strip_newline(suffix_content),
                                )
                            };
                            let highlight_style = Style::default()
                                .fg(Color::Black)
                                .bg(Color::White)
                                .add_modifier(Modifier::BOLD);
                            Line::from(vec![
                                Span::styled(
                                    format!("{line_no:gutter_width$} | ",),
                                    highlight_style,
                                ),
                                Span::styled(prefix_content, highlight_style),
                                Span::styled(
                                    highlight_content,
                                    Style::default()
                                        .fg(Color::White)
                                        .bg(Color::DarkGray)
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(suffix_content, highlight_style),
                            ])
                        } else {
                            let line_content =
                                strip_newline(&content.as_bytes()[span.start..span.end]);
                            Line::from(vec![
                                Span::styled(format!("{line_no:gutter_width$} | ",), Color::Gray),
                                Span::styled(line_content, Color::White),
                            ])
                        }
                    })
                    .collect();
                (0, lines)
            }
        };
        let filename = resolved
            .as_ref()
            .map(|loc| loc.source_file.path().display().to_string())
            .unwrap_or_else(|| "nofile".to_string());

        let selected_line = self.selected_line.saturating_sub(1);
        let list = List::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
            .highlight_spacing(HighlightSpacing::Always);
        //.highlight_style(
        //Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD),
        //);
        let mut list_state = ListState::default().with_selected(Some(selected_line as usize));

        frame.render_stateful_widget(list, area, &mut list_state);
        frame.render_widget(
            Block::default()
                .title("Source Code")
                .borders(Borders::ALL)
                .border_style(self.border_style())
                .border_type(self.border_type())
                .title_bottom(
                    Line::from(format!("{} of {}", self.selected_line, self.num_lines,))
                        .right_aligned(),
                )
                .title(
                    Line::styled(&filename, Style::default().add_modifier(Modifier::ITALIC))
                        .right_aligned(),
                ),
            area,
        );
        Ok(())
    }
}

fn strip_newline(s: &[u8]) -> std::borrow::Cow<'_, str> {
    if let Some(sans_newline) = s.strip_suffix(b"\n") {
        String::from_utf8_lossy(sans_newline)
    } else {
        String::from_utf8_lossy(s)
    }
}
