use chumsky::prelude::*;

use crate::lexer::token::Token;

use super::expr::{Expr, ExprRecursive};

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Neg,
}

pub(super) fn unary_parser<'a>(
    expr: ExprRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    let minus = just(Token::Minus)
        .repeated()
        .at_least(1)
        .then(expr)
        .foldr(|_, rhs| Expr::Unary {
            op: UnaryOp::Neg,
            right: Box::new(rhs),
        });

    minus
}
