use std::{fs, process::Command};

const PROJECT_GIT_URL: &str = "https://github.com/Tu-vecino-digital/tvd-web";
const PROJECT_FOLDER_NAME: &str = "tvd-web";

fn main() {
    if project_already_exists() {
        update_project();
    } else {
        clone_project();
    }
    install_dependencies();
    start_project();
}

fn project_already_exists() -> bool {
    fs::exists(PROJECT_FOLDER_NAME).expect(
        "Failed checking if project folder already exists. Could it be missing permissions?",
    )
}

fn update_project() {
    let status = Command::new("git")
        .current_dir(PROJECT_FOLDER_NAME)
        .arg("pull")
        .status()
        .expect("Failed launching git. Do you have it installed?");
    assert!(status.success(), "Failed updating project.");
}

fn clone_project() {
    let status = Command::new("git")
        .arg("clone")
        .arg(PROJECT_GIT_URL)
        .status()
        .expect("Failed launching git. Do you have it installed?");
    assert!(status.success(), "Failed cloning repository.");
}

fn install_dependencies() {
    let status = Command::new("bun")
        .current_dir(PROJECT_FOLDER_NAME)
        .arg("install")
        .status()
        .expect("Failed launching bun. Do you have it installed?");
    assert!(status.success(), "Failed during dependency installation.");
}

fn start_project() {
    Command::new("bun")
        .current_dir(PROJECT_FOLDER_NAME)
        .arg("run")
        .arg("dev")
        .status()
        .expect("Failed starting project.");
}
