use super::ast;
use super::lexer::Token;
use super::lexer::{Lexer, LexerError};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Unknown,
}

impl From<LexerError> for ParserError {
    fn from(_input: LexerError) -> Self {
        Self::Unknown
    }
}

impl From<&LexerError> for ParserError {
    fn from(_input: &LexerError) -> Self {
        Self::Unknown
    }
}

type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'p> {
    source: &'p str,
    lexer: Peekable<Lexer<'p>>,
    statements: Vec<ast::Statement<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            lexer: Lexer::new(source.as_bytes()).peekable(),
            source,
            statements: vec![],
        }
    }

    pub fn parse(mut self) -> ast::Script<'p> {
        loop {
            match self.add_entry() {
                Ok(true) => {}
                Ok(false) => break,
                Err(err) => {
                    println!("Encountered an error: {:#?}", err);
                    break;
                }
            }
        }

        ast::Script {
            statements: self.statements.into_boxed_slice()
        }
    }

    fn add_entry(&mut self) -> ParserResult<bool> {
        loop {
            match self.lexer.next() {
                Some(Token::Error(err)) => {
                    return Err(err.into());
                },
                Some(decl @ Token::Var) => {
                    self.add_statement(decl)?;
                    return Ok(true);
                },
                Some(Token::Eol) => {}
                Some(t) => panic!("Unexpected Token: {:#?}", t),
                None => {
                    return Ok(false);
                }
            }
        }
    }

    fn add_statement(&mut self, decl: Token) -> ParserResult<()> {
        match decl {
            Token::Var => {
                let stmt = ast::Statement::VariableDeclaration(
                        self.get_variable_declaration(ast::DeclarationKind::Var)?
                );
                self.statements.push(stmt);
                Ok(())
            },
            _ => Err(ParserError::Unknown)
        }
    }

    fn get_variable_declaration(&mut self, kind: ast::DeclarationKind) -> ParserResult<ast::VariableDeclaration<'p>> {
        let mut declarators = vec![];

        loop {
            match self.lexer.peek() {
                Some(Token::Error(err)) => {
                    return Err(err.into());
                },
                Some(Token::Identifier(_)) => {
                    declarators.push(
                        self.get_variable_declarator()?
                    );
                },
                _ => {
                    break;
                }
            }
        }

        if Some(&Token::Semicolon) == self.lexer.peek() {
            self.lexer.next();
        }

        Ok(ast::VariableDeclaration {
            kind,
            declarators: declarators.into_boxed_slice()
        })
    }

    fn get_variable_declarator(&mut self) -> ParserResult<ast::VariableDeclarator<'p>> {
        let ident = if let Some(Token::Identifier(r)) = self.lexer.next() {
            ast::Identifier { name: &self.source[r] }
        } else {
            return Err(ParserError::Unknown);
        };

        if Some(Token::EqSign) != self.lexer.next() {
            return Err(ParserError::Unknown);
        };

        let init = self.maybe_get_expression()?;

        Ok(ast::VariableDeclarator {
            binding: ast::Binding::Identifier(ident),
            init,
        })
    }

    fn maybe_get_expression(&mut self) -> ParserResult<Option<ast::Expression<'p>>> {
        if let Some(Token::StringLiteral(r)) = self.lexer.peek() {
            let s = ast::LiteralString { value: &self.source[r.start..r.end] };
            self.lexer.next();
            Ok(Some(ast::Expression::LiteralString(s)))
        } else {
            Ok(None)
        }
    }
}
