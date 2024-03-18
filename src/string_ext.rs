pub trait StringManip {
    fn capitalize_first(&self) -> String;
}

impl StringManip for String {
    fn capitalize_first(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
