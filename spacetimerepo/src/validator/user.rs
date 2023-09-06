pub fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        return Err("Names must not be empty".to_string())
    }

    Ok(name)
}
