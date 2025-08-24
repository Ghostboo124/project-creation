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
    backend::{Backend, CrosstermBackend}, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, Terminal
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

/// Runs the main terminal UI event loop, drawing the UI and updating `app` in response to user input.
///
/// This function drives the application's state machine: it repeatedly draws the UI to `terminal`,
/// reads keyboard events, and mutates `app` (screen, inputs, project data) according to the current
/// `CurrentScreen`. It also triggers project persistence and creation when the flow reaches
/// `CreateProject`.
///
/// Important behaviors:
/// - Pressing 'q' will exit the loop (returns `Ok(false`) unless the user is currently entering a
///   project name or folder (those screens ignore 'q').
/// - While on text-entry screens (`SelectProjectName`, `SelectProjectFolder`) printable characters
///   are appended to `app.text_input`, Backspace removes the last character, and Enter will
///   sanitise and commit the input if non-empty, advancing the flow.
/// - On `CreateProject`, Enter calls `app.save_project()` and `app.create_project()` and advances to
///   `ProjectCreated`.
/// - The function returns an `io::Result<bool>` to surface terminal I/O errors; the returned `bool`
///   signals whether the app requested to exit (calling code treats `false` as "quit").
///
/// Parameters:
/// - `terminal`: the TUI terminal used for drawing (mutated for each frame).
/// - `app`: the application state to update in response to events.
///
/// # Examples
///
/// ```no_run
/// use ratatui::Terminal;
/// // create terminal and app appropriately...
/// // let mut terminal: Terminal<...> = ...;
/// // let mut app = App::new();
/// // run the UI loop (propagates I/O errors)
/// // let _ = run_app(&mut terminal, &mut app);
/// ```
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
                                app.sanitise_input();
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
                                app.sanitise_input();
                                app.project_folder = Some(app.text_input.clone());
                                app.current_screen = CurrentScreen::CreateProject;
                                app.text_input.clear();
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::CreateProject => {
                    if key.code == KeyCode::Enter {
                        app.save_project();
                        app.create_project();
                        app.current_screen = CurrentScreen::ProjectCreated;
                    }
                }
                CurrentScreen::ProjectCreated => {
                    match key.code {
                        _ => {
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                }
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
