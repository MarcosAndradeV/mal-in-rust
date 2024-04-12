use core::fmt;
use std::{iter::Peekable, rc::Rc};
use crate::mal_types::{MalResult, MalType, MalErr};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind {
    Colon,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    String,
    Number,
    Bool,
    Symbol,
    Nil,
    EOF,
    Ilegal,
}

#[derive(Debug)]
pub struct Token {
  kind: TokenKind,
  value: Option<String>
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = &self.value {
            write!(f, "{:?}({})", self.kind, s)
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}
impl Token {
    fn new(kind: TokenKind, value: Option<String>) -> Token {
        Self { kind, value }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

pub struct Lexer {
  input: Vec<u8>,
  pos: usize,
  read_pos: usize,
  curr_ch: u8,
}

impl Lexer {
  pub fn new(input: String) -> Self {
      let mut lex = Self {
          input: input.into_bytes(),
          pos: 0,
          read_pos: 0,
          curr_ch: 0,
      };
      lex.read_ch();
      lex
  }
  pub fn read_ch(&mut self) {
      if self.read_pos >= self.input.len() {
          self.curr_ch = 0
      } else {
          self.curr_ch = self.input[self.read_pos]
      }
      self.pos = self.read_pos;
      self.read_pos += 1;
  }
  pub fn peek_ch(&mut self) -> u8 {
      if self.read_pos >= self.input.len() {
          return 0;
      } else {
          return self.input[self.read_pos];
      }
  }
  pub fn skip_whitespace(&mut self) {
      loop {
          if !matches!(self.curr_ch, b' ' | b'\t' | b'\n' | b'\r') {
              break;
          }
          self.read_ch()
      }
  }
  pub fn skip_comment(&mut self) {
    loop {
        if !matches!(self.curr_ch, b'\n') {
            break;
        }
        self.read_ch()
    }
}

  fn keyword_or_identifier(&mut self) -> Token {
      let pos = self.pos;
      loop {
          if matches!(self.curr_ch, b'a'..=b'z' | b'A'..=b'Z' | b'_') {
              self.read_ch(); continue;
          }
          break;
      }
      let s = String::from_utf8_lossy(&self.input[pos..self.pos]).into_owned();
      match s.len() {
          // 2 => {
          //     match s.as_str() {
          //         //"fn" => Token::new(TokenKind::Function, s),
          //         //"if" => Token::new(TokenKind::If, s),
          //         _ => Token::new(TokenKind::Ident, Some(s))
          //     }
          // }
        3 => {
            match s.as_str() {
                "nil" => Token::new(TokenKind::Nil, Some(s)),
                _ => Token::new(TokenKind::Symbol, Some(s))
            }
        }
          // 4 => {
          //     match s.as_str() {
          //         //"else" => Token::new(TokenKind::Else, s),
          //         //"true" => Token::new(TokenKind::True, s),
          //         _ => Token::new(TokenKind::Ident, s)
          //     }
          // }
          // 5 => {
          //     match s.as_str() {
          //         //"false" => Token::new(TokenKind::False, s),
          //         _ => Token::new(TokenKind::Ident, s)
          //     }
          // }
          // 6 => {
          //     match s.as_str() {
          //         //"return" => Token::new(TokenKind::Return, s),
          //         _ => Token::new(TokenKind::Ident, s)
          //     }
          // }
          _ => Token::new(TokenKind::Symbol, Some(s))
      }
  }

  fn string(&mut self) -> Token {
    let mut buffer = String::new();
    self.read_ch();
    loop {
        match self.curr_ch {
            0 => return Token::new(TokenKind::Ilegal, None),
            b'\"' => {
                self.read_ch();
                break;
            }
            b'\\' => {self.read_ch(); match self.curr_ch {
                b'n' => {
                    buffer.push('\n');
                    self.read_ch();
                    self.read_ch();
                }
                _ => {
                    buffer.push(self.curr_ch as char);
                    self.read_ch()
                }
            }},
            b'\n' => return Token::new(TokenKind::Ilegal, None),
            _ => {
                buffer.push(self.curr_ch as char);
                self.read_ch()
            }
        }
    }
    Token::new(TokenKind::String, Some(buffer))
  }

  pub fn next_token(&mut self) -> Token {
      self.skip_whitespace();
      let tok: Token =
      match self.curr_ch {
          b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.keyword_or_identifier(),
          b'0'..=b'9' =>  self.number(),
          b'(' => Token::new(TokenKind::Lparen, None),
          b')' => Token::new(TokenKind::Rparen, None),
          b'[' => Token::new(TokenKind::Lbracket, None),
          b']' => Token::new(TokenKind::Rbracket, None),
          b':' => todo!(), //Token::new(TokenKind::Colon, None), // TODO: macros
          b';' => {self.skip_comment(); Token::new(TokenKind::Nil, None)},
          b'"' => self.string(),
          0 => Token::new(TokenKind::EOF, None),
          _ => Token::new(TokenKind::Symbol, Some((self.curr_ch as char).to_string()))
      };
      self.read_ch();
      tok
  }

  fn number(&mut self) -> Token {
      let pos: usize = self.pos;
      while matches!(self.curr_ch, b'0'..=b'9') {
          self.read_ch();
      }
      self.read_pos -= 1;
      Token::new(TokenKind::Number, Some(String::from_utf8_lossy(&self.input[pos..self.pos]).into_owned()))
  }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.kind == TokenKind::EOF { return None; }
        return Some(tok);
    }
}


pub fn read_str(s: String) -> MalResult {
    let mut reader = Lexer::new(s).peekable();
    read_form(&mut reader)
}

fn read_form(r: &mut Peekable<Lexer>) -> MalResult {
    match r.peek() {
        Some(t) => {
            match t.kind {
                TokenKind::Lparen => read_list(r),
                TokenKind::Lbracket => read_vec(r),
                _ => read_atom(r)
            }
            
        },
        None => Err(MalErr(format!("Expected: Something found EOF"))),
    }
}

fn read_vec(r: &mut Peekable<Lexer>) -> MalResult {
    let mut list: Vec<MalType> = Vec::new();
    r.next();
    loop {
        match r.peek() {
            Some(t) => {
                match t.kind {
                    TokenKind::Rbracket => break,
                    _ => ()
                }
            },
            None => return Err(MalErr(format!("Expected: List found EOF"))),
        };
        match read_form(r) {
            Ok(ok) => list.push(ok),
            Err(e) => return Err(e)
        }
    }
    r.next();
    Ok(MalType::Vector(Rc::from_iter(list)))
}

fn read_list(r: &mut Peekable<Lexer>) -> MalResult {
    let mut list: Vec<MalType> = Vec::new();
    r.next();
    loop {
        match r.peek() {
            Some(t) => {
                match t.kind {
                    TokenKind::Rparen => break,
                    _ => ()
                }
            },
            None => return Err(MalErr(format!("Expected: List found EOF"))),
        };
        match read_form(r) {
            Ok(ok) => list.push(ok),
            Err(e) => return Err(e)
        }
    }
    r.next();
    Ok(MalType::MalList(Rc::from_iter(list)))
}

fn read_atom(r: &mut Peekable<Lexer>) -> MalResult {
    match r.next() {
        Some(t) => {
            match t.kind {
                TokenKind::Symbol => Ok(MalType::Symbol(t.value.expect("Empyt symbol"))),
                TokenKind::String => Ok(MalType::Str(t.value.expect("Empyt symbol"))),
                TokenKind::Nil => Ok(MalType::Nil),
                TokenKind::Number => Ok(MalType::Number(t.value.expect("Empty Number")
                .parse::<f64>().expect("Error"))),
                _ => Err(MalErr(format!("Unimplemented: {}", t))),
            }
        },
        None => Err(MalErr(format!("Expected: atom found EOF"))),
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{Token, Lexer, TokenKind};

    use super::read_str;

    #[test]
    fn it_works() {
        let line = String::from("(");
        let mut lex = Lexer::new(line);
        assert_eq!(Some(Token {kind: TokenKind::Lparen, value: None}),lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
    }

    #[test]
    fn test_lex() {
      let ipt = String::from("(1)");
      let mut lex = Lexer::new(ipt);
      assert_eq!(Some(Token {kind: TokenKind::Lparen, value: None}),lex.next());
      assert_eq!(Some(Token {kind: TokenKind::Number, value: Some("1".to_string())}), lex.next());
      assert_eq!(Some(Token {kind: TokenKind::Rparen, value: None}),lex.next());

    }
    #[test]
    fn read_from_test() {
      let ipt = String::from("(1)");
      assert!(read_str(ipt).is_ok());
      let ipt = String::from("+");
      assert!(read_str(ipt).is_ok());
      let ipt = String::from("()");
      assert!(read_str(ipt).is_ok());
      let ipt = String::from("(())");
      assert!(read_str(ipt).is_ok());
      let ipt = String::from("(");
      assert!(read_str(ipt).is_err());
    }

}