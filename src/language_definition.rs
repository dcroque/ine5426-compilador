#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: usize,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    ReservedWord(ReservedWordType),
    Expression(ExpressionType),
    Symbol(SymbolType),
}

#[derive(Debug, Clone)]
pub struct AST {
    pub root: ASTNode,
    pub symbol_table: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub child: Vec<ASTNode>,
    pub token_name: Option<String>,
    pub value: Option<Value>,
    pub var_type: Option<VarType>,
    pub token_type: Option<TokenType>,
    pub symbol_table: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub token_name: String,
    pub value: Value,
    pub var_type: VarType,
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    KnownInt(i32),
    KnownFloat(f32),
    KnownString(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Int(i32),
    Float(i32),
    String(i32),
    NotDeclared,
}

impl TokenType {
    pub fn get_value(&self) -> String {
        use ExpressionType::*;
        use TokenType::*;

        let str_value = match self {
            ReservedWord(rw) => match rw {
                ReservedWordType::Def => "def",
                ReservedWordType::Break => "break",
                ReservedWordType::Read => "read",
                ReservedWordType::Return => "return",
                ReservedWordType::If => "if",
                ReservedWordType::Else => "else",
                ReservedWordType::For => "for",
                ReservedWordType::New => "new",
                ReservedWordType::Null => "null",
                ReservedWordType::Print => "print",
                ReservedWordType::Int => "int",
                ReservedWordType::Float => "float",
                ReservedWordType::String => "string",
            },
            Expression(ex) => match ex {
                Ident(value) => value,
                IntConst(value) => value,
                FloatConst(value) => value,
                StrConst(value) => value,
                Relop(relop) => match relop {
                    RelopType::Equal => "==",
                    RelopType::Diff => "<>",
                    RelopType::Less => "<",
                    RelopType::Greater => ">",
                    RelopType::EqualLess => "<=",
                    RelopType::EqualGreater => ">=",
                },
                Op(op) => match op {
                    OpType::Add => "+",
                    OpType::Sub => "-",
                },
                MulOp(mulop) => match mulop {
                    MulOpType::Mul => "*",
                    MulOpType::Div => "/",
                    MulOpType::Mod => "%",
                },
            },
            Symbol(s) => match s {
                SymbolType::OBrack => "[",
                SymbolType::CBrack => "]",
                SymbolType::OParenth => "(",
                SymbolType::CParenth => ")",
                SymbolType::OCurly => "{",
                SymbolType::CCurly => "}",
                SymbolType::Attrib => "=",
                SymbolType::Semicolon => ";",
                SymbolType::Comma => ",",
                SymbolType::Epsilon => "",
            },
        };
        str_value.to_string()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Ident(String),
    IntConst(String),
    FloatConst(String),
    StrConst(String),
    Relop(RelopType),
    Op(OpType),
    MulOp(MulOpType),
}

#[derive(Debug, Clone)]
pub enum RelopType {
    Equal,
    Diff,
    Less,
    Greater,
    EqualLess,
    EqualGreater,
}

#[derive(Debug, Clone)]
pub enum OpType {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum MulOpType {
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
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
    Epsilon,
}
