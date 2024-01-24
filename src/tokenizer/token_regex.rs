pub const ASTERIX_REGEX_STR: &str = r"^\*$";
pub const ASSIGN_REGEX_STR: &str = r"^=$";
pub const COMMA_REGEX_STR: &str = r"^,$";
pub const COMMENT_REGEX_STR: &str = r"^//(.*)\n$";
pub const CONST_REGEX_STR: &str = r"^const$";
pub const CURLY_CLOSE_REGEX_STR: &str = r"^\}$";
pub const CURLY_OPEN_REGEX_STR: &str = r"^\{$";
pub const BOOLEAN_REGEX_STR: &str = r"^(true|false)$";
pub const BRACKET_CLOSE_REGEX_STR: &str = r"^\)$";
pub const BRACKET_OPEN_REGEX_STR: &str = r"^\($";
pub const DASH_REGEX_STR: &str = r"^\\$";
pub const ELSE_REGEX_STR: &str = r"^else$";
pub const EQ_REGEX_STR: &str = r"^==$";
pub const GREATER_REGEX_STR: &str = r"^>$";
pub const GEQ_REGEX_STR: &str = r"^>=$";
pub const IF_REGEX_STR: &str = r"^if$";
pub const INTEGER_REGEX_STR: &str = r"^\d+$";
pub const LESS_REGEX_STR: &str = r"^<$";
pub const LEQ_REGEX_STR: &str = r"^<=$";
pub const MINUS_REGEX_STR: &str = r"^-$";
pub const NEQ_REGEX_STR: &str = r"^!=$";
pub const NEW_LINE_REGEX_STR: &str = r"^\n$";
pub const PLUS_REGEX_STR: &str = r"^\+$";
pub const SEMICOLON_REGEX_STR: &str = r"^;$";
pub const UNIT_REGEX_STR: &str = r"^unit$";
pub const VAR_REGEX_STR: &str = r"^var$";
pub const VARIABLE_NAME_REGEX_STR: &str = r"^([a-z]|[A-Z]|_)([a-z]|[A-Z]|_|[0-9])*$";
pub const WHILE_REGEX_STR: &str = r"^while$";
pub const WHITESPACE_REGEX_STR: &str = r"^ $";

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    const SOME_VARIABLE_NAME: &str = "_thisIsSomeString124__123";
    const SOME_INVALID_VARIABLE_NAME: &str = "1abc";
    const SOME_COMMENT: &str = "//THIS IS A COMMENT 124 ___ while if when \n";

    fn regex_matches(regex_str: &str, input: &str) -> bool {
        let regex = Regex::new(regex_str).unwrap();
        return regex.is_match(input);
    }

    #[test]
    fn asterix_regex() {
        assert_eq!(regex_matches(ASTERIX_REGEX_STR, "*"), true);
    }

    #[test]
    fn assign_regex() {
        assert_eq!(regex_matches(ASSIGN_REGEX_STR, "="), true);
    }

    #[test]
    fn comma_regex() {
        assert_eq!(regex_matches(COMMA_REGEX_STR, ","), true);
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
    fn curly_close_regex() {
        assert_eq!(regex_matches(CURLY_CLOSE_REGEX_STR, "}"), true);
    }

    #[test]
    fn curly_open_regex() {
        assert_eq!(regex_matches(CURLY_OPEN_REGEX_STR, "{"), true);
    }

    #[test]
    fn boolean_regex() {
        assert_eq!(regex_matches(BOOLEAN_REGEX_STR, "true"), true);
        assert_eq!(regex_matches(BOOLEAN_REGEX_STR, "false"), true);
    }

    #[test]
    fn bracket_close_regex() {
        assert_eq!(regex_matches(BRACKET_CLOSE_REGEX_STR, ")"), true);
    }

    #[test]
    fn bracket_open_regex() {
        assert_eq!(regex_matches(BRACKET_OPEN_REGEX_STR, "("), true);
    }

    #[test]
    fn dash_regex() {
        assert_eq!(regex_matches(DASH_REGEX_STR, r"\"), true);
    }

    #[test]
    fn else_regex() {
        assert_eq!(regex_matches(ELSE_REGEX_STR, "else"), true);
    }

    #[test]
    fn eq_regex() {
        assert_eq!(regex_matches(EQ_REGEX_STR, "=="), true);
    }

    #[test]
    fn greater_regex() {
        assert_eq!(regex_matches(GREATER_REGEX_STR, ">"), true);
    }

    #[test]
    fn geq_regex() {
        assert_eq!(regex_matches(GEQ_REGEX_STR, ">="), true);
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
    fn less_regex() {
        assert_eq!(regex_matches(LESS_REGEX_STR, "<"), true);
    }

    #[test]
    fn leq_regex() {
        assert_eq!(regex_matches(LEQ_REGEX_STR, "<="), true);
    }

    #[test]
    fn minus_regex() {
        assert_eq!(regex_matches(MINUS_REGEX_STR, "-"), true);
    }

    #[test]
    fn neq_regex() {
        assert_eq!(regex_matches(NEQ_REGEX_STR, "!="), true);
    }

    #[test]
    fn new_line_regex() {
        assert_eq!(regex_matches(NEW_LINE_REGEX_STR, "\n"), true);
    }

    #[test]
    fn plus_regex() {
        assert_eq!(regex_matches(PLUS_REGEX_STR, "+"), true);
    }

    #[test]
    fn semicolon_regex() {
        assert_eq!(regex_matches(SEMICOLON_REGEX_STR, ";"), true);
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
