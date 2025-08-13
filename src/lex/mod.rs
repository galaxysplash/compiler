use crate::lex::cut::cut;
use crate::lex::tokenize::tokenize;
use crate::tokens::TokenKind;

mod cut;
mod tokenize;

#[inline]
pub fn lex(string: &str) -> Vec<TokenKind> {
    tokenize(cut(string))
}
