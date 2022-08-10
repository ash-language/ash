use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::{expr::Expr, stmt::StmtRecursive};

pub(super) fn ident_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    filter_map(|span, tok| match tok {
        Token::Identifier { value, .. } => Ok(value.clone()),
        _ => Err(Simple::expected_input_found(
            span,
            vec![Some(Token::Identifier {
                value: "".to_owned(),
                space_sufix: false,
            })],
            Some(tok),
        )),
    })
}

pub(super) fn ident_with_suffix_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone
{
    filter_map(|span, tok| match tok {
        Token::Identifier {
            value,
            space_sufix: true,
        } => Ok(value.clone()),
        _ => Err(Simple::expected_input_found(
            span,
            vec![Some(Token::Identifier {
                value: "".to_owned(),
                space_sufix: false,
            })],
            Some(tok),
        )),
    })
}

pub(super) fn block_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    just(Token::LBrace)
        .ignore_then(stmt.repeated())
        .then_ignore(just(Token::RBrace))
        .map(Expr::Block)
}
