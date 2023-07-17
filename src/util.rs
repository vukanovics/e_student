pub fn string_to_url(string: &str) -> String {
    string
        .to_string()
        .chars()
        .filter(|c| char::is_ascii_alphanumeric(c) || c == &' ')
        .map(|c| match c {
            ' ' => '_',
            _ => char::to_ascii_lowercase(&c),
        })
        .collect()
}
