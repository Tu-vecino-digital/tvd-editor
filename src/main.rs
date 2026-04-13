use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use directories::UserDirs;

const PROJECT_GIT_URL: &str = "https://github.com/Tu-vecino-digital/tvd-web";
const PROJECT_FOLDER_NAME: &str = "tvd-web";

fn main() {
    let user_dirs = UserDirs::new().unwrap();
    let document_dir = user_dirs.document_dir().unwrap();
    let project_path = document_dir.join(PROJECT_FOLDER_NAME);

    if project_already_exists(&project_path) {
        update_project(&project_path);
    } else {
        clone_project(document_dir);
    }
    install_dependencies(&project_path);
    start_project(&project_path);
}

fn project_already_exists(project_path: &PathBuf) -> bool {
    fs::exists(project_path).expect(
        "Failed checking if project folder already exists. Could it be missing permissions?",
    )
}

fn update_project(project_path: &PathBuf) {
    let status = Command::new("git")
        .current_dir(project_path)
        .arg("pull")
        .status()
        .expect("Failed launching git. Do you have it installed?");
    assert!(status.success(), "Failed updating project.");
}

fn clone_project(installation_path: &Path) {
    let status = Command::new("git")
        .current_dir(installation_path)
        .arg("clone")
        .arg(PROJECT_GIT_URL)
        .status()
        .expect("Failed launching git. Do you have it installed?");
    assert!(status.success(), "Failed cloning repository.");
}

fn install_dependencies(project_path: &PathBuf) {
    let status = Command::new("bun")
        .current_dir(project_path)
        .arg("install")
        .status()
        .expect("Failed launching bun. Do you have it installed?");
    assert!(status.success(), "Failed during dependency installation.");
}

fn start_project(project_path: &PathBuf) {
    Command::new("bun")
        .current_dir(project_path)
        .arg("run")
        .arg("dev")
        .status()
        .expect("Failed starting project.");
}
