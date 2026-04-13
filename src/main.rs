use std::{
    fs,
    path::PathBuf,
    process::Command,
    thread::{self, sleep},
    time::Duration,
};

use directories::UserDirs;

const PROJECT_GIT_URL: &str = "https://github.com/Tu-vecino-digital/tvd-web";
const PROJECT_FOLDER_NAME: &str = "tvd-web";

fn main() {
    let path = project_path();

    if project_already_exists(&path.project_path) {
        update_project(&path.project_path);
    } else {
        clone_project(&path.installation_path);
    }

    install_dependencies(&path.project_path);
    thread::spawn(|| {
        let one_second = Duration::from_secs(1);
        sleep(one_second);
        try_open_browser_tab();
    });
    start_project(&path.project_path);
}

struct ProjectPath {
    project_path: PathBuf,
    installation_path: PathBuf,
}

fn project_path() -> ProjectPath {
    let user_dirs = UserDirs::new().unwrap();
    let document_dir = user_dirs.document_dir().unwrap();

    ProjectPath {
        installation_path: document_dir.to_path_buf(),
        project_path: document_dir.join(PROJECT_FOLDER_NAME),
    }
}

fn try_open_browser_tab() {
    let url = "http://localhost:4321";
    webbrowser::open(url).expect(
        format!("Failed opening a web browser for you. Please open a new browser tab at {url}")
            .as_str(),
    );
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

fn clone_project(installation_path: &PathBuf) {
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
