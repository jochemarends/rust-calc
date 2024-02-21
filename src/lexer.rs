use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Over,
    End,
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut stream = TokenStream::from_str(text);
    
    loop {
        match stream.next() {
            Some(Token::End) => break,
            Some(token) => tokens.push(token),
            None => return Err("ERROR: invalid syntax"),
        }
    }

    Ok(tokens)
}

struct TokenStream<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> TokenStream<'a> {
    pub fn from_str(text: &'a str) -> Self {
        Self {
            chars: text.chars().peekable()
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let is_digit = |c: &char| c.is_digit(10);

        // have we reached the end of the buffer
        if self.chars.peek().is_none() {
            return Some(Token::End);
        }

        if is_digit(self.chars.peek().unwrap()) {
            let mut buffer = String::new();
            buffer.push(self.chars.next().unwrap());
            // read all consecutive digits
            while self.chars.peek().map_or(false, is_digit) {
                buffer.push(self.chars.next().unwrap());
            }
            return Some(Token::Number(buffer.parse().unwrap()));
        }

        match self.chars.next().unwrap() {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Times),
            '/' => Some(Token::Over),
            _ => None,
        }
    }
}

