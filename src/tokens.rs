pub enum TokenKind {
    Struct,

    OpenBracket,
    CloseBracket,

    OpenBrace,
    CloseBrace,

    NewLine,

    If,
    Then,
    End,

    Eq,
    UnEq,

    Assignment,

    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    Identifier(String),
}
