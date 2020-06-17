pub fn parse_uint(a: &str) -> Result<usize, String> {
    match a.parse() {
        Ok(num) => Ok(num),
        Err(reason) => Err(reason.to_string()),
    }
}
