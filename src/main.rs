mod file_handler;
mod project;

use project::{rust::RustProject, scan_project_deps, Project};

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    scan_project_deps(RustProject::default());
}
