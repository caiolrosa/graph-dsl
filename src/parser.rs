use std::iter::Peekable;
use std::process::{exit};
use std::slice::{Iter};
use crate::token::Token;
use crate::graph::{Graph, Node, Edge};

pub enum ParserError {
    ExpectedToken(String, Token),
    UnexpectedToken(Token),
    InvalidAttributeDeclaration(String),
    InvalidEdgeDeclaration(String)
}

pub struct Parser {
    graph: Graph
}

impl Parser {
    pub fn new() -> Self {
        return Parser {
            graph: Graph::new()
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> &Graph {
        let mut token_iter = tokens.iter().peekable();
        while let Some(token) = token_iter.next() {
            match token {
                Token::GraphKeyword => self.graph_keyword(&mut token_iter),
                Token::IdentifierDeclaration(id) 
                    => self.identifier_declaration(id.to_string(), &mut token_iter),
                Token::CurlyClose => continue,
                _ => self.parse_error(ParserError::UnexpectedToken(token.clone()))
            }
        }

        return &self.graph;
    }

    fn graph_keyword(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        let token = tokens.next().expect("Expected {{ or [ but found nothing.");

        if *token == Token::BracketOpen {
            let attr_tuple = self.attr_declaration(tokens);
            self.graph.attrs.insert(attr_tuple.0, attr_tuple.1);
            return;
        }

        if *token != Token::CurlyOpen {
            self.parse_error(ParserError::ExpectedToken("{ or [".to_string(), token.clone()));
        }
    }

    fn identifier_declaration(&mut self, id: String, tokens: &mut Peekable<Iter<Token>>) {
        let mut node = Node::with_label(id);
        while let Some(t) = tokens.next() {
            if *t == Token::BracketOpen {
                let attr_tuple = self.attr_declaration(tokens);
                node.attrs.insert(attr_tuple.0, attr_tuple.1);
                self.graph.nodes.push(node);
                return;
            }

            if *t == Token::Dash {
                if let Some(n) = tokens.next() {
                    if *n != Token::Dash {
                        self.parse_error(ParserError::ExpectedToken("-".to_string(), n.clone()))
                    }
                    let edge = self.edge_declaration(node, tokens);
                    self.graph.edges.push(edge);
                    return;
                }
            }
        }
    }

    fn edge_declaration(&mut self, left_node: Node, tokens: &mut Peekable<Iter<Token>>) -> Edge {
        let existing_node = self.graph.nodes.iter().find(|node| node.label == left_node.label);
        if let None = existing_node {
            self.parse_error(ParserError::InvalidEdgeDeclaration(left_node.label));
        }

        let mut edge = Edge::default();
        while let Some(t) = tokens.next() {
            match t {
                Token::IdentifierDeclaration(id) => {
                    let right_node = self.graph.nodes.iter().find(|node| node.label == *id);
                    if let None = right_node {
                        self.parse_error(ParserError::InvalidEdgeDeclaration(id.clone()));
                    }
                    edge = Edge::with_nodes(existing_node.unwrap().clone(), right_node.unwrap().clone());
                },
                Token::BracketOpen => {
                    let attr_tuple = self.attr_declaration(tokens);
                    edge.attrs.insert(attr_tuple.0, attr_tuple.1);
                    return edge;
                },
                _ => self.parse_error(ParserError::UnexpectedToken(t.clone()))
            }
        }

        eprintln!("Missing closing ]");
        exit(1);
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
                Token::Quotation => continue,
                Token::IdentifierDeclaration(id) => {
                    attr_val = id.clone();
                    self.expect_token(tokens, Token::Quotation);
                },
                Token::BracketClose => {
                    if attr_key.is_empty() || attr_val.is_empty() {
                        self.parse_error(ParserError::InvalidAttributeDeclaration("Attribute name and value cannot be empty".to_string()));
                    }
                    return (attr_key, attr_val);
                },
                _ => { 
                    self.parse_error(ParserError::ExpectedToken("one of: ], =, \", keyword or indetifier".to_string(), t.clone()));
                }
            }
        }

        eprintln!("Invalid syntax expected ]");
        exit(1);
    }

    fn expect_token(&self, tokens: &mut Peekable<Iter<Token>>, expected: Token) {
        let next = tokens.peek()
                         .expect(format!("Expected {:?} but found nothing", expected).as_str());
        if **next != expected {
            eprintln!("Expected {:?} but found {:?}", expected, **next);
            exit(1);
        }

        tokens.next();
    }

    fn parse_error(&self, error: ParserError) {
        eprint!("Parse Error: ");
        match error {
            ParserError::ExpectedToken(expected, found)
                => eprintln!("Expected {} but found {:?} instead.", expected, found),
            ParserError::InvalidAttributeDeclaration(message)
                => eprintln!("Invalid attribute declaration, {}", message),
            ParserError::InvalidEdgeDeclaration(token)
                => eprintln!("Can't create edge with non existing node {}", token),
            ParserError::UnexpectedToken(token)
                => eprintln!("Unexpected token {:?}", token)
        }

        exit(1);
    }
}