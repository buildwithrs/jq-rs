use std::collections::HashMap;

/// A set of token will be used to parse json data
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Str(String),
    Number(f64),
    Bool(bool),
    Null,

    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    Comma, // ,
    Colon, // :
    Whitespace,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub struct Lexer {
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            if self.is_end() {
                break;
            }

            self.start = self.current;
            let tk = self.next_token()?;
            match tk {
                Token::Whitespace => continue,
                _ => tokens.push(tk),
            }
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.advance().ok_or_else(|| format!("Eof of Tokens!"))?;
        match ch {
            ' ' | '\n' | '\t' => Ok(Token::Whitespace),
            '"' => self.parse_str(),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            _ => {
                if ch.is_ascii_digit() {
                    self.parse_number(ch)
                } else if ch.is_alphabetic() {
                    self.parse_ident(ch)
                } else {
                    Err(format!("unknown char: {ch}"))
                }
            }
        }
    }

    fn parse_str(&mut self) -> Result<Token, String> {
        let mut is_end = false;
        let mut res = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                is_end = true;
                break;
            }

            res.push(ch);
            self.advance();
        }

        if !is_end && self.is_end() {
            return Err(format!("unterminated string"));
        }

        self.advance(); // consume right "
        Ok(Token::Str(res))
    }

    fn parse_number(&mut self, ch: char) -> Result<Token, String> {
        let mut number_text = String::new();
        number_text.push(ch);

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number_text.push(ch);
                self.advance();
                continue;
            }

            break;
        }

        let num = if let Ok(n) = number_text.parse::<f64>() {
            n
        } else {
            return Err(format!("invalid number"));
        };

        Ok(Token::Number(num))
    }

    fn parse_ident(&mut self, ch: char) -> Result<Token, String> {
        let mut ident = String::new();
        ident.push(ch);
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() || ch.eq(&'_') {
                ident.push(ch);
                self.advance();
                continue;
            }

            break;
        }
        match ident.as_str() {
            "true" => Ok(Token::Bool(true)),
            "false" => Ok(Token::Bool(false)),
            "null" => Ok(Token::Null),
            _ => Err(format!("unknown ident: {ident}")),
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            Some(self.chars[self.current])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            self.current += 1;
            Some(self.chars[self.current - 1])
        }
    }
    fn is_end(&self) -> bool {
        self.current >= self.chars.len()
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, String> {
        let tk = self.advance().ok_or_else(|| format!("End of Tokens"))?;
        match tk {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::Null => Ok(JsonValue::Null),
            Token::Str(s) => Ok(JsonValue::Str(s.clone())),
            Token::Number(n) => Ok(JsonValue::Number(n)),
            Token::Bool(b) => Ok(JsonValue::Bool(b)),
            _ => Err(format!("unknown token: {:?}", tk)),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut arr = Vec::new();
        loop {
            let tk = self.peek();
            if tk == Some(Token::RightBracket) || self.is_end() {
                break;
            }

            let val = self.parse_value()?;
            arr.push(val);
            if self.peek().is_some_and(|v| v.eq(&Token::RightBracket)) {
                break;
            }
            self.consume(Token::Comma)?;
        }

        self.consume(Token::RightBracket)?;
        Ok(JsonValue::Array(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut object = HashMap::new();
        loop {
            let tk = self.peek();
            if tk == Some(Token::RightBrace) || self.is_end() {
                break;
            }

            let key = self.parse_value()?;
            if !matches!(key, JsonValue::Str(_)) {
                return Err(format!("json key must be string"));
            }

            self.consume(Token::Colon)?;
            let value = self.parse_value()?;
            let key_s = if let JsonValue::Str(s) = key {
                s
            } else {
                return Err(format!("json key must be string"));
            };

            object.insert(key_s, value);
            if let Some(ch) = self.peek() {
                if ch == Token::RightBrace {
                    break;
                }

                self.consume(Token::Comma)?;
            }
        }
        self.consume(Token::RightBrace)?;

        Ok(JsonValue::Object(object))
    }

    fn consume(&mut self, tk: Token) -> Result<(), String> {
        let current = self.peek().ok_or_else(|| format!("end of tokens"))?;

        if current == tk {
            self.advance();
            Ok(())
        } else {
            Err(format!("expect {:?}, but get: {:?}", tk, current))
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        if self.is_end() {
            None
        } else {
            Some(self.tokens[self.current].clone())
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_end() {
            None
        } else {
            self.current += 1;
            Some(self.tokens[self.current - 1].clone())
        }
    }
}
