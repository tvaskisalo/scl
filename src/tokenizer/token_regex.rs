pub const COMMENT_REGEX_STR: &str = r"^//(.*)\n$";
pub const CONST_REGEX_STR: &str = r"^const$";
pub const BOOLEAN_REGEX_STR: &str = r"^(true|false)$";
pub const ELSE_REGEX_STR: &str = r"^else$";
pub const IF_REGEX_STR: &str = r"^if$";
pub const INTEGER_REGEX_STR: &str = r"^\d+$";
pub const NEW_LINE_REGEX_STR: &str = r"^\n$";
pub const UNIT_REGEX_STR: &str = r"^unit$";
pub const VAR_REGEX_STR: &str = r"^var$";
pub const VARIABLE_NAME_REGEX_STR: &str = r"^([a-z]|[A-Z]|_)([a-z]|[A-Z]|_|[0-9])* $";
pub const WHILE_REGEX_STR: &str = r"^while$";
pub const WHITESPACE_REGEX_STR: &str = r"^ $";

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    const SOME_VARIABLE_NAME: &str = "_thisIsSomeString124__123 ";
    const SOME_INVALID_VARIABLE_NAME: &str = "1abc ";
    const SOME_COMMENT: &str = "//THIS IS A COMMENT 124 ___ while if when \n";

    fn regex_matches(regex_str: &str, input: &str) -> bool {
        let regex = Regex::new(regex_str).unwrap();
        return regex.is_match(input);
    }

    #[test]
    fn comment_regex() {
        assert_eq!(regex_matches(COMMENT_REGEX_STR, " "), false);
        assert_eq!(regex_matches(COMMENT_REGEX_STR, "\n"), false);
        assert_eq!(regex_matches(COMMENT_REGEX_STR, SOME_VARIABLE_NAME), false);
        assert_eq!(regex_matches(COMMENT_REGEX_STR, SOME_COMMENT), true);
    }

    #[test]
    fn const_regex() {
        assert_eq!(regex_matches(CONST_REGEX_STR, "const"), true);
    }

    #[test]
    fn boolean_regex() {
        assert_eq!(regex_matches(BOOLEAN_REGEX_STR, "true"), true);
        assert_eq!(regex_matches(BOOLEAN_REGEX_STR, "false"), true);
    }

    #[test]
    fn else_regex() {
        assert_eq!(regex_matches(ELSE_REGEX_STR, "else"), true);
    }

    #[test]
    fn if_regex() {
        assert_eq!(regex_matches(IF_REGEX_STR, "if"), true);
    }

    #[test]
    fn integer_regex() {
        assert_eq!(regex_matches(INTEGER_REGEX_STR, "12347787897798"), true);
        assert_eq!(regex_matches(INTEGER_REGEX_STR, "1233a12"), false);
    }

    #[test]
    fn new_line_regex() {
        assert_eq!(regex_matches(NEW_LINE_REGEX_STR, "\n"), true);
    }

    #[test]
    fn unit_regex() {
        assert_eq!(regex_matches(UNIT_REGEX_STR, "unit"), true);
    }

    #[test]
    fn var_regex() {
        assert_eq!(regex_matches(VAR_REGEX_STR, "var"), true);
    }

    #[test]
    fn variable_name_regex() {
        assert_eq!(
            regex_matches(VARIABLE_NAME_REGEX_STR, SOME_VARIABLE_NAME),
            true
        );
        assert_eq!(
            regex_matches(VARIABLE_NAME_REGEX_STR, SOME_INVALID_VARIABLE_NAME),
            false
        );
    }

    #[test]
    fn while_regex() {
        assert_eq!(regex_matches(WHILE_REGEX_STR, "while"), true);
    }

    #[test]
    fn whitespace_regex() {
        assert_eq!(regex_matches(WHITESPACE_REGEX_STR, " "), true);
        assert_eq!(regex_matches(WHITESPACE_REGEX_STR, "\n"), false);
        assert_eq!(
            regex_matches(WHITESPACE_REGEX_STR, SOME_VARIABLE_NAME),
            false
        );
    }
}
