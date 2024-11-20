pub mod node_js;

pub trait Project<F> {
    /// Default contrustor for a Project
    /// Some informations could be changes likes accepeted_extensions for example this is why is
    /// why structs implementing Project should have their own constructors too to change some
    /// functionnalities
    fn default() -> Self;

    /// Returns the number of dependency in the projet.
    fn parse_deps(&mut self, deps_file_content: &str) -> usize;

    /// Check if a file is theorically a dependency importer
    /// Meaning that the file should have extension, name or anything else matching the language.
    fn should_scan_file(&self, file_name: &str) -> bool;

    fn get_deps_names(&self, parsed_file: F) -> Vec<String>;
}
