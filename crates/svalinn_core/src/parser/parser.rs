use crate::{common::Spanned, lexer::token::Token, prelude::SvResult};
use chumsky::{prelude::*, Parser as ChumskyParser, Stream};

use super::stmt::{statement_parser, Stmt};

pub(crate) struct Parser<'a>(BoxedParser<'a, Token, Vec<Stmt>, Simple<Token>>);

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let parser = recursive(|stmt| statement_parser(stmt)).repeated();
        Self(parser.then_ignore(end()).boxed())
    }

    // TODO: Return spanned Stmt
    pub fn parse(&self, tokens: Vec<Spanned<Token>>) -> SvResult<Vec<Stmt>, Token> {
        let len = tokens.len();
        let tokens = Stream::from_iter(len..len + 1, tokens.into_iter());
        self.0.parse(tokens)
    }
}