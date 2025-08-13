use crate::tokens::TokenKind;

pub fn tokenize(cut_strings: Vec<String>) -> Vec<TokenKind> {
    let mut tokens: Vec<TokenKind> = vec![];

    for cut_string in cut_strings {
        tokens.push(match cut_string.as_str() {
            "if" => TokenKind::If,
            " " => TokenKind::Space,

            "collection" => TokenKind::Collection,

            "=" => TokenKind::Eq,
            "<>" => TokenKind::UnEq,

            ":=" => TokenKind::Assignment,

            "(" => TokenKind::OpenBracket,
            ")" => TokenKind::CloseBracket,

            "{" => TokenKind::OpenBrace,
            "}" => TokenKind::CloseBrace,

            "\n" => TokenKind::NewLine,

            _ => TokenKind::Identifier(cut_string),
        })
    }

    tokens
}
