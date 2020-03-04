use std::process::Command;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_pasteboard() -> Result<String> {
    let last_pasteboard = Command::new("pbpaste").output()?.stdout;
    let output = String::from_utf8(last_pasteboard)?;
    return Ok(format_pasteboard(output));
}

fn format_pasteboard(output: String) -> String {
    let mut string_output = output.trim().to_string();
    string_output.push_str("\n");
    return string_output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_pasteboard() {
        let test_string = " a string ";
        let expected = "a string\n";
        let actual = format_pasteboard(test_string.to_string());
        assert_eq!(expected, actual);
    }
}
