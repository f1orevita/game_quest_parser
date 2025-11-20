use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Expected {expected}, found {found}")]
    SyntaxError { expected: String, found: String },
    #[error("Invalid number format")]
    InvalidNumber,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Default)]
pub struct Quest {
    pub name: String,
    pub steps: Vec<String>,
    pub reward: i32,
    pub active: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    QuestKeyword,
    Identifier(String),
    StringLiteral(String),
    Number(i32),
    LBrace,
    RBrace,
    Colon,
    Comma,
    True,
    False,
    Eof,
}

struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }

        match self.input.next() {
            None => Ok(Token::Eof),
            Some('{') => Ok(Token::LBrace),
            Some('}') => Ok(Token::RBrace),
            Some(':') => Ok(Token::Colon),
            Some(',') => Ok(Token::Comma),
            Some('"') => self.read_string(),
            Some(c) if c.is_alphabetic() => self.read_identifier(c),
            Some(c) if c.is_ascii_digit() || c == '-' => self.read_number(c),
            Some(c) => Err(ParseError::UnexpectedChar(c)),
        }
    }

    fn read_string(&mut self) -> Result<Token, ParseError> {
        let mut s = String::new();
        while let Some(&c) = self.input.peek() {
            if c == '"' {
                self.input.next();
                return Ok(Token::StringLiteral(s));
            }
            s.push(self.input.next().unwrap());
        }
        Err(ParseError::UnexpectedEOF)
    }

    fn read_identifier(&mut self, first: char) -> Result<Token, ParseError> {
        let mut ident = String::from(first);
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "quest" => Ok(Token::QuestKeyword),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            _ => Ok(Token::Identifier(ident)),
        }
    }

    fn read_number(&mut self, first: char) -> Result<Token, ParseError> {
        let mut num_str = String::from(first);
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                num_str.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        let num = num_str
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidNumber)?;
        Ok(Token::Number(num))
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Self {
            lexer,
            current_token,
        })
    }

    fn eat(&mut self, expected: Token) -> Result<(), ParseError> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(ParseError::SyntaxError {
                expected: format!("{:?}", expected),
                found: format!("{:?}", self.current_token),
            })
        }
    }

    /// Parses the main Quest definition.
    ///
    /// # Grammar Rule
    /// ```ebnf
    /// QUEST_DEF ::= "quest" (IDENTIFIER | STRING) "{" BODY "}"
    /// ```
    ///
    /// This is the entry point for the parser. It expects the keyword `quest`,
    /// followed by a name, and then a block of properties enclosed in curly braces.
    pub fn parse_quest(&mut self) -> Result<Quest, ParseError> {
        self.eat(Token::QuestKeyword)?;

        let quest_name = match &self.current_token {
            Token::Identifier(name) | Token::StringLiteral(name) => name.clone(),
            _ => {
                return Err(ParseError::SyntaxError {
                    expected: "Identifier or String".to_string(),
                    found: format!("{:?}", self.current_token),
                })
            }
        };
        self.current_token = self.lexer.next_token()?;

        self.eat(Token::LBrace)?;

        let mut quest = Quest {
            name: quest_name,
            ..Default::default()
        };

        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            self.parse_property(&mut quest)?;
            if self.current_token == Token::Comma {
                self.eat(Token::Comma)?;
            }
        }

        self.eat(Token::RBrace)?;
        Ok(quest)
    }

    /// Parses individual properties inside the Quest body.
    ///
    /// # Grammar Rule
    /// ```ebnf
    /// PROPERTY ::= KEY ":" VALUE
    /// KEY      ::= "reward" | "active" | "step"
    /// VALUE    ::= INTEGER | BOOLEAN | STRING
    /// ```
    ///
    /// Handles specific keys:
    /// - `reward`: Expects an integer number.
    /// - `active`: Expects a boolean (`true`/`false`).
    /// - `step`: Expects a string literal (can be repeated).
    fn parse_property(&mut self, quest: &mut Quest) -> Result<(), ParseError> {
        let key = match &self.current_token {
            Token::Identifier(k) => k.clone(),
            _ => {
                return Err(ParseError::SyntaxError {
                    expected: "Property Key".to_string(),
                    found: format!("{:?}", self.current_token),
                })
            }
        };
        self.current_token = self.lexer.next_token()?;

        self.eat(Token::Colon)?;

        match key.as_str() {
            "reward" => {
                if let Token::Number(n) = self.current_token {
                    quest.reward = n;
                    self.current_token = self.lexer.next_token()?;
                } else {
                    return Err(ParseError::SyntaxError {
                        expected: "Number".into(),
                        found: format!("{:?}", self.current_token),
                    });
                }
            }
            "active" => {
                match self.current_token {
                    Token::True => quest.active = true,
                    Token::False => quest.active = false,
                    _ => {
                        return Err(ParseError::SyntaxError {
                            expected: "Bool".into(),
                            found: format!("{:?}", self.current_token),
                        })
                    }
                }
                self.current_token = self.lexer.next_token()?;
            }
            "step" => {
                if let Token::StringLiteral(s) = &self.current_token {
                    quest.steps.push(s.clone());
                    self.current_token = self.lexer.next_token()?;
                }
            }
            _ => {
                self.current_token = self.lexer.next_token()?;
            }
        }
        Ok(())
    }
}
