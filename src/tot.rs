use std::process::Command;
use urlencoding::encode;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn prepend_to_tot(dot: u64, string: &String) -> Result<()> {
    let pasteboard = make_prepend_url(dot, string);
    Command::new("open")
        .args(&["-j", "-g", pasteboard.trim()])
        .spawn()?;

    return Ok(());
}

fn make_prepend_url(dot: u64, string: &String) -> String {
    let encoded = encode(string);
    return format!("tot://{}/prepend?text={}", dot, encoded);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_prepend_url() {
        let plain_text = String::from("blah blah blah");
        let expected = format!("tot://1/prepend?text={}", encode(&plain_text));
        let actual = make_prepend_url(1, &plain_text);
        assert_eq!(actual, expected);
    }
}
