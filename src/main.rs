mod file_handler;
mod project;

use clap::{Parser, Subcommand};
use project::{node_js::NodeProject, rust::RustProject, scan_project_deps, Project};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    All,
    NodeJS,
    Rust,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::All) => {
            todo!("Add project detection with the configured package name constant.")
        },
        Some(Commands::NodeJS) => scan_project_deps(NodeProject::default()),
        Some(Commands::Rust) => scan_project_deps(RustProject::default()),
        None => {}
    }
}
