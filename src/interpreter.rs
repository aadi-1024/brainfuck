use std::collections::HashMap;
use std::io::Read;
use crate::tokens::{Token,TokenType,Tokenizer};
use crate::errors::RuntimeErr;

#[derive(Default,Debug)]
pub struct Interpreter {
    cur_idx: usize,
    cur_ptr: usize,
    memory: Vec<u8>,
    tokens: Vec<Token>,
    jump_indexes: HashMap<usize,usize>
}

impl Interpreter {
    pub fn new(file: &str) -> Result<Interpreter, Box<dyn std::error::Error>> {
        let mut i = Interpreter::default();
        let tokenizer = Tokenizer::new(file)?;

        //brainfuck by default has a memory of 30000 bytes
        i.memory.resize(30000, 0);
        i.tokens = tokenizer.into_iter().collect();
        //match parenthesis to calculate jump values, return error if doesnt exist
        let mut stk: Vec<Token> = Vec::new();
        for j in &i.tokens {
            match j.kind {
                TokenType::JumpNext => {
                    stk.push(j.clone());
                },
                TokenType::JumpBack => {
                    if let Some(x) = stk.last() {
                        if let TokenType::JumpNext = x.kind {
                            i.jump_indexes.insert(j.index, x.index);
                            i.jump_indexes.insert(x.index, j.index);
                            stk.pop();
                        } else {
                            return Err(Box::new(RuntimeErr { message: String::from("no matching jump")}));
                        }
                    } else {
                        return Err(Box::new(RuntimeErr { message: String::from("no matching jump")}));
                    }
                }
                _ => {},
            };
        }
        if !stk.is_empty() {
            return Err(Box::new(RuntimeErr { message: String::from("no matching jump")}));
        }
        Ok(i)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let token = &self.tokens[self.cur_idx];
            match token.kind {
                TokenType::DecByte => self.memory[self.cur_ptr] -= 1,
                TokenType::IncByte => self.memory[self.cur_ptr] += 1,
                TokenType::IncPtr => self.cur_ptr += 1,
                TokenType::DecPtr => self.cur_ptr -= 1,
                TokenType::Write => print!("{}", self.memory[self.cur_ptr] as char),
                TokenType::Read => {
                    let mut byte: [u8; 1] = [0];
                    std::io::stdin().read_exact(&mut byte).unwrap();
                    self.memory[self.cur_ptr] = byte[0];
                },
                TokenType::JumpNext => {
                    if self.memory[self.cur_ptr] == 0 {
                        self.cur_idx = self.jump_indexes.get(&self.cur_idx).unwrap().clone();
                    }
                },
                TokenType::JumpBack => {
                    if self.memory[self.cur_ptr] != 0 {
                        self.cur_idx = self.jump_indexes.get(&self.cur_idx).unwrap().clone();
                    }
                },
                TokenType::Comment => {},
                TokenType::EOF => {break},
            }
            self.cur_idx += 1;
        }
        Ok(())
    }
}