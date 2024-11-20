mod file_handler;
mod project;

use project::node_js;

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    node_js::run();
}
