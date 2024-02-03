use regex::Regex;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    filename: String,
    line: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    IntLiteral,
    Operator,
    Punctuation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    location: Location,
    token_type: TokenType,
    value: String,
}

fn move_indexes(substring_start_index: &mut usize, substring_end_index: &mut usize) {
    *substring_start_index = *substring_end_index;
    *substring_end_index = *substring_end_index + 1;
}

// Check if character at given index is "empty"
// "empty" means a space, newline, tab or string end
fn char_is_empty(input: &String, index: usize) -> bool {
    let next_char = input.chars().nth(index);
    if next_char.is_some() {
        let character = next_char.unwrap();
        return (character == ' ') | (character == '\n') | (character == '\t');
    } else {
        // Index larger than input size
        return true;
    }
}

fn handle_new_line(line: &mut usize, col: &mut usize) {
    *line = *line + 1;
    *col = 1;
}

fn handle_col(col: &mut usize, size: usize) {
    *col = *col + size;
}

pub fn tokenize(mut input: String) -> Vec<Token> {
    let assign_regex = Regex::new(ASSIGN_REGEX_STR).unwrap();
    let asterix_regex = Regex::new(ASTERIX_REGEX_STR).unwrap();
    let comma_regex = Regex::new(COMMA_REGEX_STR).unwrap();
    let comment_regex = Regex::new(COMMENT_REGEX_STR).unwrap();
    let const_regex = Regex::new(CONST_REGEX_STR).unwrap();
    let curly_close_regex = Regex::new(CURLY_CLOSE_REGEX_STR).unwrap();
    let curly_open_regex = Regex::new(CURLY_OPEN_REGEX_STR).unwrap();
    let boolean_regex = Regex::new(BOOLEAN_REGEX_STR).unwrap();
    let bracket_close_regex = Regex::new(BRACKET_CLOSE_REGEX_STR).unwrap();
    let bracket_open_regex = Regex::new(BRACKET_OPEN_REGEX_STR).unwrap();
    let dash_regex = Regex::new(DASH_REGEX_STR).unwrap();
    let else_regex = Regex::new(ELSE_REGEX_STR).unwrap();
    let eq_regex = Regex::new(EQ_REGEX_STR).unwrap();
    let greater_regex = Regex::new(GREATER_REGEX_STR).unwrap();
    let geq_regex = Regex::new(GEQ_REGEX_STR).unwrap();
    let if_regex = Regex::new(IF_REGEX_STR).unwrap();
    let integer_regex = Regex::new(INTEGER_REGEX_STR).unwrap();
    let less_regex = Regex::new(LESS_REGEX_STR).unwrap();
    let leq_regex = Regex::new(LEQ_REGEX_STR).unwrap();
    let minus_regex = Regex::new(MINUS_REGEX_STR).unwrap();
    let neq_regex = Regex::new(NEQ_REGEX_STR).unwrap();
    let new_line_regex = Regex::new(NEW_LINE_REGEX_STR).unwrap();
    let plus_regex = Regex::new(PLUS_REGEX_STR).unwrap();
    let semicolon_regex = Regex::new(SEMICOLON_REGEX_STR).unwrap();
    let unit_regex = Regex::new(UNIT_REGEX_STR).unwrap();
    let var_regex = Regex::new(VAR_REGEX_STR).unwrap();
    let variable_name_regex = Regex::new(VARIABLE_NAME_REGEX_STR).unwrap();
    let while_regex = Regex::new(WHILE_REGEX_STR).unwrap();
    let whitespace_regex = Regex::new(WHITESPACE_REGEX_STR).unwrap();

    let mut substring_start_index: usize = 0;
    let mut substring_end_index: usize = 1;
    let mut tokens: Vec<Token> = Vec::new();

    let mut col: usize = 1;
    let mut line: usize = 1;

    while substring_end_index <= input.len() {
        let slice_ends = char_is_empty(&input, substring_end_index);
        let slice: &str = input
            .get_mut(substring_start_index..substring_end_index)
            .unwrap();
        let current_location: Location = Location {
            filename: "TODO".to_string(),
            line: line.clone(),
            col: col.clone(),
        };
        let mut new_line_matched: bool = false;
        match slice_ends {
            true => {
                match slice {
                    // Must match first
                    // These regexes do not care if the slice ends or not
                    s if new_line_regex.is_match(s) => new_line_matched = true,
                    s if whitespace_regex.is_match(s) => {}
                    s if comment_regex.is_match(s) => new_line_matched = true,
                    s if eq_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if geq_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if leq_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if neq_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
 
                    // Rest alphabetically
                    // Regexes demand that the next char after the slice is "empty"
                    s if assign_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if asterix_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if comma_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if const_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if curly_close_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if curly_open_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if boolean_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if bracket_close_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if bracket_open_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if dash_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if else_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if greater_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if if_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if integer_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::IntLiteral,
                        value: slice.to_string(),
                    }),
                    s if less_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if minus_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if plus_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Operator,
                        value: slice.to_string(),
                    }),
                    s if semicolon_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Punctuation,
                        value: slice.to_string(),
                    }),
                    s if unit_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::IntLiteral,
                        value: slice.to_string(),
                    }),
                    s if var_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if while_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),
                    s if variable_name_regex.is_match(s) => tokens.push(Token {
                        location: current_location,
                        token_type: TokenType::Identifier,
                        value: slice.to_string(),
                    }),

                    // Nothing matched, move slice end index by one
                    _ => {
                        substring_end_index = substring_end_index + 1;
                        continue;
                    }
                }
            }
            false => {
                match slice {
                    // These regexes do not care if the slice ends or not
                    s if new_line_regex.is_match(s) => new_line_matched = true,
                    s if whitespace_regex.is_match(s) => {}
                    s if comment_regex.is_match(s) => new_line_matched = true,

                    // Nothing matched, move slice end index by one
                    _ => {
                        substring_end_index = substring_end_index + 1;
                        continue;
                    }
                }
            }
        }

        // Something matched, so move indexes.
        move_indexes(&mut substring_start_index, &mut substring_end_index);

        if new_line_matched {
            handle_new_line(&mut line, &mut col);
        } else {
            // Move col
            handle_col(&mut col, slice.len())
        }
    }
    return tokens;
}
#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize::*;

    #[test]
    fn simple_string() {
        let expected_tokens: Vec<Token> = vec![
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 1,
                },
                token_type: TokenType::Identifier,
                value: "if".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 5,
                },
                token_type: TokenType::IntLiteral,
                value: "3".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 2,
                    col: 1,
                },
                token_type: TokenType::Identifier,
                value: "while".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 2,
                    col: 7,
                },
                token_type: TokenType::Identifier,
                value: "_var_nameconst".to_string(),
            },
        ];

        let tokens: Vec<Token> = tokenize("if  3\nwhile _var_nameconst ".to_string());
        assert_eq!(expected_tokens[0], tokens[0]);
        assert_eq!(expected_tokens[1], tokens[1]);
        assert_eq!(expected_tokens[2], tokens[2]);
        assert_eq!(expected_tokens[3], tokens[3]);
    }

    #[test]
    fn longer_string() {
        let expected_tokens: Vec<Token> = vec![
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 1,
                },
                token_type: TokenType::IntLiteral,
                value: "12334".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 2,
                    col: 2,
                },
                token_type: TokenType::Identifier,
                value: "const".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 2,
                    col: 8,
                },
                token_type: TokenType::Identifier,
                value: "variable_name".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 2,
                    col: 22,
                },
                token_type: TokenType::Identifier,
                value: "true".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 3,
                    col: 1,
                },
                token_type: TokenType::Identifier,
                value: "while".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 6,
                    col: 1,
                },
                token_type: TokenType::IntLiteral,
                value: "unit".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 6,
                    col: 6,
                },
                token_type: TokenType::Identifier,
                value: "if".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 6,
                    col: 9,
                },
                token_type: TokenType::Identifier,
                value: "else".to_string(),
            },
        ];

        let tokens: Vec<Token> = tokenize("12334 // This is a comment var const if else \n const variable_name true\nwhile    \n\n\nunit if else ".to_string());
        assert_eq!(expected_tokens[0], tokens[0]);
        assert_eq!(expected_tokens[1], tokens[1]);
        assert_eq!(expected_tokens[2], tokens[2]);
        assert_eq!(expected_tokens[3], tokens[3]);
        assert_eq!(expected_tokens[4], tokens[4]);
        assert_eq!(expected_tokens[5], tokens[5]);
        assert_eq!(expected_tokens[6], tokens[6]);
        assert_eq!(expected_tokens[7], tokens[7]);
    }

    #[test]
    fn test_punctuation_and_operators()  {
        let expected_tokens: Vec<Token> = vec![
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 1,
                },
                token_type: TokenType::Operator,
                value: "+".to_string(),
            }, 
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 3,
                },
                token_type: TokenType::Operator,
                value: "-".to_string(),
            }, 
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 5,
                },
                token_type: TokenType::Operator,
                value: "*".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 7,
                },
                token_type: TokenType::Operator,
                value: "/".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 9,
                },
                token_type: TokenType::Operator,
                value: "=".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 11,
                },
                token_type: TokenType::Punctuation,
                value: "(".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 13,
                },
                token_type: TokenType::Operator,
                value: "==".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 16,
                },
                token_type: TokenType::Punctuation,
                value: ")".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 18,
                },
                token_type: TokenType::Operator,
                value: "!=".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 21,
                },
                token_type: TokenType::Punctuation,
                value: "{".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 23,
                },
                token_type: TokenType::Operator,
                value: "<".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 25,
                },
                token_type: TokenType::Punctuation,
                value: "}".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 27,
                },
                token_type: TokenType::Operator,
                value: "<=".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 30,
                },
                token_type: TokenType::Punctuation,
                value: ",".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 32,
                },
                token_type: TokenType::Operator,
                value: ">".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 34,
                },
                token_type: TokenType::Punctuation,
                value: ";".to_string(),
            },
            Token {
                location: Location {
                    filename: "TODO".to_string(),
                    line: 1,
                    col: 36,
                },
                token_type: TokenType::Operator,
                value: ">=".to_string(),
            },
        ];
        let tokens: Vec<Token> = tokenize(r"+ - * / = ( == ) != { < } <= , > ; >=".to_string());
        assert_eq!(expected_tokens[0], tokens[0]);
        assert_eq!(expected_tokens[1], tokens[1]);
        assert_eq!(expected_tokens[2], tokens[2]);
        assert_eq!(expected_tokens[3], tokens[3]);
        assert_eq!(expected_tokens[4], tokens[4]);
        assert_eq!(expected_tokens[5], tokens[5]);
        assert_eq!(expected_tokens[6], tokens[6]);
        assert_eq!(expected_tokens[7], tokens[7]);
        assert_eq!(expected_tokens[8], tokens[8]);
        assert_eq!(expected_tokens[9], tokens[9]);
        assert_eq!(expected_tokens[10], tokens[10]);
        assert_eq!(expected_tokens[11], tokens[11]);
        assert_eq!(expected_tokens[12], tokens[12]);
        assert_eq!(expected_tokens[13], tokens[13]);
        assert_eq!(expected_tokens[14], tokens[14]);
        assert_eq!(expected_tokens[15], tokens[15]);
        assert_eq!(expected_tokens[16], tokens[16]);
    }
}
