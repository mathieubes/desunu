mod file_handler;
mod project;

use project::{node_js::NodeProject, scan_project_deps, Project};

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    scan_project_deps(NodeProject::default());
}
