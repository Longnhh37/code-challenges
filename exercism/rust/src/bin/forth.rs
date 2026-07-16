use std::collections::HashMap;
use std::rc::Rc;

// ---------- program setup -------------
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

// ---------- lexer + parser -------------
#[derive(Clone)]
enum Token {
    Number(i32),
    Ops(Ops),
    Math(Math),
    Word(Rc<Vec<Token>>),
}

#[derive(Clone, Copy)]
enum Ops {
    Dup,
    Drop,
    Swap,
    Over,
}

#[derive(Clone, Copy)]
enum Math {
    Add,
    Minus,
    Mul,
    Div,
}

// ---------- business logic -------------
pub struct Forth {
    stack: Vec<i32>,
    map: HashMap<String, Rc<Vec<Token>>>,
}

impl Forth {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let words: Vec<&str> = input.split_ascii_whitespace().collect();
        let tokens = self.compile(&words)?;
        self.run(&tokens)
    }

    // ---------- helpers -------------
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn run(&mut self, tokens: &[Token]) -> Result {
        for tok in tokens {
            match tok {
                Token::Number(n) => self.stack.push(*n),
                Token::Math(m) => self.math_handle(m)?,
                Token::Ops(o) => self.ops_handle(o)?,
                Token::Word(def) => self.run(&def.clone())?,
            }
        }
        Ok(())
    }

    fn compile(&mut self, words: &[&str]) -> std::result::Result<Vec<Token>, Error> {
        let mut output = Vec::new();
        let mut i = 0;
        while i < words.len() {
            let cur = words[i].to_ascii_lowercase();
            if cur == ":" {
                let end = words[i + 1..]
                    .iter()
                    .position(|&w| w == ";")
                    .map(|p| p + i + 1)
                    .ok_or(Error::InvalidWord)?;
                self.define_word(&words[i + 1..end])?;
                i = end + 1;
                continue;
            }
            output.push(self.token_for(&cur)?);
            i += 1;
        }
        Ok(output)
    }

    fn token_for(&self, word: &str) -> std::result::Result<Token, Error> {
        use Math::*;
        use Ops::*;

        if let Ok(n) = word.parse::<i32>() {
            return Ok(Token::Number(n));
        }

        if let Some(def) = self.map.get(&word.to_ascii_lowercase()) {
            return Ok(Token::Word(Rc::clone(def)));
        }

        let tok = match word {
            "+" => Token::Math(Add),
            "-" => Token::Math(Minus),
            "*" => Token::Math(Mul),
            "/" => Token::Math(Div),
            "dup" => Token::Ops(Dup),
            "drop" => Token::Ops(Drop),
            "swap" => Token::Ops(Swap),
            "over" => Token::Ops(Over),
            _ => return Err(Error::UnknownWord),
        };

        Ok(tok)
    }

    fn math_handle(&mut self, ops: &Math) -> Result {
        use Math::*;
        if self.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        match ops {
            Add => self.stack.push(left + right),
            Minus => self.stack.push(left - right),
            Mul => self.stack.push(left * right),
            Div => {
                if right == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack.push(left / right);
            }
        }

        Ok(())
    }

    fn ops_handle(&mut self, ops: &Ops) -> Result {
        use Ops::*;
        match ops {
            Dup | Drop if self.stack.is_empty() => return Err(Error::StackUnderflow),
            Swap | Over if self.len() < 2 => return Err(Error::StackUnderflow),
            Dup => self.stack.push(self.stack[self.len() - 1]),
            Drop => {
                self.stack.pop().unwrap();
            }
            Swap => {
                let i = self.len() - 2;
                let j = i + 1;
                self.stack.swap(i, j);
            }
            Over => self.stack.push(self.stack[self.len() - 2]),
        }

        Ok(())
    }

    fn define_word(&mut self, def_words: &[&str]) -> Result {
        let (name, body) = def_words.split_first().ok_or(Error::InvalidWord)?;
        if name.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let compiled = self.compile(body)?;
        self.map
            .insert(name.to_ascii_lowercase(), Rc::new(compiled));
        Ok(())
    }
}

fn main() {}
