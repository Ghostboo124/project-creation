use std::collections::HashMap;
use std::process::{Command, ExitStatus};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    SelectProjectType,
    SelectProjectName,
    SelectProjectFolder,
    CreateProject,
    ProjectCreated,
}

/// All the different types of projects
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectTypes {
    Python,
    UvPython,
    Rust,
    CmakeCpp,
}

/// Struct containing important app data
pub struct App {
    /// The current project being created
    pub current_project: Option<String>,
    /// The type of project being created
    pub project_type: ProjectTypes,
    /// The name of the project being created
    pub project_name: Option<String>,
    /// The folder of the project being created
    pub project_folder: Option<String>,
    /// The command to be executed (stored as string)
    pub command: Option<String>,
    /// The current screen being displayed
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_project: None,
            project_type: ProjectTypes::Python,
            project_name: None,
            project_folder: None,
            command: None,
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn save_project(&mut self) {
        if let Some(project_folder) = &self.project_folder {
            if let Some(project_name) = &self.project_name {
                let mut command: String;
                match self.project_type {
                    ProjectTypes::Python => todo!("Impliment Python Project Creation"),
                    ProjectTypes::UvPython => {
                        command = format!("uv init --name {} --vcs git --app --no-description --author-from git {}", project_name, project_folder);
                    }
                    ProjectTypes::Rust => {
                        command = format!("cargo new --name {} --vcs git --bin --edition 2024 {}", project_name, project_folder);
                    }
                    ProjectTypes::CmakeCpp => todo!("Impliment Cmake Cpp Project Creation"),
                };

                // Store the command string
                self.command = Some(command);
            }
        }
    }

    pub fn create_project(&self) -> i32 {
        if let Some(command) = &self.command {
            let command_status = Command::new(command).status();

            match command_status {
                Ok(status) => {
                    if status.success() {
                        println!("Project created successfully");
                        0
                    } else {
                        println!("Error: Command failed with exit code {}", status.code().unwrap_or(-1));
                        status.code().unwrap_or(-1)
                    }
                }
                Err(_) => {
                    println!("Error: Failed to create project");
                    -1
                }
            }
        } else {
            -1
        }
    }
}
