pub const COMMENT_REGEX: &str = r"^//(.*)\n$";
pub const CONST_REGEX: &str = r"const";
pub const BOOLEAN_REGEX: &str = r"(true|false)";
pub const ELSE_REGEX: &str = r"else";
pub const IF_REGEX: &str = r"if";
pub const INTEGER_REGEX: &str = r"^\d+$";
pub const UNIT_REGEX: &str = r"unit";
pub const VAR_REGEX: &str = r"var";
pub const VARIABLE_NAME_REGEX: &str = r"^([a-z]|[A-Z]|_)([a-z]|[A-Z]|_|[0-9])*$";
pub const WHILE_REGEX: &str = r"while";
pub const WHITESPACE_REGEX: &str = r" |\n";

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    const SOME_VARIABLE_NAME: &str = "_thisIsSomeString124__123";
    const SOME_INVALID_VARIABLE_NAME: &str = "1abc";
    const SOME_COMMENT: &str = "//THIS IS A COMMENT 124 ___ while if when \n";

    fn regex_matches(regex_str: &str, input: &str) -> bool {
        let regex = Regex::new(regex_str).unwrap();
        return regex.is_match(input);
    }

    #[test]
    fn comment_regex() {
        assert_eq!(regex_matches(COMMENT_REGEX, " "), false);
        assert_eq!(regex_matches(COMMENT_REGEX, "\n"), false);
        assert_eq!(regex_matches(COMMENT_REGEX, SOME_VARIABLE_NAME), false);
        assert_eq!(regex_matches(COMMENT_REGEX, SOME_COMMENT), true);
    }

    #[test]
    fn const_regex() {
        assert_eq!(regex_matches(CONST_REGEX, "const"), true);
    }

    #[test]
    fn boolean_regex() {
        assert_eq!(regex_matches(BOOLEAN_REGEX, "true"), true);
        assert_eq!(regex_matches(BOOLEAN_REGEX, "false"), true);
    }

    #[test]
    fn else_regex() {
        assert_eq!(regex_matches(ELSE_REGEX, "else"), true);
    }

    #[test]
    fn if_regex() {
        assert_eq!(regex_matches(IF_REGEX, "if"), true);
    }

    #[test]
    fn integer_regex() {
        assert_eq!(regex_matches(INTEGER_REGEX, "12347787897798"), true);
        assert_eq!(regex_matches(INTEGER_REGEX, "1233a12"), false);
    }

    #[test]
    fn unit_regex() {
        assert_eq!(regex_matches(UNIT_REGEX, "unit"), true);
    }

    #[test]
    fn var_regex() {
        assert_eq!(regex_matches(VAR_REGEX, "var"), true);
    }

    #[test]
    fn variable_name_regex() {
        assert_eq!(regex_matches(VARIABLE_NAME_REGEX, SOME_VARIABLE_NAME), true);
        assert_eq!(
            regex_matches(VARIABLE_NAME_REGEX, SOME_INVALID_VARIABLE_NAME),
            false
        );
    }

    #[test]
    fn while_regex() {
        assert_eq!(regex_matches(WHILE_REGEX, "while"), true);
    }

    #[test]
    fn whitespace_regex() {
        assert_eq!(regex_matches(WHITESPACE_REGEX, " "), true);
        assert_eq!(regex_matches(WHITESPACE_REGEX, "\n"), true);
        assert_eq!(regex_matches(WHITESPACE_REGEX, SOME_VARIABLE_NAME), false);
    }
}
