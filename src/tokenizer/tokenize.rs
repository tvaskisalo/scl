use regex::Regex;

use super::*;

pub fn tokenize(mut input: String) -> Vec<String> {
    let comment_regex = Regex::new(COMMENT_REGEX_STR).unwrap();
    let const_regex = Regex::new(CONST_REGEX_STR).unwrap();
    let boolean_regex = Regex::new(BOOLEAN_REGEX_STR).unwrap();
    let else_regex = Regex::new(ELSE_REGEX_STR).unwrap();
    let if_regex = Regex::new(IF_REGEX_STR).unwrap();
    let integer_regex = Regex::new(INTEGER_REGEX_STR).unwrap();
    let new_line_regex = Regex::new(NEW_LINE_REGEX_STR).unwrap();
    let unit_regex = Regex::new(UNIT_REGEX_STR).unwrap();
    let var_regex = Regex::new(VAR_REGEX_STR).unwrap();
    let variable_name_regex = Regex::new(VARIABLE_NAME_REGEX_STR).unwrap();
    let while_regex = Regex::new(WHILE_REGEX_STR).unwrap();
    let whitespace_regex = Regex::new(WHITESPACE_REGEX_STR).unwrap();

    let mut substring_start_index: usize = 0;
    let mut substring_end_index: usize = 1;
    let mut tokens: Vec<String> = Vec::new();

    fn move_indexes(substring_start_index: &mut usize, substring_end_index: &mut usize) {
        *substring_start_index = *substring_end_index;
        *substring_end_index = *substring_end_index + 1;
    }

    while substring_end_index <= input.len() {
        let slice: &str = input
            .get_mut(substring_start_index..substring_end_index)
            .unwrap();
        if slice.len() == 0 {
            continue;
        }
        match slice {
            // Must match first
            s if new_line_regex.is_match(s) => {}
            s if whitespace_regex.is_match(s) => {}
            s if comment_regex.is_match(s) => {}

            // Rest alphabetically
            s if const_regex.is_match(s) => {}
            s if boolean_regex.is_match(s) => {}
            s if else_regex.is_match(s) => {}
            s if if_regex.is_match(s) => tokens.push(slice.to_string()),
            s if integer_regex.is_match(s) => tokens.push(slice.to_string()),
            s if unit_regex.is_match(s) => {}
            s if var_regex.is_match(s) => {}
            s if while_regex.is_match(s) => tokens.push(slice.to_string()),

            // Check last
            s if variable_name_regex.is_match(s) => tokens.push(slice.trim().to_string()),

            _ => {
                substring_end_index = substring_end_index + 1;
                continue;
            }
        }
        // Something matched, so move indexes.
        move_indexes(&mut substring_start_index, &mut substring_end_index);
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;

    #[test]
    fn simple_string() {
        assert_eq!(
            tokenize("if  3\nwhile _var_nameconst ".to_string()),
            ["if", "3", "while", "_var_nameconst"]
        )
    }
}
