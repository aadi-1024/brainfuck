use std::fs;

use crate::errors;

#[derive(Clone,Default,Debug)]
pub enum TokenType {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    Write,
    Read,
    JumpNext,
    JumpBack,
    EOF,
    #[default]
    Comment
}

#[derive(Clone,Default,Debug)]
pub struct Token {
    pub index: usize,
    pub kind: TokenType,
}

#[derive(Debug)]
pub struct Tokenizer {
    data: Vec<u8>,
    cur_idx: usize,
}

impl Tokenizer {
    pub fn new(file: &str) -> Result<Tokenizer, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(file)?;
        let t = Tokenizer {
            data: data.into_bytes(),
            cur_idx: 0,
        };
        Ok(t)
    }

    fn read_next(&mut self) -> Result<Token, Box<dyn std::error::Error>>{
        if self.cur_idx > self.data.len() {
            return Err(Box::new(errors::EOF {}))
        } else if self.cur_idx == self.data.len() {
            let t = Ok(Token {
                index: self.cur_idx,
                kind: TokenType::EOF,
            });
            self.cur_idx += 1;
            return t;
        }

        let mut token = Token::default();
        token.index = self.cur_idx;
        token.kind = match self.data[self.cur_idx] {
            b'>' => TokenType::IncPtr,
            b'<' => TokenType::DecPtr,
            b'+' => TokenType::IncByte,
            b'-' => TokenType::DecByte,
            b'.' => TokenType::Write,
            b',' => TokenType::Read,
            b'[' => TokenType::JumpNext,
            b']' => TokenType::JumpBack,
             _ => TokenType::Comment,
        };

        self.cur_idx += 1;
        Ok(token)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(x) = self.read_next() {
            Some(x)
        } else {
            None
        }
    }
}

#[test]
pub fn test_read() {
    let t = Tokenizer::new("sample").unwrap();
    for i in t {
        println!("{:#?}", i);
    }
}