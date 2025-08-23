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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentlyEditing {
    None,
    Name,
    Folder,
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
    pub current_project: String,
    /// The type of project being created
    pub project_type: ProjectTypes,
    /// The name of the project being created
    pub project_name: String,
    /// The folder of the project being created
    pub project_folder: String,
    /// The command to be executed (stored as string)
    pub command: String,
    /// The current screen being displayed
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_project: String::new(),
            project_type: ProjectTypes::Python,
            project_name: String::new(),
            project_folder: String::new(),
            command: String::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn save_project(&mut self) {
        let project_path = format!("{}", self.project_folder);
        let mut command: String = String::new();
        match self.project_type {
            ProjectTypes::Python => todo!("Impliment Python Project Creation"),
            ProjectTypes::UvPython => {
                command = format!("uv init --name {} --vcs git --app --no-description --author-from git {}", self.project_name, self.project_folder);
            }
            ProjectTypes::Rust => {
                command = format!("cargo new --name {} --vcs git --bin --edition 2024 {}", self.project_name, self.project_folder);
            }
            ProjectTypes::CmakeCpp => todo!("Impliment Cmake Cpp Project Creation"),
        };

        // Store the command string
        self.command = command;
    }

    pub fn create_project(&self) -> i32 {
        let command_status = Command::new(self.command.clone()).status();

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
    }
}
