//! Write multi-line strings like in Scala.
//!
//! Inspired by `stripmargin` crate (https://docs.rs/stripmargin/0.1.1/stripmargin/).
pub trait StripMargin {
    /// For every line in this string, strip a leading prefix consisting of blanks
    /// or control characters, followed by `margin`.
    ///
    /// # Example
    /// ```
    /// use adventofcode::StripMargin;
    /// assert_eq!(
    ///     r#"> Hello,
    ///        >   world!
    ///        > "#
    ///     .strip_margin_of("> "),
    ///     "Hello,\n  world!\n",
    /// );
    /// ```
    fn strip_margin_of(&self, margin: &str) -> String;

    /// Shorthand for `strip_margin_of("|")`.
    ///
    /// # Example
    /// ```
    /// use adventofcode::StripMargin;
    /// assert_eq!(
    ///     r#"|Hello,
    ///        |  world!
    ///        |"#
    ///     .strip_margin(),
    ///     "Hello,\n  world!\n",
    /// );
    /// ```
    fn strip_margin(&self) -> String {
        self.strip_margin_of("|")
    }
}

impl<S: AsRef<str>> StripMargin for S {
    fn strip_margin_of(&self, margin: &str) -> String {
        self.as_ref()
            .split('\n')
            .map(|line| match line.trim_start().strip_prefix(margin) {
                Some(trimmed) => trimmed,
                _ => line,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_margin() {
        let input = "  |1\n|2\n |3\n  b | 4";
        assert_eq!(&input.strip_margin(), "1\n2\n3\n  b | 4");
    }

    #[test]
    fn test_input_with_empty_lines() {
        let input = r"
            |1721
            |979
            |366
            |
            |299
            |675
            |1456
            |";
        let l = input.strip_margin();
        let v: Vec<_> = l.split('\n').collect();
        assert_eq!(
            v,
            vec!["", "1721", "979", "366", "", "299", "675", "1456", ""]
        );
    }
}
