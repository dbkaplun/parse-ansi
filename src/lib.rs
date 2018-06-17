//! Parse ANSI escape codes (colors, underlines, etc.)
//!
//! ```rust
//! extern crate parse_ansi;
//!
//! assert_eq!(
//!     parse_ansi::ANSI_REGEX.replace_all(
//!         b"Hello, \x1b[42mworld\x1b[0m!",
//!         b"" as &[u8],
//!     ),
//!     b"Hello, world!" as &[u8],
//! );
//! ```

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
extern crate itertools;

use regex::bytes::{Matches, Regex};

// Inspired by https://github.com/nodejs/node/blob/641d4a4159aaa96eece8356e03ec6c7248ae3e73/lib/internal/readline.js#L9
pub const ANSI_RE: &str =
    r"[\x1b\x9b]\[[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]";

lazy_static! {
    /// A `Regex` that matches ANSI escape codes.
    ///
    /// ```rust
    /// # use parse_ansi::ANSI_REGEX;
    /// assert_eq!(
    ///     ANSI_REGEX.replace_all(b"foo \x1b[42mbar\x1b[0m baz", b"" as &[u8]),
    ///     b"foo bar baz" as &[u8],
    /// );
    pub static ref ANSI_REGEX: Regex = Regex::new(ANSI_RE).unwrap();
}

/// Parses ANSI escape codes from the given text, returning an `Iterator<Item = Match>`.
///
/// ```rust
/// # use parse_ansi::parse_bytes;
/// let ansi_text = b"Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = parse_bytes(ansi_text)
///     .map(|m| (m.start(), m.end()))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse_bytes(text: &[u8]) -> Matches {
    ANSI_REGEX.find_iter(text)
}

#[cfg(test)]
mod tests {
    use super::parse_bytes;
    use itertools::zip_eq;

    fn test_parse(text: &[u8], expected: &[(usize, usize, &[u8])]) {
        for (match_, expected_match) in zip_eq(parse_bytes(text), expected.iter()) {
            assert_eq!(
                &(match_.start(), match_.end(), match_.as_bytes()),
                expected_match,
            );
        }
    }

    #[test]
    fn red_underline() {
        test_parse(
            b"before \x1b[31;4mred underline\x1b[0m after",
            &[(7, 14, b"\x1b[31;4m"), (27, 31, b"\x1b[0m")],
        );
    }
}
