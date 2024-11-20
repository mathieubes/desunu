use std::{collections::HashMap, env, fs::File};

use crate::file_handler::read_file;

use super::Project;

use serde::Deserialize;

const DEPS_FILE: &str = "package.json";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePackagesHandler {
    dependencies: HashMap<String, String>,
    scripts: HashMap<String, String>,
}

pub struct NodeProject {
    deps: Vec<String>,

    allowed_extensions: Vec<String>,
    excluded_paths: Vec<String>,
}

impl NodeProject {
    fn new(allowed_extensions: Vec<String>, excluded_files: Vec<String>) -> Self {
        Self {
            allowed_extensions,
            excluded_paths: excluded_files,
            deps: vec![],
        }
    }
}

impl Project for NodeProject {
    fn default() -> Self {
        Self::new(
            vec![
                "js".into(),
                "jsx".into(),
                "ts".into(),
                "tsx".into(),
                "scss".into(),
                "sass".into(),
                "json".into(),
            ],
            vec!["package.json".into(), "node_modules/".into()],
        )
    }

    fn parse_deps(&mut self, deps_file_content: &str) -> usize {
        let packages_handler: NodePackagesHandler = serde_json::from_str(deps_file_content)
            .unwrap_or_else(|_| panic!("Cannot parse {DEPS_FILE} file."));
        self.deps = get_deps_names(packages_handler);
        self.deps.len()
    }

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
}

pub fn read_deps_file() -> String {
    let f = File::open(DEPS_FILE).unwrap_or_else(|_| {
        panic!(
            "No file \"{DEPS_FILE}\" in {}",
            env::current_dir().unwrap().display()
        )
    });
    read_file(f).unwrap_or_else(|_| panic!("Cannot read {DEPS_FILE} file."))
}

fn is_used_in_package_scripts(parsed_file: &NodePackagesHandler, name: &str) -> bool {
    for script in parsed_file.scripts.values() {
        if script.contains(name) {
            return true;
        }
    }
    false
}

fn get_deps_names(parsed_file: NodePackagesHandler) -> Vec<String> {
    let mut names: Vec<String> =
        parsed_file
            .dependencies
            .iter()
            .fold(Vec::new(), |mut acc, (name, _version)| {
                if name.starts_with("@types/") || is_used_in_package_scripts(&parsed_file, name) {
                    return acc;
                }
                acc.push(name.into());
                acc
            });
    names.sort();
    names
}

#[cfg(test)]
mod project_node_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn should_scan_file_works() {
        let project = NodeProject::default();
        assert_eq!(project.should_scan_file("foo.js"), true);
        assert_eq!(project.should_scan_file("foo.ts"), true);
        assert_eq!(project.should_scan_file("foo.tsx"), true);
        assert_eq!(project.should_scan_file("foo.jsx"), true);
        assert_eq!(project.should_scan_file("foo.rs"), false);
        assert_eq!(project.should_scan_file("foo.jssx"), false);
        assert_eq!(project.should_scan_file("package.json"), false);
        assert_eq!(project.should_scan_file("foo/node_modules/foo.ts"), false);

        let project = NodeProject::new(vec![String::from("js")], Vec::new());
        assert_eq!(project.should_scan_file("foo.js"), true);
        assert_eq!(project.should_scan_file("foo.ts"), false);
        assert_eq!(project.should_scan_file("foo.jsx"), false);

        let project = NodeProject::new(
            vec![String::from("ts")],
            vec![String::from("bar.ts"), String::from("node_modules/")],
        );
        assert_eq!(project.should_scan_file("foo/bar/foo.ts"), true);
        assert_eq!(project.should_scan_file("foo/bar/bar.ts"), false);
        assert_eq!(project.should_scan_file("bar.ts"), false);
        assert_eq!(project.should_scan_file("foo/bar/package.json"), false);
        assert_eq!(project.should_scan_file("foo/node_modules/foo.ts"), false);
    }

    #[test]
    fn get_deps_names_works() {
        let mut packages_handler = NodePackagesHandler {
            dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        packages_handler
            .dependencies
            .insert("foo".into(), "0.1.0".into());
        packages_handler
            .dependencies
            .insert("bar".into(), "0.1.0".into());
        packages_handler
            .dependencies
            .insert("@types/foo".into(), "0.1.0".into());

        assert_eq!(get_deps_names(packages_handler), vec!["bar", "foo"]);
    }

    #[test]
    fn parse_deps_works() {
        let mut project = NodeProject::default();

        let file_content = "{
        \"name\": \"foo\",
        \"dependencies\": {
            \"foo\": \"0.1.0\",
            \"bar\": \"0.1.0\",
            \"bazz\": \"0.1.0\"
        },
        \"devDependencies\": {
            \"dev-foo\": \"0.1.0\",
            \"dev-bar\": \"0.1.0\",
            \"dev-bazz\": \"0.1.0\"
        },
        \"scripts\": {
            \"foo\": \"quix\"
        }
        }";

        assert_eq!(project.parse_deps(file_content), 3);
        assert_eq!(project.deps.len(), 3);
        assert_eq!(project.deps, vec!["bar", "bazz", "foo"]);
    }

    #[test]
    fn guess_if_package_scripts_use_deps() {
        let mut packages_handler = NodePackagesHandler {
            dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        packages_handler
            .scripts
            .insert("foo".into(), "foo bar baz".into());

        assert_eq!(is_used_in_package_scripts(&packages_handler, "bar"), true);
        assert_eq!(is_used_in_package_scripts(&packages_handler, "qux"), false);
    }
}
