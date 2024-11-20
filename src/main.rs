mod file_handler;
mod project;

use project::{node_js::NodeProject, run_project, Project};

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    run_project(NodeProject::default());
}
