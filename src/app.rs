use std::collections::HashMap;
use std::process::{Command, ExitStatus};
use std::fmt;

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
    /// The text input for the current screen
    pub text_input: String,
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
            text_input: String::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn save_project(&mut self) {
        if let Some(project_folder) = &self.project_folder {
            if let Some(project_name) = &self.project_name {
                #[allow(unused_mut)]
                let mut command: String;
                match self.project_type {
                    ProjectTypes::Python => {
                        if cfg!(target_os = "windows") {
                            command = format!(
                                "New-Item -ItemType Directory -Name {} -Force; Set-Location {}; python3 -m venv .venv; '# {}' | Out-File -FilePath README.md -Encoding UTF8; New-Item -ItemType Directory -Name src -Force; Set-Location src; 'def main():' | Out-File -FilePath main.py -Encoding UTF8; '    print(\"Hello, World!\")' | Add-Content -Path main.py; '' | Add-Content -Path main.py; 'main()' | Add-Content -Path main.py; Set-Location ..; .venv\\Scripts\\Activate.ps1",
                                project_folder,
                                project_folder,
                                project_name
                            );
                        } else {
                            command = format!(
                                "mkdir {} && cd {} && python3 -m venv .venv && echo # {} > README.md && mkdir src && cd src && echo 'def main():\n  print(\"Hello, World!\")' > main.py && cd .. && source .venv/bin/activate",
                                project_folder,
                                project_folder,
                                project_name
                            );
                        }
                    }
                    ProjectTypes::UvPython => {
                        command = format!("uv init --name {} --vcs git --app --no-description --author-from git {}", project_name, project_folder);
                    }
                    ProjectTypes::Rust => {
                        command = format!("cargo new --name {} --vcs git --bin --edition 2024 {}", project_name, project_folder);
                    }
                    ProjectTypes::CmakeCpp => {
                        if cfg!(target_os = "windows") {
                            command = format!(
                                "New-Item -ItemType Directory -Name {} -Force; Set-Location {}; '# {}' | Out-File -FilePath README.md -Encoding UTF8; New-Item -ItemType Directory -Name src -Force; Set-Location src; '#include <iostream>' | Out-File -FilePath main.cpp -Encoding UTF8; 'int main() {{' | Add-Content -Path main.cpp; '    std::cout << \"Hello, World!\" << std::endl;' | Add-Content -Path main.cpp; '    return 0;' | Add-Content -Path main.cpp; '}}' | Add-Content -Path main.cpp; Set-Location ..; 'cmake_minimum_required(VERSION 3.10)' | Out-File -FilePath CMakeLists.txt -Encoding UTF8; 'project({} CXX)' | Add-Content -Path CMakeLists.txt; 'set(CMAKE_CXX_STANDARD 17)' | Add-Content -Path CMakeLists.txt; 'set(CMAKE_CXX_STANDARD_REQUIRED ON)' | Add-Content -Path CMakeLists.txt; 'if(WIN32)' | Add-Content -Path CMakeLists.txt; '    set(CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}} -fexceptions\")' | Add-Content -Path CMakeLists.txt; '    set(CMAKE_C_FLAGS \"${{CMAKE_C_FLAGS}} -fexceptions\")' | Add-Content -Path CMakeLists.txt; 'endif()' | Add-Content -Path CMakeLists.txt; 'add_executable(${{PROJECT_NAME}} src/main.cpp)' | Add-Content -Path CMakeLists.txt",
                                project_folder,
                                project_folder,
                                project_name,
                                project_name
                            );
                        } else {
                            command = format!(
                                "mkdir {} && cd {} && echo # {} > README.md && mkdir src && cd src && echo '#include <iostream>' > main.cpp && echo 'int main() {{' >> main.cpp && echo '    std::cout << \"Hello, World!\" << std::endl;' >> main.cpp && echo '    return 0;' >> main.cpp && echo '}}' >> main.cpp && cd .. && echo 'cmake_minimum_required(VERSION 3.10)' > CMakeLists.txt && echo 'project({})' >> CMakeLists.txt && echo 'set(CMAKE_CXX_STANDARD 17)' >> CMakeLists.txt && echo 'set(CMAKE_CXX_STANDARD_REQUIRED ON)' >> CMakeLists.txt && echo 'add_executable(${{PROJECT_NAME}} src/main.cpp)' >> CMakeLists.txt",
                                project_folder,
                                project_folder,
                                project_name,
                                project_name
                            );
                        }
                    }
                };

                // Store the command string
                self.command = Some(command);
            }
        }
    }

    pub fn create_project(&self) -> i32 {
        if let Some(command) = &self.command {
            let command_status = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(&["-Command", command])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
            } else {
                Command::new("bash")
                    .args(&["-c", command])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
            };

            match command_status {
                Ok(status) => {
                    if status.success() {
                        0
                    } else {
                        status.code().unwrap_or(-1)
                    }
                }
                Err(_) => {
                    -1
                }
            }
        } else {
            -1
        }
    }
}

impl fmt::Display for ProjectTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectTypes::Python => write!(f, "Python"),
            ProjectTypes::UvPython => write!(f, "Python with UV"),
            ProjectTypes::Rust => write!(f, "Rust"),
            ProjectTypes::CmakeCpp => write!(f, "C++ with CMake"),
        }
    }
}
