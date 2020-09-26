use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BracketOpen,
    BracketClose,
    CurlyOpen,
    CurlyClose,
    Equals,
    Quotation,
    Dash,
    GraphKeyword,
    BGColorKeyword,
    ColorKeyword,
    IdentifierDeclaration(String),
}

pub struct Tokenizer{}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer{};
    }

    pub fn parse(&mut self, dsl: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut chars = dsl.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                continue;
            }
            let token = self.parse_token(c, &mut chars);
            tokens.push(token);
        }

        return tokens;
    }

    fn parse_token(&mut self, curr_char: char, chars: &mut Peekable<Chars<'_>>) -> Token {
        if curr_char.is_alphanumeric() {
            let identifier = self.read_identifier(curr_char, chars);
            let token = self.parse_identifier(identifier);
            return token;
        }

        match curr_char {
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            '[' => Token::BracketOpen,
            ']' => Token::BracketClose,
            '=' => Token::Equals,
            '"' => Token::Quotation,
            '-' => Token::Dash,
            _ => panic!("Invalid syntax, unrecognized character {}", curr_char)
        }
    }

    fn parse_identifier(&self, token: String) -> Token {
        match token.to_lowercase().as_str() {
            "graph" => Token::GraphKeyword,
            "bgcolor" => Token::BGColorKeyword,
            "color" => Token::ColorKeyword,
            _ => Token::IdentifierDeclaration(token),
        }
    }

    fn read_identifier(&self, curr_char: char, chars: &mut Peekable<Chars<'_>>) -> String {
        let mut token = curr_char.to_string();
        while let Some(c) = chars.next() {
            if c.is_whitespace() || !c.is_alphanumeric() {
                break;
            }

            token.push(c);
            if let Some(n) = chars.peek() {
                if n.is_whitespace() || !n.is_alphanumeric() {
                    break;
                }
            }
        }

        return token;
    }
}