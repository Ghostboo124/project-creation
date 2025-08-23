use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen, ProjectTypes};

/*
pub enum CurrentScreen {
    Main,
    SelectProjectType,
    SelectProjectName,
    SelectProjectFolder,
    CreateProject,
    ProjectCreated,
}

pub enum ProjectTypes {
    Python,
    UvPython,
    Rust,
    CmakeCpp,
}
*/

pub fn ui(frame: &mut Frame, app: &App) {
    //    Main page
    // TODO: Impliment UI, see drawings for reference.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let borders = Paragraph::new("").block(Block::default().borders(Borders::ALL));
    frame.render_widget(borders, chunks[0]);

    //  Bottom Nav Bar
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("No Projects", Style::default().fg(Color::Red)),
            CurrentScreen::SelectProjectType => Span::styled("Select Project", Style::default().fg(Color::Blue)),
            CurrentScreen::SelectProjectName => Span::styled("Select Name", Style::default().fg(Color::Black)),
            CurrentScreen::SelectProjectFolder => Span::styled("Select Folder", Style::default().fg(Color::Black)),
            CurrentScreen::CreateProject => Span::styled("Confirm Project", Style::default().fg(Color::Red)),
            CurrentScreen::ProjectCreated => Span::styled("Project Created", Style::default().fg(Color::Red)),
        }
        .to_owned()
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) quit / (e) create new project",
                Style::default().fg(Color::LightBlue)),
            _ => Span::styled(
                "(q) quit / (enter) continue",
                Style::default().fg(Color::LightBlue)),

        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
    
    match app.current_screen {
        CurrentScreen::Main => {
            let main_area = centred_rect(50, 10, frame.area());
            let main_text = Span::styled("Press 'e' to create a new project", Style::default().fg(Color::White));
            let main_paragraph = Paragraph::new(Line::from(main_text))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(main_paragraph, main_area);
        }
        CurrentScreen::SelectProjectType => {
            let project_type_area = centred_rect(10, 50, chunks[0]);
            let project_type_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(project_type_area);

            let python_text = Paragraph::new(Span::styled(
                "Python",
                Style::default().fg(Color::White)
            ))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(if app.project_type == ProjectTypes::Python {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default()
                });
            let uv_python_text = Paragraph::new(Span::styled(
                "Python with UV",
                Style::default().fg(Color::White)
            ))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(if app.project_type == ProjectTypes::UvPython {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default()
                });
            let rust_text = Paragraph::new(Span::styled(
                "Rust",
                Style::default().fg(Color::White)
            ))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(if app.project_type == ProjectTypes::Rust {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default()
                });
            let cmake_cpp_text = Paragraph::new(Span::styled(
                "C++ with CMake",
                Style::default().fg(Color::White)
            ))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(if app.project_type == ProjectTypes::CmakeCpp {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default()
                });

            // Render text widgets normally
            frame.render_widget(python_text, project_type_chunks[0]);
            frame.render_widget(uv_python_text, project_type_chunks[1]);
            frame.render_widget(rust_text, project_type_chunks[2]);
            frame.render_widget(cmake_cpp_text, project_type_chunks[3]);
        }
        _ => todo!("Impliment other UI screens"),
    }


}

fn centred_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rect into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle verticle piece into width-wide pieces and return the middle piece
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}