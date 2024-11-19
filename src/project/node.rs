use std::{collections::HashMap, env, fs::File};

use crate::file_handler::read_file;

use super::Project;

use serde::Deserialize;

const DEPS_FILE: &'static str = "package.json";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageJson {
    dependencies: HashMap<String, String>,
}

struct NodeProject {
    allowed_extensions: Vec<String>,
    deps: Vec<String>,
}

impl NodeProject {
    fn new(allowed_extensions: Vec<String>) -> Self {
        Self {
            allowed_extensions,
            deps: vec![],
        }
    }
}

impl Project<PackageJson> for NodeProject {
    fn default() -> Self {
        Self {
            allowed_extensions: vec!["js".into(), "jsx".into(), "ts".into(), "tsx".into()],
            deps: vec![],
        }
    }

    fn parse_deps(&mut self, deps_file_content: &str) -> usize {
        let package_json: PackageJson = serde_json::from_str(deps_file_content)
            .expect(&format!("Cannot parse {DEPS_FILE} file."));
        self.deps = self.get_deps_names(package_json);
        self.deps.len()
    }

    fn is_deps_importer(&self, file_name: &str) -> bool {
        for ext in self.allowed_extensions.iter() {
            if file_name.ends_with(&format!(".{ext}")) {
                return true;
            }
        }
        return false;
    }

    fn get_deps_names(&self, parsed_file: PackageJson) -> Vec<String> {
        // TODO Dependency with '@types/' prefix shoud not be pushed in the deps array
        let mut names: Vec<String> = parsed_file
            .dependencies
            .iter()
            .map(|(name, _version)| name.into())
            .collect();
        names.sort();
        names
    }
}

fn read_deps_file() -> String {
    let f = File::open(DEPS_FILE).expect(&format!(
        "No file \"{DEPS_FILE}\" in {}",
        env::current_dir().unwrap().display()
    ));
    read_file(f).expect(&format!("Cannot read {DEPS_FILE} file."))
}

#[cfg(test)]
mod project_node_tests {
    use std::collections::HashMap;

    use crate::project::{node::PackageJson, Project};

    use super::NodeProject;

    #[test]
    fn constructors() {
        let project = NodeProject::default();
        assert_eq!(
            project.allowed_extensions,
            vec![
                String::from("js"),
                String::from("jsx"),
                String::from("ts"),
                String::from("tsx")
            ]
        );
        let project = NodeProject::new(vec![String::from("foo")]);
        assert_eq!(project.allowed_extensions, vec![String::from("foo")]);
    }

    #[test]
    fn is_deps_importer() {
        let project = NodeProject::default();
        assert_eq!(project.is_deps_importer("foo.js"), true);
        assert_eq!(project.is_deps_importer("foo.ts"), true);
        assert_eq!(project.is_deps_importer("foo.tsx"), true);
        assert_eq!(project.is_deps_importer("foo.jsx"), true);
        assert_eq!(project.is_deps_importer("foo.rs"), false);
        assert_eq!(project.is_deps_importer("foo.jssx"), false);

        // Making a project to only test .js files
        let project = NodeProject::new(vec![String::from("js")]);
        assert_eq!(project.is_deps_importer("foo.js"), true);
        assert_eq!(project.is_deps_importer("foo.ts"), false);
        assert_eq!(project.is_deps_importer("foo.jsx"), false);
    }

    #[test]
    fn get_deps_names() {
        let project = NodeProject::default();
        let mut package_json = PackageJson {
            dependencies: HashMap::new(),
        };
        package_json.dependencies.insert("foo".into(), "0.1.0".into());
        package_json.dependencies.insert("bar".into(), "0.1.0".into());

        assert_eq!(project.get_deps_names(package_json), vec!["bar", "foo"]);
    }

    #[test]
    fn parse_deps() {
        let mut project = NodeProject::default();
        let mut package_json = PackageJson {
            dependencies: HashMap::new(),
        };
        package_json.dependencies.insert("foo".into(), "0.1.0".into());
        package_json.dependencies.insert("bar".into(), "0.1.0".into());

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
        }
        }";


        assert_eq!(project.parse_deps(file_content), 3);
        assert_eq!(project.deps.len(), 3);
        assert_eq!(project.deps, vec!["bar", "bazz", "foo"]);
    }
}
