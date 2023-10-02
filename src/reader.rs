use std::{iter::Peekable, rc::Rc};
use crate::mal_types::{MalResult, MalType, MalErr};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
  Ilegal,
  Eof,
  Equal,
  Plus,
  Minus,
  Star,
  Slash,
  Comma,
  Semicolon,
  Lparen,
  Rparen,
  Lbracket,
  Rbracket,
  String(String),
  Number(f64),
  Bool(bool),
  Ident(String),
  Nil
}


pub struct Lexer {
  input: String,
  pos: usize,
  rpos: usize,
  ch: u8
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let mut lex: Lexer = Self { input, pos: 0, rpos: 0, ch: 0 };
    lex.read_char();
    lex
  }

  fn read_char(&mut self) {
    if self.rpos >= self.input.len() {
      self.ch = 0;
    } else {
        self.ch = *self.input.as_bytes().get(self.rpos).unwrap();
    }
    self.pos = self.rpos;
    self.rpos += 1;
  }

  fn next_token(&mut self) -> Token {
    self.skip_whitspace();
    let tok: Token = match self.ch {
        b'=' => Token::Equal,
        b',' => Token::Comma,
        b'+' => Token::Plus,
        b'-' => Token::Minus,
        b'*' => Token::Star,
        b'/' => Token::Slash,
        b';' => {
          self.read_comment();
          Token::Nil
        },
        b'(' => Token::Lparen,
        b'[' => Token::Lbracket,
        b')' => Token::Rparen,
        b']' => Token::Rbracket,
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
          let ident = self.read_ident();
          match ident.as_str() {
              "false" => Token::Bool(false),
              "true" => Token::Bool(true),
              "nil" => Token::Nil,
              _ => Token::Ident(ident)
          }
        }
        b'0'..=b'9' => Token::Number(self.read_int()),
        0 => Token::Eof,
        _ => Token::Ilegal
    };
    tok
  }

  fn read_int(&mut self) -> f64 {
    let pos: usize = self.pos;
    while self.is_digit() || self.ch == b'.' {
        self.read_char();
    }
    self.rpos -= 1;
    self.input
    .get(pos..self.pos).unwrap()
    .parse::<f64>().unwrap()
  }

  fn read_ident(&mut self) -> String {
    let pos: usize = self.pos;
    while self.is_letter() {
        self.read_char()
    }
    self.rpos -= 1;
    self.input
    .get(pos..self.pos)
    .unwrap().to_string()
  }

  fn read_comment(&mut self) {
    let pos: usize = self.pos;
    while !(self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' || self.ch == 0) {
        self.read_char()
    }
    self.rpos -= 1;
  }
  fn skip_whitspace(&mut self) {
    while self.ch == b'\n' || self.ch == b' ' || self.ch == b'\t' || self.ch == b'\r' {
        self.read_char();
    }
  }

  fn is_letter(&self) -> bool {
    (b'a'..=b'z').contains(&self.ch) || (b'A'..=b'Z').contains(&self.ch) || self.ch == b'_'
  }

  fn is_digit(&self) -> bool {
    b'0' <= self.ch && self.ch <= b'9'
  }

}

// b'a'..=b'z' | b'A'..=b'Z' | b'_'
// b'0'..=b'9'


impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        //println!("ch antes next token: {}",self.ch);
        let curr = self.next_token();
        if curr == Token::Eof { return None; }
        //println!("ch depois next token: {}",self.ch);
        self.read_char();
        return Some(curr);
    }
}


pub fn read_str(s: String) -> MalResult {
    let mut reader: Peekable<Lexer> = Lexer::new(s).peekable();
    read_form(&mut reader)
}

fn read_form(r: &mut Peekable<Lexer>) -> MalResult {
    match r.peek() {
        Some(t) => {
            match *t {
                Token::Lparen => read_list(r),
                _ => read_atom(r)
            }
            
        },
        None => Err(MalErr),
    }
}

fn read_list(r: &mut Peekable<Lexer>) -> MalResult {
    let mut list: Vec<MalType> = Vec::new();
    r.next();
    loop {
        match r.peek() {
            Some(t) => {
                match t {
                    Token::Rparen => break,
                    _ => ()
                }
            },
            None => return Err(MalErr),
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
            match t {
                Token::Ident(s) => Ok(MalType::Symbol(Rc::from_iter(s.bytes()))),
                Token::Equal => Ok(MalType::Symbol(Rc::from_iter("=".bytes()))),
                Token::Plus => Ok(MalType::Symbol(Rc::from_iter("+".bytes()))),
                Token::Minus => Ok(MalType::Symbol(Rc::from_iter("-".bytes()))),
                Token::Star => Ok(MalType::Symbol(Rc::from_iter("*".bytes()))),
                Token::Slash => Ok(MalType::Symbol(Rc::from_iter("/".bytes()))),
                Token::Number(n) => Ok(MalType::Number(n)),
                Token::Bool(b) => Ok(MalType::Bool(b)),
                Token::Nil => Ok(MalType::Nil),
                _ => Err(MalErr),
            }
        },
        None => return Err(MalErr),
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{Token, Lexer};

    use super::read_str;

    #[test]
    fn it_works() {
        let line = String::from("(");
        let mut lex = Lexer::new(line);
        assert_eq!(Some(Token::Lparen),lex.next());
        assert_eq!(None,lex.next());
    }

    #[test]
    fn test_lex() {
      let ipt = String::from("(1)");
      let mut lex = Lexer::new(ipt);
      assert_eq!(Some(Token::Lparen),lex.next());
      assert_eq!(Some(Token::Number(1.0)),lex.next());
      assert_eq!(Some(Token::Rparen),lex.next());

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