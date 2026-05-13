//! Syntactic Parser for MPL (Magick Programming Language)
//! 
//! Transforms a linear stream of esoteric Tokens into a strictly-typed,
//! hierarchical Abstract Syntax Tree (AST). Operates with O(N) complexity 
//! to guarantee sub-millisecond translation into the Occult Virtual Machine.

use crate::ast::{Program, Statement, Expression, MplType};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /// Initializes the parser by capturing the first token from the Lexer stream.
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    /// Consumes the active token and advances the ritual state.
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Drives the core compilation phase, accumulating deterministic statements.
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }
        Program { statements }
    }

    /// Routes logic based on the root intent (Sacrifice vs Transmute).
    fn parse_statement(&mut self) -> Statement {
        match self.current_token {
            Token::Sacrifice => self.parse_sacrifice(),
            Token::Transmute => self.parse_transmute(),
            Token::Invoke => self.parse_invoke_statement(),
            _ => panic!("Invalid ritual construction: Phase disjoint. Expected statement primitive."),
        }
    }

    /// Parses variable assignment (Sacrifice) routing physical constants to memory.
    /// Syntax: `sacrifice <identifier> = <expression>`
    fn parse_sacrifice(&mut self) -> Statement {
        self.advance(); // Consume 'sacrifice'
        
        let identifier = match &self.current_token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Architectural Error: Expected valid identifier after 'sacrifice'"),
        };
        self.advance();

        if self.current_token != Token::Enshrine {
            panic!("Architectural Error: Expected '=' (Enshrine) binding in sacrifice statement");
        }
        self.advance();

        let value = self.parse_expression();

        Statement::Sacrifice { identifier, value }
    }

    /// Parses intent manifestation (Transmute) returning state to the ether.
    /// Syntax: `transmute <expression>`
    fn parse_transmute(&mut self) -> Statement {
        self.advance(); // consume 'transmute'
        let intent = self.parse_expression();
        Statement::Transmute { intent, target: "ether".to_string() }
    }

    /// Parses a direct function invocation that operates independently as a statement.
    /// Syntax: `invoke <target>()`
    fn parse_invoke_statement(&mut self) -> Statement {
        self.advance(); // consume 'invoke'
        let target = match &self.current_token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Architectural Error: Expected valid identifier for invocation target"),
        };
        self.advance();
        
        if self.current_token != Token::LeftParen {
            panic!("Architectural Error: Expected '(' for parameter bounding");
        }
        // Advance past parenthesis. Simplistic matching for MVP.
        self.advance();
        if self.current_token != Token::RightParen {
            panic!("Architectural Error: Currently invoke only supports 0 arity natively. Expected ')'");
        }
        self.advance();
        
        // Return an evaluation block or direct execution mapping in the AST.
        // For simplicity here, we wrap it in a pseudo Transmute/Execute statement.
        Statement::Transmute { 
            intent: Expression::Invocation { target: target.clone(), args: vec![] }, 
            target: target 
        }
    }

    /// Resolves literal values and primitive recursive vectors (expressions).
    fn parse_expression(&mut self) -> Expression {
        match self.current_token.clone() {
            Token::FreqLiteral(f) => {
                self.advance();
                Expression::Literal(MplType::Frequency(f))
            }
            Token::IntentString(s) => {
                self.advance();
                Expression::Literal(MplType::Intent(s))
            }
            Token::Identifier(id) => {
                self.advance();
                Expression::Variable(id)
            }
            _ => panic!("Architectural Error: Expected computable expression logic"),
        }
    }
}
