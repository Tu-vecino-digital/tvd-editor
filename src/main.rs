use std::process::Command;

const PROJECT_GIT_URL: &str = "https://github.com/Tu-vecino-digital/tvd-web";

fn main() {
    clone_project();
}

fn clone_project() {
    let mut child = Command::new("git")
        .arg("clone")
        .arg(PROJECT_GIT_URL)
        .spawn()
        .expect("Failed launching git. Do you have it installed?");
    child.wait().expect("Failed waiting for git to exit.");
}
