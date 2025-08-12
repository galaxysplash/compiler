use crate::{lexer::*, tokens::TokenKind};

mod lexer;
mod tokens;

fn main() {
    for tokens in lex("input { by usr }") {
        eprintln!(
            "{}\n",
            match tokens {
                TokenKind::End => String::from("end"),
                TokenKind::Eq => String::from("="),
                TokenKind::Identifier(content) => content,
                TokenKind::CloseBracket => String::from("}"),
                _ => String::from("..."),
            }
        );
    }
}
