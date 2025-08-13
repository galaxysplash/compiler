pub enum TokenKind {
    Collection,

    OpenBracket,
    CloseBracket,

    OpenBrace,
    CloseBrace,

    NewLine,

    If,
    Colon,

    Space,

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
