pub fn upper(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(letter) => letter.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
