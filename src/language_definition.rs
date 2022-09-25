#[derive(Debug)]
pub struct Tokens {
    token_type: TokenType,
    value: String,
    position: usize,
}

#[derive(Debug)]
pub enum TokenType {
    ReservedWord,
    Expression,
    Symbol,
}

#[derive(Debug)]
pub enum ReservedWord {
    Def,
    Break,
    Read,
    Return,
    If,
    Else,
    For,
    New,
    Null,
    Print,
    Int,
    Float,
    String,
}

#[derive(Debug)]
pub enum Expression {
    Ident(String),
    IntConst(String),
    FloatConst(String),
    StrConst(String),
    Relop(String),
    Op(String),
}

#[derive(Debug)]
pub enum Symbol {
    OBrack,
    CBrack,
    OParenth,
    CParenth,
    OCurly,
    CCurly,
    Attrib,
    Semicolon,
    Comma,
}