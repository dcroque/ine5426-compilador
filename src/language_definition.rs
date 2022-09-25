#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
    position: usize,
}

#[derive(Debug)]
pub enum TokenType {
    ReservedWord(ReservedWordType),
    Expression(ExpressionType),
    Symbol(SymbolType),
}

#[derive(Debug)]
pub enum ReservedWordType {
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
pub enum ExpressionType {
    Ident(String),
    IntConst(String),
    FloatConst(String),
    StrConst(String),
    Relop(Relop),
    Op(Op),
    MulOp(MulOp),
}

#[derive(Debug)]
pub enum Relop {
    Equal,
    Diff,
    Less,
    Greater,
    EqualLess,
    EqualGreater,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum SymbolType {
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