use crate::{app::app, lex::lex, tokens::TokenKind};

mod app;
mod lex;
mod mode;
mod pos;
mod tokens;

fn main() {
    app(|instruction: String| {
        let tokens: Vec<TokenKind> = lex(&instruction);

        for token in tokens {
            match token {
                TokenKind::If => println!("if"),
                TokenKind::Assignment => println!(":="),
                _ => println!("???"),
            }
        }

        todo!("change this ^^^^^^^^^^")
    });
}
