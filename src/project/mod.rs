use std::collections::HashSet;

use colored::Colorize;
use walkdir::WalkDir;

use crate::file_handler::{read_file_at_path, string_exists_in_multiline_text};

pub mod node_js;
pub mod rust;

pub trait Project {
    const DEPS_FILE: &str;
    const ALLOWED_EXTENSIONS: &[&str];
    const EXCLUDED_PATHS: &[&str];

    fn parse_deps(&mut self, deps_file_content: &str) -> usize;
    fn deps(&self) -> &HashSet<String>;
}

pub fn scan_project_deps<T: Project>(mut project: T) {
    // TODO improve error managment
    let deps_count = project.parse_deps(&read_file_at_path(T::DEPS_FILE).unwrap());
    println!("{deps_count} packages found in current project.");
    let mut used_deps = HashSet::new();

    let mut scanned_file_count = 0usize;
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        if entry.path().is_dir() || !should_scan_file::<T>(entry.path().to_str().unwrap()) {
            continue;
        }

        scanned_file_count += 1;

        let buf = read_file_at_path(entry.path().to_str().unwrap()).unwrap();
        let mut used_deps_in_file = Vec::new();

        for dep_name in project.deps().iter() {
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

    for dep_name in project.deps().iter() {
        if !used_deps.contains(dep_name) {
            println!("Not used : {}", dep_name.red());
        }
    }

    println!("{} files scanned", scanned_file_count);
    println!("{} unused deps", deps_count - used_deps.len());
}

fn should_scan_file<T: Project>(file_path: &str) -> bool {
    if file_path == "." {
        return true;
    }

    for excluded in T::EXCLUDED_PATHS.iter() {
        if file_path.contains(excluded) {
            return false;
        }
    }

    for ext in T::ALLOWED_EXTENSIONS.iter() {
        if file_path.ends_with(&format!(".{ext}")) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{node_js::NodeProject, rust::RustProject, should_scan_file};

    #[test]
    fn node_js_should_scan_file() {
        assert_eq!(should_scan_file::<NodeProject>("foo.js"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.ts"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.tsx"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.jsx"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.json"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.scss"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.sass"), true);
        assert_eq!(should_scan_file::<NodeProject>("foo.rs"), false);
        assert_eq!(should_scan_file::<NodeProject>("foo.jssx"), false);
        assert_eq!(should_scan_file::<NodeProject>("package.json"), false);
        assert_eq!(
            should_scan_file::<NodeProject>("foo/node_modules/foo.ts"),
            false
        );
    }

    #[test]
    fn rust_should_scan_file() {
        assert_eq!(should_scan_file::<RustProject>("foo.rs"), true);
        assert_eq!(should_scan_file::<RustProject>("foo.rss"), false);
        assert_eq!(should_scan_file::<RustProject>("foo.js"), false);
        assert_eq!(should_scan_file::<RustProject>("Cargo.toml"), false);
        assert_eq!(should_scan_file::<RustProject>("Cargo.lock"), false);
        assert_eq!(should_scan_file::<RustProject>("foo.toml"), false);
    }
}
