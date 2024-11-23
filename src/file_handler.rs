use std::{fs::File, io::Read};

pub fn read_file(mut f: File) -> Result<String, String> {
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.to_string()),
    }
}

pub fn read_file_at_path(path: &str) -> Result<String, String> {
    match File::open(path) {
        Ok(f) => read_file(f),
        Err(e) => Err(e.to_string()),
    }
}

pub fn string_exists_in_multiline_text(term: &str, content: &str) -> bool {
    for line in content.lines() {
        if line.contains(term) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_exists() {
        let content = "foo\nbar\nbazz";
        assert_eq!(string_exists_in_multiline_text("bar", &content), true);
        assert_eq!(string_exists_in_multiline_text("foo", &content), true);
        assert_eq!(string_exists_in_multiline_text("fooo", &content), false);
        assert_eq!(string_exists_in_multiline_text("bazzz", &content), false);
    }
}
