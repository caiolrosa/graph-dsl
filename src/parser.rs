use std::collections::HashMap;
use std::iter::Peekable;
use std::process::{exit};
use std::slice::{Iter};
use crate::token::Token;
use crate::graph::{Graph, Node, Edge};

pub struct Parser {
    graph: Graph
}

impl Parser {
    pub fn new() -> Self {
        return Parser {
            graph: Graph::new()
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        let mut token_iter = tokens.iter().peekable();
        while let Some(token) = token_iter.next() {
            match token {
                Token::GraphKeyword => self.graph_keyword(&mut token_iter),
                Token::IdentifierDeclaration(id) 
                    => self.identifier_declaration(id.to_string(), &mut token_iter),
                _ => unimplemented!("Default Case")
            }
        }
    }

    fn graph_keyword(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        let token = tokens.next().expect("Expected {{ or [ but found nothing.");

        if *token == Token::BracketOpen {
            let attr_tuple = self.attr_declaration(tokens);
            self.graph.attrs.insert(attr_tuple.0, attr_tuple.1);
        }

        if *token != Token::CurlyOpen {
            eprintln!("Expected {{ or [ but found {:?}.", token);
            exit(1);
        }
    }

    fn identifier_declaration(&self, id: String, tokens: &mut Peekable<Iter<Token>>) {

    }

    fn attr_declaration(&self, tokens: &mut Peekable<Iter<Token>>) -> (String, String) {
        let mut attr_key = String::default();
        let mut attr_val = String::default();
        while let Some(t) = tokens.next() {
            match t {
                Token::ColorKeyword => { 
                    attr_key = "color".to_string();
                    self.expect_token(tokens, Token::Equals);
                },
                Token::BGColorKeyword => {
                    attr_key = "bgcolor".to_string();
                    self.expect_token(tokens, Token::Equals);
                },
                Token::IdentifierDeclaration(id) => {
                    attr_val = id.clone();
                    self.expect_token(tokens, Token::Quotation);
                },
                Token::BracketClose => {
                    if attr_key.is_empty() || attr_val.is_empty() {
                        eprintln!("Invalid attribute declaration");
                        exit(1);
                    }
                    return (attr_key, attr_val);
                },
                _ => { 
                    eprintln!("Expected one of: ], =, \", keyword or indetifier but found {:?}", t);
                    exit(1);
                }
            }
        }

        eprintln!("Invalid syntax expected ]");
        exit(1);
    }

    fn expect_token(&self, tokens: &mut Peekable<Iter<Token>>, expected: Token) {
        let next = tokens.next()
                         .expect(format!("Expected {:?} but found nothing", expected).as_str());
        if *next != expected {
            eprintln!("Expected {:?} but found {:?}", expected, *next);
        }
    }
}