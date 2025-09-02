![hackatime coding time badge](https://hackatime-badge.hackclub.com/U0825U4K39Q/project-creation)
![build status](https://img.shields.io/github/actions/workflow/status/Ghostboo124/project-creation/rust.yml?branch=main&event=push)
![Crates.io Version](https://img.shields.io/crates/v/project-creation-tui)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/project-creation-tui)
![GitHub Repo stars](https://img.shields.io/github/stars/Ghostboo124/project-creation?style=flat)

# Project Creation

My TUI for generating different types of projects (python, uv python, rust, cmake cpp are the goals)

## Demo

![demo video](https://www.youtube.com/embed/BvbK70bw50U?si=pW9IDg7G5RKGs8mX)

## Usage

> [!NOTE]
> If you are using an operating system that is not Microsoft Windows, then you will need to compile it manually

To use this, you can build it from [source](https://github.com/Ghostboo124/project-creation#building), or you can download it from the [releases page](https://github.com/Ghostboo124/project-creation/releases/latest) then you run the file or you can install it from [crates.io](https://crates.io/crates/project-creation-tui/) with `cargo install project-creation-tui` which installs it and adds it to path.

You will start of in the main menu, press (e) to continue to the project creation,
then you will be able to select the project type, input the project name and folder for the project to go into and confirm it, at any time other than during project name and folder name inputting you can press (q) to get out and you press (enter) to continue

## Building

To build this project you need to have [rust](https://www.rust-lang.org/tools/install) installed, after installing it you can run `cargo build --release` to compile the project to target/release/project-creation-tui or target\release\project-creation-tui.exe or you can run `cargo run --release` to compile and run the project

## Credits

So far this has been a solo job, though I will disclose that A.I. was used to help make this project because I was using the Cursor code editor and I used tab auto completions and the agent mode A.I. chat to help bugfix and learn new features as the whole TUI thing in rust is new to me.
