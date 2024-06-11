use std::fs;

use crate::errors;

#[derive(Default,Debug)]
pub enum TokenType {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    Write,
    Read,
    JumpNext,
    JumpBack,
    #[default]
    Comment
}

#[derive(Default,Debug)]
pub struct Token {
    index: usize,
    kind: TokenType,
}

#[derive(Debug)]
pub struct Tokenizer {
    data: Vec<u8>,
    cur_idx: usize,
    result: Vec<Token>
}

impl Tokenizer {
    pub fn new(file: &str) -> Result<Tokenizer, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(file)?;
        let t = Tokenizer {
            data: data.into_bytes(),
            cur_idx: 0,
            result: vec![],
        };
        Ok(t)
    }

    fn read_next(&mut self) -> Result<Token, Box<dyn std::error::Error>>{
        if self.cur_idx == self.data.len() {
            return Err(Box::new(errors::EOF {}))
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

#[test]
pub fn test_read() {
    let mut t = Tokenizer::new("sample").unwrap();
    while let Ok(x) = t.read_next() {
        println!("{:#?}", x);
    }
}