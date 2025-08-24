use std::collections::HashMap;
use std::process::{Command, ExitStatus};
use std::fmt;
use regex::Regex;

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

    /// Execute the prepared shell command stored in `self.command` and return its exit status.
    ///
    /// If `self.command` is `None` this returns -1. When a command is present it is executed
    /// via `powershell -Command <command>` on Windows or `bash -c <command>` on other platforms.
    /// Standard output and standard error from the child process are discarded.
    ///
    /// Returns:
    /// - `0` if the child process exited successfully,
    /// - the child's exit code if it exited with a non-zero status and an exit code is available,
    /// - `-1` if the command failed to spawn/execute or if no command was set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = crate::App::new();
    /// app.command = Some(if cfg!(target_os = "windows") {
    ///     "exit 0".into()
    /// } else {
    ///     "exit 0".into()
    /// });
    /// let code = app.create_project();
    /// assert_eq!(code, 0);
    /// ```
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

    /// Replace any character in `self.text_input` that is not a word character or a hyphen with an underscore.
    ///
    /// This mutates the `App` in place by updating its `text_input` field. Useful for producing
    /// filesystem- and identifier-safe strings (letters, digits, underscore, and `-` are preserved).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = crate::App::new();
    /// app.text_input = "My Project!@# Name".into();
    /// app.sanitise_input();
    /// assert_eq!(app.text_input, "My_Project___Name");
    /// ```
    pub fn sanitise_input(&mut self) {
        let re = Regex::new(r"[^\w\-]").unwrap();
        self.text_input = re.replace_all(&self.text_input, "_").to_string();
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
