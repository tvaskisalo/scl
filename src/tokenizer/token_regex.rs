pub const WHITESPACE_REGEX: &str = r" |\n";
pub const COMMENT_REGEX: &str = r"//(.*)n";

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::{COMMENT_REGEX, WHITESPACE_REGEX};

    const WHITESPACE: &str = " ";
    const NEW_LINE: &str = "\n";
    const SOME_STRING: &str = "_thisIsSomeString124__123";
    const SOME_COMMENT: &str = r"//THIS IS A COMMENT 124 ___ while if when \n";

    fn regex_matches(regex_str: &str, input: &str) -> bool {
        let regex = Regex::new(regex_str).unwrap();
        return regex.is_match(input);
    }

    #[test]
    fn whitespace_regex() {
        assert_eq!(regex_matches(WHITESPACE_REGEX, WHITESPACE), true);
        assert_eq!(regex_matches(WHITESPACE_REGEX, NEW_LINE), true);
        assert_eq!(regex_matches(WHITESPACE_REGEX, SOME_STRING), false);
    }

    #[test]
    fn comment_regex() {
        assert_eq!(regex_matches(COMMENT_REGEX, WHITESPACE), false);
        assert_eq!(regex_matches(COMMENT_REGEX, NEW_LINE), false);
        assert_eq!(regex_matches(COMMENT_REGEX, SOME_STRING), false);
        assert_eq!(regex_matches(COMMENT_REGEX, SOME_COMMENT), true);
    }
}
