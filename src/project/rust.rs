use std::{env, fs::File};

use serde::Deserialize;
use toml::Table;

use crate::file_handler::read_file;

use super::Project;

const DEPS_FILE: &str = "Cargo.toml";

#[derive(Deserialize)]
pub struct RustPackagesHandler {
    dependencies: Table,
}

pub struct RustProject {
    deps: Vec<String>,

    allowed_extensions: Vec<String>,
    excluded_paths: Vec<String>,
}

impl RustProject {
    fn new(allowed_extensions: Vec<String>, excluded_paths: Vec<String>) -> Self {
        Self {
            allowed_extensions,
            excluded_paths,
            deps: Vec::new(),
        }
    }
}

impl Project for RustProject {
    fn default() -> Self {
        Self::new(
            Vec::from(["rs".into()]),
            // For now, excluding Cargo.toml file is not necessary but if in the future we need to
            // include .toml files then we can't miss it.
            Vec::from(["Cargo.toml".into()]),
        )
    }

    fn parse_deps(&mut self, deps_file_content: &str) -> usize {
        let packages_handler: RustPackagesHandler = toml::from_str(deps_file_content)
            .unwrap_or_else(|e| panic!("Cannot parse {DEPS_FILE} file. {}", e));
        self.deps = get_deps_names(packages_handler);
        self.deps.len()
    }

    // TODO should_scan_file functions are exactly the same for both projects
    fn should_scan_file(&self, file_path: &str) -> bool {
        if file_path == "." {
            return true;
        }

        for excluded in self.excluded_paths.iter() {
            if file_path.contains(excluded) {
                return false;
            }
        }
        for ext in self.allowed_extensions.iter() {
            if file_path.ends_with(&format!(".{ext}")) {
                return true;
            }
        }
        false
    }

    fn deps(&self) -> &Vec<String> {
        &self.deps
    }

    // TODO Always same function, need to be refactored
    fn read_deps_file() -> String {
        let f = File::open(DEPS_FILE).unwrap_or_else(|_| {
            panic!(
                "No file \"{DEPS_FILE}\" in {}",
                env::current_dir().unwrap().display()
            )
        });
        read_file(f).unwrap_or_else(|_| panic!("Cannot read {DEPS_FILE} file."))
    }
}

fn get_deps_names(parsed_file: RustPackagesHandler) -> Vec<String> {
    let mut names = Vec::from_iter(
        parsed_file
            .dependencies
            .iter()
            .map(|(name, _version)| name.clone()),
    );
    names.sort();
    names
}

#[cfg(test)]
mod project_rust_tests {
    use super::*;

    #[test]
    fn should_scan_file_works() {
        let project = RustProject::default();
        assert_eq!(project.should_scan_file("foo.rs"), true);
        assert_eq!(project.should_scan_file("foo.js"), false);
        assert_eq!(project.should_scan_file("Cargo.toml"), false);
        assert_eq!(project.should_scan_file("Cargo.lock"), false);
    }

    #[test]
    fn get_deps_names_works() {
        let mut packages_handler = RustPackagesHandler {
            dependencies: Table::new(),
        };
        packages_handler
            .dependencies
            .insert("foo".into(), "0.1.0".into());
        packages_handler
            .dependencies
            .insert("bar".into(), "0.1.0".into());

        assert_eq!(get_deps_names(packages_handler), Vec::from(["bar", "foo"]));
    }

    #[test]
    fn parse_deps_works() {
        let mut project = RustProject::default();

        let file_content = "[dependencies]
            foo = \"2.1.0\"
            bar = { version = \"1.0.215\", features = [\"derive\"] }";

        assert_eq!(project.parse_deps(file_content), 2);
        assert_eq!(project.deps.len(), 2);
        assert_eq!(project.deps, Vec::from(["bar", "foo"]));
    }
}
