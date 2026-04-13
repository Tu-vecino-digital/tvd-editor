use std::process::Command;

const PROJECT_GIT_URL: &str = "https://github.com/Tu-vecino-digital/tvd-web";
const PROJECT_FOLDER_NAME: &str = "tvd-web";

fn main() {
    clone_project();
    install_dependencies();
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
