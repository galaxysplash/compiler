pub enum TokenKind {
    Struct,

    OpenBracket,
    CloseBracket,

    If,
    Then,
    End,

    Eq,
    UnEq,

    Identifier(String),
}
