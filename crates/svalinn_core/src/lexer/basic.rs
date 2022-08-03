use chumsky::prelude::*;

use super::token::Token;

pub(super) fn basic_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    let arrow = just("=>").to(Token::Arrow);
    let colon = just("::")
        .to(Token::DoubleColon)
        .or(just(':').to(Token::Colon));

    let ops = one_of("+-*/")
        .map_with_span(|c, _span| match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            _ => unreachable!(),
        })
        .labelled("operators");

    let other = one_of("=,").map_with_span(|c, _span| match c {
        '=' => Token::Equal,
        ',' => Token::Comma,
        _ => unreachable!(),
    });

    arrow.or(colon).or(ops).or(other)
}
