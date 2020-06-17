pub fn parse_uint(a: &str) -> Result<usize, String> {
    match a.parse() {
        Ok(num) => Ok(num),
        Err(reason) => Err(reason.to_string()),
    }
}
pub fn find_newline(string: &str) -> Option<usize> {
    match (
        string.find('\n').map(|i| i + 1),
        string.find("\r\n").map(|i| i + 2),
    ) {
        (Some(lf), Some(crlf)) => Some(lf.min(crlf)),
        (Some(lf), None) => Some(lf),
        (None, Some(crlf)) => Some(crlf),
        (None, None) => None,
    }
}
