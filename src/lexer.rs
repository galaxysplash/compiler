use crate::tokens::TokenKind;

pub fn lex(string: &str) -> Vec<TokenKind> {
    tokenize(cut_string(string))
}

fn cut_string(uncut_string: &str) -> Vec<String> {
    let mut cut_string: Vec<String> = vec![];
    let mut string_buffer: String = String::from("");

    for c in uncut_string.chars() {
        if c == ' ' {
            cut_string.push(string_buffer.clone());
            string_buffer.clear();
            continue;
        }

        string_buffer.push(c);
    }

    cut_string.push(string_buffer.clone());

    cut_string
}

fn tokenize(cut_strings: Vec<String>) -> Vec<TokenKind> {
    let mut tokens: Vec<TokenKind> = vec![];

    for cut_string in cut_strings {
        tokens.push(match cut_string.as_str() {
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "end" => TokenKind::End,
            "struct" => TokenKind::Struct,
            _ => TokenKind::Identifier(cut_string),
        })
    }

    tokens
}
