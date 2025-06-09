pub trait StringExt {
    fn slugify(&self) -> String;
}

impl StringExt for str {
    fn slugify(&self) -> String {
        self.to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    }
}

impl StringExt for String {
    fn slugify(&self) -> String {
        self.as_str().slugify()
    }
}
