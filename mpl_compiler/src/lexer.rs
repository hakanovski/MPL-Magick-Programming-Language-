//! Lexical Analyzer for MPL (Magick Programming Language)
//! 
//! Converts raw .ms (MagickScript) strings into discrete, immutable Tokens.
//! Designed for absolute determinism, this lexer skips noise (whitespace) 
//! and strictly categorizes logical intent into actionable systemic concepts.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // ---- Keywords (The Rites) ----
    Sacrifice, // Assign memory / Variable declaration
    Transmute, // Mutate state / Return logic
    Invoke,    // Execute function / Call external FFI

    // ---- Data Types (The Elements) ----
    SigilLiteral(String),  // A hashed or strict identifier literal
    FreqLiteral(f64),      // Floating-point harmonic value
    IntentString(String),  // String literal representing human phrasing

    // ---- Symbols (The Geometry) ----
    Identifier(String), // Variable/Function name
    Enshrine,  // '=' (Assignment)
    Resonate,  // '+' (Addition)
    Catalyze,  // '*' (Multiplication)
    OpenGate,  // '{' (Block start)
    CloseGate, // '}' (Block end)
    LeftParen, // '('
    RightParen,// ')'
    
    /// End of the ritual text.
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    /// Iteratively yields the next deterministic token from the script.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.current_char();

        match ch {
            '=' => { self.advance(); Token::Enshrine }
            '+' => { self.advance(); Token::Resonate }
            '*' => { self.advance(); Token::Catalyze }
            '{' => { self.advance(); Token::OpenGate }
            '}' => { self.advance(); Token::CloseGate }
            '(' => { self.advance(); Token::LeftParen }
            ')' => { self.advance(); Token::RightParen }
            '"' => self.lex_intent_string(),
            _ if ch.is_alphabetic() => self.lex_identifier_or_keyword(),
            _ if ch.is_digit(10) => self.lex_frequency(),
            _ => panic!("Entropic syntax error at byte {}: Unrecognized fractal '{}'", self.position, ch),
        }
    }

    /// Evaluates the active character slice.
    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    /// Marches the position forward by the exact UTF-8 byte length.
    fn advance(&mut self) {
        self.position += self.current_char().len_utf8();
    }

    /// Purges chaotic white-space noise from the probability stream.
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    /// Parses alphabetic strings into actionable keywords or dynamic identifiers.
    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        let value = &self.input[start..self.position];
        match value {
            "sacrifice" => Token::Sacrifice,
            "transmute" => Token::Transmute,
            "invoke" => Token::Invoke,
            _ => Token::Identifier(value.to_string()),
        }
    }

    /// Extracts numerical constructs (floating-point frequencies) targeting precise MLX parameters.
    fn lex_frequency(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && (self.current_char().is_digit(10) || self.current_char() == '.') {
            self.advance();
        }
        let value = &self.input[start..self.position];
        let freq: f64 = value.parse().expect("Failed to stabilize harmonic frequency parse");
        Token::FreqLiteral(freq)
    }

    /// Traps localized chaotic textual intent within double quotes.
    fn lex_intent_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        let value = &self.input[start..self.position];
        self.advance(); // Skip closing quote
        Token::IntentString(value.to_string())
    }
}
