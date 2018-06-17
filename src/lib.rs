#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
extern crate itertools;

use regex::bytes::{Captures, Regex};

// Inspired by https://github.com/nodejs/node/blob/641d4a4159aaa96eece8356e03ec6c7248ae3e73/lib/internal/readline.js#L9
pub const ANSI_RE: &str =
    r"[\x1b\x9b]\[[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]";

lazy_static! {
    pub static ref ANSI_REGEX: Regex = Regex::new(ANSI_RE).unwrap();
}

/// Parses ANSI escape codes from the given text, returning an iterator of `Captures`.
///
/// ```rust
/// # use parse_ansi::parse_bytes;
/// let ansi_text = b"Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = parse_bytes(ansi_text)
///     .flat_map(|caps| caps.iter().collect::<Vec<_>>())
///     .filter_map(|cap| cap.map(|c| {
///         (c.start(), c.end())
///     }))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse_bytes(text: &[u8]) -> impl Iterator<Item = Captures> {
    ANSI_REGEX.captures_iter(text)
}

#[cfg(test)]
mod tests {
    use super::parse_bytes;
    use itertools::zip_eq;

    fn test_parse(text: &[u8], expected: &[&[Option<(usize, usize, &[u8])>]]) {
        for (caps, expected_caps) in zip_eq(parse_bytes(text), expected.iter()) {
            for (match_, expected_match) in zip_eq(caps.iter(), expected_caps.iter()) {
                if let Some(match_) = match_ {
                    assert_eq!(
                        (match_.start(), match_.end(), match_.as_bytes()),
                        expected_match.unwrap(),
                    );
                } else {
                    assert!(expected_match.is_none());
                }
            }
        }
    }

    #[test]
    fn red_underline() {
        test_parse(
            b"before \x1b[31;4mred underline\x1b[0m after",
            &[
                &[Some((7, 14, b"\x1b[31;4m"))],
                &[Some((27, 31, b"\x1b[0m"))],
            ],
        );
    }
}
