#[must_use]
pub fn format_de_number(value: f64, seperator: &str) -> String {
    value.to_string().replace('.', seperator)
}

#[allow(clippy::missing_panics_doc)]
pub fn format_with_thousands_seperator(value: f64, seperator: &str) -> String {
    format!("{value:.0}")
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(seperator)
}
