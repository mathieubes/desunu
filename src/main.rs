mod file_handler;
mod project;

use std::collections::HashSet;

use file_handler::{read_file_at_path, string_exists_in_multiline_text};
use project::{
    node::{read_deps_file, NodeProject},
    Project,
};
use walkdir::WalkDir;

use colored::*;

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    let mut project = NodeProject::default();
    let deps_count = project.parse_deps(&read_deps_file());
    println!("{deps_count} packages found in current project.");
    let mut used_deps = HashSet::new();

    let mut scanned_file_count = 0usize;
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        if entry.path().is_dir() || !project.should_scan_file(entry.path().to_str().unwrap()) {
            continue;
        }

        scanned_file_count += 1;

        let buf = read_file_at_path(entry.path().to_str().unwrap()).unwrap();
        let mut used_deps_in_file = Vec::new();

        for dep_name in project.deps.iter() {
            if string_exists_in_multiline_text(dep_name, &buf) {
                used_deps_in_file.push(dep_name);
            }
        }

        let total_unused_deps_count = deps_count - used_deps.len();

        let print_str = format!(
            "==============================
> File : {}
> Deps found : {:?}
> Unused deps count : {}
==============================",
            entry.path().display(),
            used_deps_in_file,
            total_unused_deps_count
        );

        if used_deps_in_file.is_empty() {
            println!("{}", print_str.red());
        } else {
            println!("{}", print_str);
        }

        for dep_name in used_deps_in_file.into_iter() {
            used_deps.insert(dep_name);
        }
    }

    for dep_name in project.deps.iter() {
        if !used_deps.contains(dep_name) {
            println!("Not used : {}", dep_name.red());
        }
    }

    println!("{} files scanned", scanned_file_count);
    println!("{} unused deps", deps_count - used_deps.len());
}
