/*
 * Copyright (C) Alexander Perkins, 2025
 *
 * This work is free.  You can redistribute it and/or modify it under the
 * terms of the Do What The Fuck You Want To But It's Not My Fault Public
 * License, Version 1, as published by Ben McGinnes.  See the
 * COPYING.WTFNMFPLv1a.txt file for more details.
 */

#![allow(unused_imports)]
#![allow(dead_code)]

use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, ProjectTypes},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create an instance of App and run it
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    // Restore terminal
    let _ = disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    let _ = terminal.show_cursor();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if key.code == KeyCode::Char('q') && app.current_screen != CurrentScreen::SelectProjectName && app.current_screen != CurrentScreen::SelectProjectFolder {
                return Ok(false);
            }

            match app.current_screen {
                CurrentScreen::Main => {
                    if key.code == KeyCode::Char('e') {
                        app.current_screen = CurrentScreen::SelectProjectType;
                    }
                }
                CurrentScreen::SelectProjectType => {
                    if key.code == KeyCode::Up {
                        match app.project_type {
                            ProjectTypes::Python => app.project_type = ProjectTypes::CmakeCpp,
                            ProjectTypes::UvPython => app.project_type = ProjectTypes::Python,
                            ProjectTypes::Rust => app.project_type = ProjectTypes::UvPython,
                            ProjectTypes::CmakeCpp => app.project_type = ProjectTypes::Rust,
                        }
                    }

                    if key.code == KeyCode::Down {
                        match app.project_type {
                            ProjectTypes::Python => app.project_type = ProjectTypes::UvPython,
                            ProjectTypes::UvPython => app.project_type = ProjectTypes::Rust,
                            ProjectTypes::Rust => app.project_type = ProjectTypes::CmakeCpp,
                            ProjectTypes::CmakeCpp => app.project_type = ProjectTypes::Python,
                        }
                    }

                    if key.code == KeyCode::Enter {
                        app.current_screen = CurrentScreen::SelectProjectName;
                    }
                }
                CurrentScreen::SelectProjectName => {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.text_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.text_input.pop();
                        }
                        KeyCode::Enter => {
                            if !app.text_input.is_empty() {
                                app.project_name = Some(app.text_input.clone());
                                app.current_screen = CurrentScreen::SelectProjectFolder;
                                app.text_input.clear();
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::SelectProjectFolder => {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.text_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.text_input.pop();
                        }
                        KeyCode::Enter => {
                            if !app.text_input.is_empty() {
                                app.project_folder = Some(app.text_input.clone());
                                app.current_screen = CurrentScreen::CreateProject;
                                app.text_input.clear();
                            }
                        }
                        _ => {}
                    }
                }
                _ => todo!("Impliment other screens"),
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         assert_eq!(1, 1);
//     }
// }
