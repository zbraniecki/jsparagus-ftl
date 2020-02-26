use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Var,
    Identifier(Range<usize>),
    EqSign,
    StringLiteral(Range<usize>),
    Semicolon,
    Error(LexerError),
    Eol,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    Unknown,
}

type LexerResult = Result<Token, LexerError>;
type LexerOptionResult = Result<Option<Token>, LexerError>;

#[derive(Clone, Debug)]
pub struct Lexer<'l> {
    source: &'l [u8],
    ptr: usize,
    done: bool,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l [u8]) -> Self {
        Lexer {
            source,
            ptr: 0,
            done: false,
        }
    }

    fn get_ident(&mut self) -> Token {
        let start = self.ptr;
        self.ptr += 1;
        while let Some(b) = self.source.get(self.ptr) {
            if !b.is_ascii_alphanumeric() && *b != b'-' && *b != b'_' {
                break;
            }
            self.ptr += 1;
        }
        Token::Identifier(start..self.ptr)
    }

    fn is_var(&mut self, b: &u8) -> bool {
        b == &b'v' &&
            self.source.get(self.ptr + 1) == Some(&b'a') &&
            self.source.get(self.ptr + 2) == Some(&b'r')
    }

    fn skip_inline_ws(&mut self) {
        while let Some(b' ') = self.source.get(self.ptr) {
            self.ptr += 1;
        }
    }

    fn get_string_literal(&mut self, quote: &u8) -> LexerResult {
        let start = self.ptr;
        let end;
        loop {
            match self.source.get(self.ptr) {
                Some(b) if b == quote => {
                    end = self.ptr;
                    self.ptr += 1;
                    break;
                },
                None => {
                    return Err(LexerError::Unknown);
                },
                _ => {
                    self.ptr += 1;
                }
            }
        }

        Ok(Token::StringLiteral(start..end))
    }

    fn tokenize_script(&mut self) -> LexerOptionResult {
        self.skip_inline_ws();
        loop {
            match self.source.get(self.ptr) {
                Some(b'/') => {
                    self.ptr += 1;
                    if self.source.get(self.ptr) == Some(&b'/') {
                        self.ptr += 1;
                        while let Some(b) = self.source.get(self.ptr) {
                            self.ptr += 1;
                            if b == &b'\n' {
                                break;
                            }
                        }
                    }
                }
                Some(b) if b.is_ascii_alphabetic() => {
                    if self.is_var(b) {
                        self.ptr += 3;
                        return Ok(Some(Token::Var));
                    } else {
                        return Ok(Some(self.get_ident()));
                    }
                },
                Some(b';') => {
                    self.ptr += 1;
                    return Ok(Some(Token::Semicolon));
                },
                Some(b'=') => {
                    self.ptr += 1;
                    return Ok(Some(Token::EqSign));
                }
                Some(quote @ b'"') => {
                    self.ptr += 1;
                    return Ok(Some(self.get_string_literal(quote)?));
                }
                Some(b'\n') => {
                    self.ptr += 1;
                    return Ok(Some(Token::Eol));
                }
                None => {
                    return Ok(None);
                },
                _ => {
                    panic!("Unknown token!")
                }
            }
        }
    }

    #[inline]
    pub fn try_next(&mut self) -> LexerOptionResult {
        if self.done {
            Ok(None)
        } else {
            let result = self.tokenize_script();
            if result.is_err() || result == Ok(None) {
                self.done = true;
            }
            result
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().unwrap_or_else(|err| Some(Token::Error(err)))
    }
}
