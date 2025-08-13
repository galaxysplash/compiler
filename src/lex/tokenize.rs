use crate::tokens::TokenKind;

pub fn tokenize(cut_strings: Vec<String>) -> Vec<TokenKind> {
    let mut tokens: Vec<TokenKind> = vec![];

    for cut_string in cut_strings {
        tokens.push(match cut_string.as_str() {
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "end" => TokenKind::End,

            "struct" => TokenKind::Struct,

            "=" => TokenKind::Eq,
            "<>" => TokenKind::UnEq,

            ":=" => TokenKind::Assignment,

            "(" => TokenKind::OpenBracket,
            ")" => TokenKind::CloseBracket,

            "{" => TokenKind::OpenBrace,
            "}" => TokenKind::CloseBrace,

            "U8" => TokenKind::U8,
            "U16" => TokenKind::U16,
            "U32" => TokenKind::U32,
            "U64" => TokenKind::U64,

            "I8" => TokenKind::I8,
            "I16" => TokenKind::I16,
            "I32" => TokenKind::I32,
            "I64" => TokenKind::I64,

            "F32" => TokenKind::F32,
            "F64" => TokenKind::F64,

            _ => TokenKind::Identifier(cut_string),
        })
    }

    tokens
}
