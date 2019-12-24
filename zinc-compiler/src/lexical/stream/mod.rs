//!
//! The lexical token stream.
//!

mod comment;
mod integer;
mod string;
mod symbol;
mod word;

pub use self::comment::parse as parse_comment;
pub use self::comment::Error as CommentParserError;
pub use self::integer::parse as parse_integer;
pub use self::integer::Error as IntegerParserError;
pub use self::string::parse as parse_string;
pub use self::string::Error as StringParserError;
pub use self::symbol::parse as parse_symbol;
pub use self::symbol::Error as SymbolParserError;
pub use self::word::parse as parse_word;
pub use self::word::Error as WordParserError;

use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::StringLiteral;
use crate::lexical::Token;

pub struct TokenStream {
    input: String,
    offset: usize,
    location: Location,
}

impl TokenStream {
    pub fn new(input: String) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new(1, 1),
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        let token = self.advance()?;
        log::trace!("{:?}", token);
        Ok(token)
    }

    ///
    /// The function algorithm checks if the character:
    /// 1. Is contained within the alphabet
    /// 2. Is a whitespace
    /// 3. Starts a comment
    /// 4. Starts a string literal
    /// 5. Starts a symbol (operator or delimiter)
    /// 6. Starts a number (decimal or hexadecimal)
    /// 7. Starts a word (keyword, boolean literal, or identifier)
    /// 8. Panics if non of the above, thus the alphabet must contain all the characters being
    /// passed to subscanners
    ///
    fn advance(&mut self) -> Result<Token, Error> {
        while let Some(character) = self.input.chars().nth(self.offset) {
            if character.is_ascii_whitespace() {
                if character == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else if character != '\r' {
                    self.location.column += 1;
                }
                self.offset += 1;
                continue;
            }

            if character == '/' {
                match parse_comment(&self.input[self.offset..]) {
                    Ok((size, lines, column, _comment)) => {
                        self.location.line += lines;
                        self.location.column = column;
                        self.offset += size;
                        continue;
                    }
                    Err(CommentParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(CommentParserError::NotAComment) => {}
                }
            }

            if character == '\"' {
                match parse_string(&self.input[self.offset..]) {
                    Ok((size, value)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::String(StringLiteral::new(value))),
                            location,
                        ));
                    }
                    Err(StringParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(StringParserError::NotAString) => {}
                }
            }

            if character.is_ascii_digit() {
                match parse_integer(&self.input[self.offset..]) {
                    Ok((size, integer)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        ));
                    }
                    Err(IntegerParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(IntegerParserError::NotAnInteger) => {}
                    Err(error) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::InvalidIntegerLiteral(location, error));
                    }
                }
            }

            if Identifier::can_start_with(character) {
                match parse_word(&self.input[self.offset..]) {
                    Ok((size, lexeme)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(lexeme, location));
                    }
                    Err(error) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::InvalidWord(location, error));
                    }
                }
            }

            match parse_symbol(&self.input[self.offset..]) {
                Ok((size, symbol)) => {
                    let location = Location::new(self.location.line, self.location.column);
                    self.location.column += size;
                    self.offset += size;
                    return Ok(Token::new(Lexeme::Symbol(symbol), location));
                }
                Err(SymbolParserError::UnexpectedEnd) => {
                    let location = Location::new(self.location.line, self.location.column);
                    return Err(Error::UnexpectedEnd(location));
                }
                Err(SymbolParserError::NotASymbol) => {}
                Err(error) => {
                    let location = Location::new(self.location.line, self.location.column);
                    return Err(Error::InvalidSymbol(location, error));
                }
            }
        }

        Ok(Token::new(Lexeme::Eof, self.location))
    }
}