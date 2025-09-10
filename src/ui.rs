use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen, ProjectTypes};

pub fn ui(frame: &mut Frame, app: &App) {
    let whole = frame.area();

    // Very small terminal fallback: just show a compressed message
    if whole.width < 24 || whole.height < 10 {
        draw_tiny(frame, app, whole);
        return;
    }

    // Main vertical layout: content + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(whole);

    let borders = Paragraph::new("").block(Block::default().borders(Borders::ALL));
    frame.render_widget(borders, chunks[0]);

    // Footer (navigation + key hints)
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("No Projects", Style::default().fg(Color::Red)),
            CurrentScreen::SelectProjectType => Span::styled("Select Project", Style::default().fg(Color::Blue)),
            CurrentScreen::SelectProjectName => Span::styled("Select Name", Style::default().fg(Color::Blue)),
            CurrentScreen::SelectProjectFolder => Span::styled("Select Folder", Style::default().fg(Color::Blue)),
            CurrentScreen::CreateProject => Span::styled("Confirm Project", Style::default().fg(Color::Red)),
            CurrentScreen::ProjectCreated => Span::styled("Project Created", Style::default().fg(Color::Red)),
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = match app.current_screen {
        CurrentScreen::Main => Span::styled("(q) quit / (e) new project", Style::default().fg(Color::LightBlue)),
        CurrentScreen::SelectProjectName => Span::styled("(enter) continue", Style::default().fg(Color::LightBlue)),
        CurrentScreen::SelectProjectFolder => Span::styled("(enter) continue", Style::default().fg(Color::LightBlue)),
        _ => Span::styled("(q) quit / (enter) continue", Style::default().fg(Color::LightBlue)),
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // Main content based on current screen
    match app.current_screen {
        CurrentScreen::Main => {
            let area = safe_centred_rect(50, 10, chunks[0]);
            let text = Span::styled("Press 'e' to create a new project", Style::default().fg(Color::White));
            let para = Paragraph::new(Line::from(text))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(para, area);
        }
        CurrentScreen::SelectProjectType => {
            let area = safe_centred_rect(22, 60, chunks[0]);
            // Ensure minimum width for labels; fallback to full chunk if too small
            let area = if area.width < 14 { chunks[0] } else { area };
            let items_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(area);

                let mk = |label: &'static str, active: bool| {
                    Paragraph::new(Span::styled(label, Style::default().fg(Color::White)))
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL))
                        .style(if active { Style::default().bg(Color::Blue) } else { Style::default() })
                };

            frame.render_widget(mk("Python", app.project_type == ProjectTypes::Python), items_layout[0]);
            frame.render_widget(mk("Python + UV", app.project_type == ProjectTypes::UvPython), items_layout[1]);
            frame.render_widget(mk("Rust", app.project_type == ProjectTypes::Rust), items_layout[2]);
            frame.render_widget(mk("C++ (CMake)", app.project_type == ProjectTypes::CmakeCpp), items_layout[3]);
        }
        CurrentScreen::SelectProjectName => {
            let area = safe_centred_rect(60, 25, chunks[0]);
            render_input_box(
                frame,
                area,
                "Project Name",
                &app.text_input,
                true,
            );
        }
        CurrentScreen::SelectProjectFolder => {
            let area = safe_centred_rect(60, 25, chunks[0]);
            render_input_box(
                frame,
                area,
                "Project Folder",
                &app.text_input,
                true,
            );
        }
        CurrentScreen::CreateProject => {
            let area = safe_centred_rect(60, 40, chunks[0]);
            if let (Some(project_name), Some(project_folder)) = (&app.project_name, &app.project_folder) {
                let lines = vec![
                    Line::from(Span::styled(format!("Project Type: {}", app.project_type), Style::default().fg(Color::White))),
                    Line::from(Span::styled(format!("Project Name: {}", project_name), Style::default().fg(Color::White))),
                    Line::from(Span::styled(format!("Project Folder: {}", project_folder), Style::default().fg(Color::White))),
                    Line::from(Span::styled("Press (enter) to confirm", Style::default().fg(Color::White))),
                ];
                let para = Paragraph::new(lines)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Confirm"));
                frame.render_widget(para, area);
            }
        }
        CurrentScreen::ProjectCreated => {
            let area = safe_centred_rect(60, 25, chunks[0]);
            let lines = vec![
                Line::from(Span::styled("Project created successfully", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press any key to continue", Style::default().fg(Color::White))),
            ];
            let para = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Done"));
            frame.render_widget(para, area);
        }
    }
}

/// Safe centered rectangle that falls back if space is too small.
/// If resulting middle slice is too small to hold a bordered widget, returns the original r.
fn safe_centred_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let px = percent_x.min(100);
    let py = percent_y.min(100);

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    let mid_v = v[1];
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(mid_v);

    let candidate = h[1];
    // Need at least width 4 (border + 2 chars) & height 3 for useful content
    if candidate.width < 4 || candidate.height < 3 {
        r
    } else {
        candidate
    }
}

/// Render an input box with truncation showing the tail of the text if overflow occurs.
/// If focused, you could add a different style (e.g. blue border).
fn render_input_box(frame: &mut Frame, area: Rect, title: &str, value: &str, focused: bool) {
    // Keep the external sizing rule you had (cap and floor height)
    let area = Rect {
        height: area.height.min(5).max(3),
        ..area
    };

    // Build the block (with title & border style)
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title,
            Style::default().fg(if focused { Color::LightBlue } else { Color::White })
        ))
        .border_style(if focused {
            Style::default().fg(Color::LightBlue)
        } else {
            Style::default()
        });

    // First render the block itself
    frame.render_widget(block.clone(), area);

    // Compute inner drawable region (content area inside borders)
    let inner = block.inner(area);
    if inner.width == 0 || inner.height == 0 {
        return; // Nothing we can draw
    }

    // Prepare the (possibly truncated) tail of the input string
    let inner_width = inner.width as usize;
    let shown = if inner_width == 0 {
        String::new()
    } else if value.len() <= inner_width {
        value.to_string()
    } else if inner_width <= 1 {
        value.chars().rev().take(1).collect()
    } else if inner_width <= 3 {
        value.chars().rev().take(inner_width).collect::<String>().chars().rev().collect()
    } else {
        let tail_len = inner_width - 1;
        let tail: String = value.chars().rev().take(tail_len).collect::<String>().chars().rev().collect();
        format!("…{}", tail)
    };

    // Vertical centering: choose a single-row rectangle centered in 'inner'
    let text_y = if inner.height <= 1 {
        inner.y
    } else {
        inner.y + (inner.height - 1) / 2
    };
    let text_area = Rect {
        x: inner.x,
        y: text_y,
        width: inner.width,
        height: 1,
    };

    // Render the text line (no Block here, since we already rendered it)
    let para = Paragraph::new(shown.clone()).style(Style::default().fg(Color::White));
    frame.render_widget(para, text_area);

    // (Optional) Show a cursor at end of text if focused:
    if focused {
        let cursor_x = text_area.x.saturating_add(shown.len() as u16);
        frame.set_cursor(cursor_x.min(text_area.x + text_area.width.saturating_sub(1)), text_area.y);
    }
}

/// Extremely small terminal fallback rendering.
fn draw_tiny(frame: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        Line::from(Span::styled("Terminal too small", Style::default().fg(Color::Red))),
        Line::from(Span::styled(format!("{:?}", app.current_screen), Style::default().fg(Color::White))),
        Line::from(Span::styled("Resize to interact", Style::default().fg(Color::LightBlue))),
    ];
    let para = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para, area);
}