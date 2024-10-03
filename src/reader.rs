use regex::Regex;

use crate::mal_types::*;

use std::collections::{BTreeMap, HashMap};

macro_rules! consume_and_assert_eq {
    ( $reader:expr, $expected:expr ) => {{
        let token = $reader
            .next()
            .expect(&format!("Expected {:?} but got None!", &$expected));
        if token != $expected {
            panic!("Expected {:?} but got {:?}", &$expected, &token);
        }
    }};
}

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn peek(&self) -> Option<String> {
        if self.tokens.len() > self.position {
            Some(self.tokens[self.position].to_owned())
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(token) = self.peek() {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn read_str(code: &str) -> MalResult {
    let tokens = tokenizer(code);
    let mut reader = Reader {
        tokens: tokens,
        position: 0,
    };
    read_form(&mut reader)
}

const TOKEN_MATCH: &str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#;

fn tokenizer(code: &str) -> Vec<String> {
    let re = Regex::new(TOKEN_MATCH).unwrap();
    let mut tokens: Vec<String> = vec![];
    for token_match in re.captures_iter(code) {
        tokens.push(token_match[1].to_string());
    }
    tokens
}

fn read_form(reader: &mut Reader) -> MalResult {
    let token = reader.peek().unwrap();
    if token.len() == 0 {
        return Err(MalError("Unexpected EOF".to_string()));
    }
    let mut chars = token.chars();
    match chars.next().unwrap() {
        ';' => {
            reader.next();
            Err(MalError("".to_string()))
        }
        '(' => read_list(reader),
        '[' => read_vector(reader),
        '"' => read_string(reader),
        ':' => read_keyword(reader),
        '\'' => read_quote(reader, "quote"),
        '~' => {
            if let Some('@') = chars.next() {
                read_quote(reader, "splice-unquote")
            } else {
                read_quote(reader, "unquote")
            }
        }
        '`' => read_quote(reader, "quasiquote"),
        '@' => read_quote(reader, "deref"),
        _ => read_atom(reader),
    }
}

fn read_string(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let mut chars = token.chars();
    if chars.next().unwrap() != '"' {
        panic!("Expected start of a string!")
    }
    let mut str = String::new();
    loop {
        match chars.next() {
            Some('"') => break,
            Some('\\') => str.push(unescape_char(chars.next())?),
            Some(c) => str.push(c),
            None => return Err(MalError("Unexpected end of string!".to_string())),
        }
    }
    Ok(MalType::Str(str))
}

fn read_keyword(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    Ok(MalType::Keyword(token[1..].to_string()))
}

fn read_quote(reader: &mut Reader, expanded: &str) -> MalResult {
    reader.next().unwrap();
    let value = read_form(reader).unwrap();
    let list = MalType::MalList(vec![MalType::Symbol(expanded.to_string()), value].into());
    Ok(list)
}

fn unescape_char(char: Option<char>) -> Result<char, MalError> {
    match char {
        Some('n') => Ok('\n'),
        Some(c) => Ok(c),
        None => Err(MalError("Unexpected end of string!".to_string())),
    }
}

fn read_list(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "(");
    let list = read_list_inner(reader, ")")?;
    Ok(MalType::MalList(list.into()))
}

fn read_vector(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "[");
    let list = read_list_inner(reader, "]")?;
    Ok(MalType::Vector(list.into()))
}

fn read_list_inner(reader: &mut Reader, close: &str) -> Result<Vec<MalType>, MalError> {
    let mut list: Vec<MalType> = Vec::new();
    loop {
        if let Some(token) = reader.peek() {
            if token == close {
                reader.next();
                break;
            }
            match read_form(reader) {
                Err(MalError(e)) if e.is_empty() => {}
                Err(other) => return Err(other),
                Ok(form) => list.push(form),
            }
        } else {
            return Err(MalError("EOF while reading list".to_string()));
        }
    }
    Ok(list)
}

const NUMBER_MATCH: &str = r#"^\-?[\d\.]+$"#;

fn read_atom(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let num_re = Regex::new(NUMBER_MATCH).unwrap();
    let value = if num_re.is_match(&token) {
        MalType::Number(token.parse::<i64>().unwrap_or(0))
    } else {
        match token.as_ref() {
            "nil" => MalType::Nil,
            "true" => MalType::Bool(true),
            "false" => MalType::Bool(false),
            _ => MalType::Symbol(token),
        }
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let code = "(+ 2 (* 3 4))";
        let tokens = tokenizer(code);
        assert_eq!(
            tokens,
            vec![
                "(".to_string(),
                "+".to_string(),
                "2".to_string(),
                "(".to_string(),
                "*".to_string(),
                "3".to_string(),
                "4".to_string(),
                ")".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_read_str() {
        let code = "(nil true false :foo \"string\" (+ 2 (* 3 4)))";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::MalList(
                vec![
                    MalType::Nil,
                    MalType::Bool(true),
                    MalType::Bool(false),
                    MalType::Keyword("foo".to_string()),
                    MalType::Str("string".to_string()),
                    MalType::MalList(
                        vec![
                            MalType::Symbol("+".to_string()),
                            MalType::Number(2),
                            MalType::MalList(
                                vec![
                                    MalType::Symbol("*".to_string()),
                                    MalType::Number(3),
                                    MalType::Number(4),
                                ]
                                .into()
                            ),
                        ]
                        .into()
                    ),
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_read_vector() {
        let code = "[1 :foo nil]";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::Vector(
                vec![
                    MalType::Number(1),
                    MalType::Keyword("foo".to_string()),
                    MalType::Nil,
                ]
                .into()
            )
        );
    }

    // #[test]
    // fn test_hash_map() {
    //     let code = "{:foo 1 \"bar\" [2 3]}";
    //     let ast = read_str(code).unwrap();
    //     let mut map = BTreeMap::new();
    //     map.insert(MalType::keyword("foo"), MalType::number(1));
    //     map.insert(
    //         MalType::string("bar"),
    //         MalType::vector(vec![MalType::number(2), MalType::number(3)]),
    //     );
    //     assert_eq!(ast, MalType::hashmap(map));
    // }

    #[test]
    fn test_unclosed_string() {
        let code = "\"abc";
        let err = read_str(code).unwrap_err();
        assert_eq!(err, MalError("Unexpected EOF".to_string()));
    }

    // #[test]
    // fn test_quote() {
    //     let code = "('foo ~bar `baz ~@fuz @buz)";
    //     let ast = read_str(code).unwrap();
    //     assert_eq!(
    //         ast,
    //         MalType::list(vec![
    //             MalType::list(vec![MalType::symbol("quote"), MalType::symbol("foo")]),
    //             MalType::list(vec![MalType::symbol("unquote"), MalType::symbol("bar")]),
    //             MalType::list(vec![MalType::symbol("quasiquote"), MalType::symbol("baz")]),
    //             MalType::list(vec![
    //                 MalType::symbol("splice-unquote"),
    //                 MalType::symbol("fuz"),
    //             ]),
    //             MalType::list(vec![MalType::symbol("deref"), MalType::symbol("buz")]),
    //         ])
    //     );
    // }

    #[test]
    fn test_comment() {
        let code = "; comment";
        let err = read_str(code).unwrap_err();
        assert_eq!(err, MalError("".to_string()));
        let code = "[1] ; comment";
        let ast = read_str(code).unwrap();
        assert_eq!(ast, MalType::Vector(vec![MalType::Number(1)].into()));
        let code = "\"str\" ; comment";
        let ast = read_str(code).unwrap();
        assert_eq!(ast, MalType::Str("str".to_string()));
    }
}
